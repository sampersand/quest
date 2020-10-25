#![allow(unused)]
use crate::{Object, Args, Literal};
use crate::types::{Text, Number, Null, Regex};
use tracing::instrument;
use parking_lot::Mutex;
use std::convert::TryFrom;
use std::sync::Arc;
use std::path::Path;
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom, BufReader, BufRead};

// is an `Arc<Mutex>` really the best way to do this
#[derive(Debug)]
pub struct File {
	file: BufReader<fs::File>
}

impl Clone for File {
	fn clone(&self) -> Self {
		Self::new(self.file.get_ref().try_clone().expect("unable to clone file"))
	}
}

impl File {
	pub fn new(file: fs::File) -> Self {
		Self { file: BufReader::new(file) }
	}

	pub fn read_all(&mut self) -> io::Result<String> {
		let mut buf = String::with_capacity(self.file.buffer().len());

		self.file.read_to_string(&mut buf)?;

		Ok(buf)
	}

	pub fn read_amnt(&mut self, amnt: usize) -> io::Result<String> {
		let mut buf = vec![0; amnt];

		self.file.read_exact(&mut buf)?;

		Ok(String::from_utf8_lossy(&buf).into_owned())
	}

	// note that if EOF is encountered before the sentinel is hit, we just return everything.
	pub fn read_until_sentinel(&mut self, sentinel: &str) -> io::Result<String> {
		if sentinel.is_empty() {
			return self.read_all();
		}

		let mut buf = Vec::with_capacity(sentinel.len());

		loop {
			let file_buf = self.file.fill_buf()?;

			if file_buf.is_empty() {
				break;
			}

			if std::str::from_utf8(&file_buf).is_err() {
				return Err(io::Error::new(io::ErrorKind::InvalidData, "stream did not contain valid UTF-8"));
			}

			if let Some((i, _)) = file_buf.windows(sentinel.len()).enumerate().find(|(_, x)| *x == sentinel.as_bytes()) {
				let end = i + sentinel.len();
				buf.extend(&file_buf[..end]);
				self.file.consume(end);
				break;
			} else {
				buf.extend(file_buf);
				let len = file_buf.len();
				self.file.consume(len);
			}

		}

		Ok(String::from_utf8(buf).expect("we checked to make sure it was valid utf8"))
	}

	// note that if EOF is encountered before the sentinel is hit, we just return everything.
	pub fn read_until_func(&mut self, func: impl Fn(&str) -> crate::Result<Option<usize>>) -> crate::Result<String> {
		let mut buf = Vec::new();

		loop {
			let file_buf = self.file.fill_buf()?;

			if file_buf.is_empty() {
				break;
			}

			let contents = std::str::from_utf8(&file_buf).map_err(|_| 
					io::Error::new(io::ErrorKind::InvalidData, "stream did not contain valid UTF-8"))?;

			if let Some(end) = func(contents)? {
				buf.extend(&file_buf[..end]);
				self.file.consume(end);
				break;
			} else {
				buf.extend(file_buf);
				let len = file_buf.len();
				self.file.consume(len);
			}

		}

		Ok(String::from_utf8(buf).expect("we checked to make sure it was valid utf8"))
	}
}

impl From<fs::File> for File {
	#[inline]
	fn from(file: fs::File) -> Self {
		Self::new(file)
	}
}

impl From<fs::File> for Object {
	#[inline]
	fn from(file: fs::File) -> Self {
		File::new(file).into()
	}
}

impl File {
	#[instrument(name="File::open", level="trace", skip(args), fields(args=?args))]
	pub fn qs_open(_: &Object, args: Args) -> crate::Result<Object> {
		let filename = args.try_arg(0)?;
		let mut openopts = OpenOptions::new();

		if let Some(arg) = args.arg(1) {
			let arg = arg.call_downcast::<Text>()?;
			let arg_str = arg.as_ref();

			if arg_str.contains('a') { openopts.append(true); openopts.create(true); }
			if arg_str.contains('t') { openopts.truncate(true); openopts.create(true); }
			if arg_str.contains('r') { openopts.read(true); }
			if arg_str.contains('w') { openopts.write(true); openopts.create(true); }
			if arg_str.contains('n') { openopts.create_new(true); }
			if arg_str.contains('N') { openopts.create(false); }
		} else {
			openopts.read(true);
		}

		let file = openopts.open(filename.call_downcast::<Text>()?.as_ref())?;

		Ok(file.into())
	}

	#[instrument(name="File::open", level="trace", skip(this, args), fields(self=?this, args=?args))]
	pub fn qs_read(this: &Object, args: Args) -> crate::Result<Object> {
		let mut this = this.try_downcast_mut::<Self>()?;

		let arg = args.arg(0);

		if let Some(amnt) = arg.and_then(Object::downcast::<Number>) {
			Ok(this.read_amnt(amnt.truncate() as usize)?.into())
		} else if let Some(end) = arg.and_then(Object::downcast::<Text>) {
			Ok(this.read_until_sentinel(end.as_ref())?.into())
		} else if let Some(rxp) = arg.and_then(Object::downcast::<Regex>) {
			// TODO: use the actual `match` quest function
			Ok(this.read_until_func(|slice| Ok(rxp.as_ref().find(slice).map(|x| x.end())))?.into())
		} else if arg.map_or(true, Object::is_a::<Null>) {
			Ok(this.read_all()?.into())
		} else if arg.as_ref().unwrap().has_attr_lit(&Literal::CALL)? {
			let arg = arg.unwrap();
			Ok(this.read_until_func(|slice| {
				let res = arg.call_attr_lit(&Literal::CALL, &[&slice.into()])?;
				if let Some(num) = res.downcast::<Number>().map(|n| n.truncate()) {
					Ok(Some(num as usize))
				} else {
					Ok(None)
				}
			})?.into())
		} else {
			Err(crate::error::TypeError::Messaged("wrong type given to read".into()).into())
		}

	}
}

impl_object_type!{
for File [(parents super::Io)]:
	"open" => function Self::qs_open,
	"read" => function Self::qs_read,
	// "write" => function Self::qs_write,
	// "close" => function Self::qs_close
}

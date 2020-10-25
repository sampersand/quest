#![allow(unused)]
use crate::{Object, Args, Literal};
use crate::types::{Text, Number, Null};
use tracing::instrument;
use parking_lot::Mutex;
use std::convert::TryFrom;
use std::sync::Arc;
use std::path::Path;
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom, BufReader, BufRead};

// is an `Arc<Mutex>` really the best way to do this
#[derive(Debug)]
pub struct File(BufReader<fs::File>);


impl Clone for File {
	fn clone(&self) -> Self {
		self.0.get_ref().try_clone().expect("unable to cloen fiel")
	}
}
impl File {
	pub fn new(file: fs::File) -> Self {
		Self(Arc::new(Mutex::new(BufReader::new(file))))
	}

	pub fn read_to_end(&mut self) -> io::Result<Text> {
		let mut buf = Text::default();

		self.0.lock().read_to_string(buf.as_mut())?;

		Ok(buf)
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
		let filename = args.try_arg(0)?.call_downcast::<Text>()?;
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

		let file = openopts.open(filename.as_ref())?;

		Ok(file.into())
	}

	#[instrument(name="File::open", level="trace", skip(this, args), fields(self=?this, args=?args))]
	pub fn qs_read(this: &Object, args: Args) -> crate::Result<Object> {
		fn read_until(mut inp: impl Read, cap: usize, func: impl Fn(&str) -> bool) -> io::Result<String> {
			let mut buf = Vec::with_capacity(cap);
		}
		let mut this = this.try_downcast_mut::<Self>()?;

		let arg = args.arg(0);

		if let Some(amnt) = arg.and_then(Object::downcast::<Number>) {
			let amnt = usize::try_from(*amnt)
				.map_err(|_| crate::error::ValueError::Messaged("bad read amount given".into()))?;
			let mut buf = vec![0; amnt];

			this.0.lock().read_exact(&mut buf)?;

			Ok(String::from_utf8_lossy(&buf).into_owned().into())
		} else if let Some(end) = arg.and_then(Object::downcast::<Text>) {
			// TODO: optimize this lol.
			let mut reader = this.0.lock();

			let mut s = String::with_capacity(end.len());
			loop {
				unimplemented!()
				// this.0.lock().read
			}
			// Ok(this.read_)
		} else if arg.map_or(true, Object::is_a::<Null>) {
			let mut buf = String::default();

			this.0.lock().read_to_string(&mut buf)?;

			Ok(buf.into())
		} else if arg.has_attr(&Literal::CALL) {
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

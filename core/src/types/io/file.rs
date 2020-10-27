#![allow(unused)]
use crate::{Object, Args, Literal};
use crate::error::ValueError;
use crate::types::{Text, Number, Null, Regex};
use tracing::instrument;
use parking_lot::Mutex;
use std::convert::TryFrom;
use std::sync::Arc;
use std::path::Path;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::io::{self, Read, Write, Seek, SeekFrom, BufReader, BufRead};

// is an `Arc<Mutex>` really the best way to do this
#[derive(Debug)]
pub struct File {
	file: Option<BufReader<fs::File>>
}

impl Clone for File {
	fn clone(&self) -> Self {
		Self { 
			file: self.file.as_ref()
				.map(|x| x.get_ref().try_clone().expect("unable to clone file"))
				.map(BufReader::new)
		}
	}
}

impl File {
	pub fn from_path<P: AsRef<Path>>(path: P, read: bool, write: bool) -> io::Result<Self> {
		OpenOptions::new().read(read).write(write).open(path)
	}

	pub fn from_fd(fd: i32, read: bool, write: bool) -> io::Result<Self> {
		OpenOptions::new().read(read).write(write).open_fd(fd)
	}

	pub fn close(&mut self) {
		self.file.take();
	}

	pub fn write(&mut self, what: &[u8]) -> io::Result<()> {
		if let Some(ref mut file) = self.file {
			file.get_mut().write(what);
		}

		Ok(())
	}

	pub fn read_all(&mut self) -> io::Result<Option<String>> {
		let mut file = 
			if let Some(ref mut file) = self.file {
				file
			} else {
				return Ok(None);
			};

		let mut buf = String::with_capacity(file.buffer().len());

		file.read_to_string(&mut buf)?;

		Ok(Some(buf))
	}

	pub fn read_amnt(&mut self, amnt: usize) -> io::Result<Option<String>> {
		let file = 
			if let Some(ref mut file) = self.file {
				file
			} else {
				return Ok(None);
			};

		let mut buf = vec![0; amnt];

		file.read_exact(&mut buf)?;

		Ok(Some(String::from_utf8_lossy(&buf).into_owned()))
	}

	// note that if EOF is encountered before the sentinel is hit, we just return everything.
	pub fn read_until_sentinel(&mut self, sentinel: &str) -> io::Result<Option<String>> {
		if sentinel.is_empty() {
			return self.read_all();
		}

		let mut file =
			if let Some(ref mut file) = self.file {
				file
			} else {
				return Ok(None)
			};

		let mut buf = Vec::with_capacity(sentinel.len());

		loop {
			let file_buf = file.fill_buf()?;

			if file_buf.is_empty() {
				break;
			}

			if std::str::from_utf8(&file_buf).is_err() {
				return Err(io::Error::new(io::ErrorKind::InvalidData, "stream did not contain valid UTF-8"));
			}

			if let Some((i, _)) = file_buf.windows(sentinel.len()).enumerate().find(|(_, x)| *x == sentinel.as_bytes()) {
				let end = i + sentinel.len();
				buf.extend(&file_buf[..end]);
				file.consume(end);
				break;
			} else {
				buf.extend(file_buf);
				let len = file_buf.len();
				file.consume(len);
			}

		}

		Ok(Some(String::from_utf8(buf).expect("we checked to make sure it was valid utf8")))
	}

	// note that if EOF is encountered before the sentinel is hit, we just return everything.
	pub fn read_until_func<F>(&mut self, func: F) -> crate::Result<Option<String>>
	where
		F: Fn(&str) -> crate::Result<Option<usize>>
	{
		let mut file =
			if let Some(ref mut file) = self.file {
				file
			} else {
				return Ok(None);
			};

		let mut buf = Vec::new();

		loop {
			let file_buf = file.fill_buf()?;

			if file_buf.is_empty() {
				break;
			}

			let contents = std::str::from_utf8(&file_buf).map_err(|_| 
					io::Error::new(io::ErrorKind::InvalidData, "stream did not contain valid UTF-8"))?;

			if let Some(end) = func(contents)? {
				buf.extend(&file_buf[..end]);
				file.consume(end);
				break;
			} else {
				buf.extend(file_buf);
				let len = file_buf.len();
				file.consume(len);
			}

		}

		Ok(Some(String::from_utf8(buf).expect("we checked to make sure it was valid utf8")))
	}
}

impl From<fs::File> for File {
	#[inline]
	fn from(file: fs::File) -> Self {
		Self { file: Some(BufReader::new(file)) }
	}
}

impl From<fs::File> for Object {
	#[inline]
	fn from(file: fs::File) -> Self {
		File::from(file).into()
	}
}

/// A class that's used to specify options when opening files.
///
/// For the time being, this is a wrapper around [`std::fs::OpenOptions`]	
#[derive(Debug, Clone)]
pub struct OpenOptions(fs::OpenOptions);

impl Default for OpenOptions {
	/// By default, files are read.
	fn default() -> Self {
		let mut opts = OpenOptions::new();
		opts.read(true);
		opts
	}
}

impl From<fs::OpenOptions> for OpenOptions {
	#[inline]
	fn from(open_opts: fs::OpenOptions) -> Self {
		Self(open_opts)
	}
}


macro_rules! forward_fn {
	($(#[$meta:meta])* $fn:ident) => {
		#[inline]
		$(#[$meta])*
		pub fn $fn(&mut self, $fn: bool) -> &mut Self {
			self.0.$fn($fn);
			self
		}
	};
}

impl OpenOptions {
	#[inline]
	pub fn new() -> Self {
		Self(fs::OpenOptions::new())
	}

	#[inline]
	pub fn read(&mut self, read: bool) -> &mut Self {
		self.0.read(read);
		self
	}

	#[inline]
	pub fn write(&mut self, write: bool) -> &mut Self {
		self.0.write(write);
		self
	}

	#[inline]
	pub fn append(&mut self, append: bool) -> &mut Self {
		self.0.append(append).write(true);
		self
	}

	#[inline]
	pub fn truncate(&mut self, truncate: bool) -> &mut Self {
		self.0.truncate(truncate).write(true);
		self
	}
	

	#[inline]
	pub fn create(&mut self, create: bool) -> &mut Self {
		self.0.create(create).write(true);
		self
	}
	

	#[inline]
	pub fn create_new(&mut self, create_new: bool) -> &mut Self {
		self.0.create_new(create_new).write(true);
		self
	}

	#[inline]
	pub fn open<P: AsRef<Path>>(&self, path: P) -> io::Result<File> {
		self.0.open(path).map(File::from)
	}

	pub fn open_fd(&self, fd: i32) -> io::Result<File> {
		self.open(format!("/dev/fd/{}", fd))
	}
}

#[derive(Debug, Clone)]
pub struct InvalidOptionChar(pub char);

impl Display for InvalidOptionChar {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "invalid option char given: {}", self.0)
	}
}

impl std::error::Error for InvalidOptionChar {}

impl TryFrom<&str> for OpenOptions {
	type Error = InvalidOptionChar;

	/// Attempts to create an [`OpenOptions`] from the given options string.
	///
	/// Characters in the string are applied in order, with capital letters disabling values.:
	/// - `r` Sets [`read`](OpenOptions::read).
	/// - `w` Sets [`write`](OpenOptions::write).
	/// - `a` Sets [`append`](OpenOptions::append).
	/// - `t` Sets [`truncate`](OpenOptions::truncate).
	/// - `n` Sets [`create_new`](OpenOptions::create_new).
	/// Any other character will yield an [`InvalidOptionChar`].
	///
	/// If an empty string is supplied, [the default](OpenOptions::default) is returned.
	fn try_from(opts_str: &str) -> Result<Self, Self::Error> {
		let mut opts = Self::new();

		for opt in opts_str.chars() {
			match opt {
				'a' | 'A' => opts.append(opt == 'a'),
				'w' | 'W' => opts.write(opt == 'w'),
				't' | 'T' => opts.truncate(opt == 't'),
				'r' | 'R' => opts.read(opt == 'r'),
				'n' | 'N' => opts.create_new(opt == 'n'),
				invalid => return Err(InvalidOptionChar(invalid))
			};
		}

		Ok(opts)
	}
}

impl File {
	#[instrument(name="File::call", level="trace", skip(args), fields(args=?args))]
	pub fn qs_call(_: &Object, args: Args) -> crate::Result<Object> {
		let filename = args.try_arg(0)?;
		let openopts =
			if let Some(arg) = args.arg(1) {
				OpenOptions::try_from(arg.call_downcast::<Text>()?.as_ref())
					.map_err(|err| ValueError::Messaged(err.to_string()))?
			} else {
				OpenOptions::default()
			};

		let file = 
			if let Some(fd) = filename.downcast::<Number>() {
				openopts.open_fd(i32::try_from(*fd)?)?
			} else {
				openopts.open(filename.call_downcast::<Text>()?.as_ref())?
			};

		Ok(file.into())
	}

	#[instrument(name="File::open", level="trace", skip(this, args), fields(self=?this, args=?args))]
	pub fn qs_read(this: &Object, args: Args) -> crate::Result<Object> {
		let mut this = this.try_downcast_mut::<Self>()?;

		let arg = args.arg(0);

		let read =
			if let Some(amnt) = arg.and_then(Object::downcast::<Number>) {
				this.read_amnt(amnt.truncate() as usize)?
			} else if let Some(end) = arg.and_then(Object::downcast::<Text>) {
				this.read_until_sentinel(end.as_ref())?
			} else if let Some(rxp) = arg.and_then(Object::downcast::<Regex>) {
				// TODO: use the actual `match` quest function
				this.read_until_func(|slice| Ok(rxp.as_ref().find(slice).map(|x| x.end())))?
			} else if arg.map_or(true, Object::is_a::<Null>) {
				this.read_all()?
			} else if arg.as_ref().unwrap().has_attr_lit(&Literal::CALL)? {
				let arg = arg.unwrap();
				this.read_until_func(|slice| {
					let res = arg.call_attr_lit(&Literal::CALL, &[&slice.into()])?;
					if let Some(num) = res.downcast::<Number>().map(|n| n.truncate()) {
						Ok(Some(num as usize))
					} else {
						Ok(None)
					}
				})?
			} else {
				return Err(crate::error::TypeError::Messaged("wrong type given to read".into()).into());
			};

		Ok(if read.as_ref().map_or(false, |s| s.is_empty()) {
			Object::default()
		} else {
			read.map(Object::from).unwrap_or_default()
		})
	}

	#[instrument(name="File::write", level="trace", skip(this, args), fields(self=?this, args=?args))]
	pub fn qs_write(this: &Object, args: Args) -> crate::Result<Object> {
		let to_write = args.try_arg(0)?.call_downcast::<Text>()?;

		this.try_downcast_mut::<Self>()?.file.get_mut().write_all(to_write.as_ref().as_ref())?;

		Ok(this.clone())
	}

	#[instrument(name="File::close", level="trace", skip(this), fields(self=?this))]
	pub fn qs_close(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_mut::<Self>()?.close();

		Ok(this.clone())
	}
}

impl_object_type!{
for File [(parents super::Io)]:
	"()" => method Self::qs_call,
	"@text" => method |_, _| panic!(),
	"read" => method Self::qs_read,
	"write" => method Self::qs_write,
	"close" => method Self::qs_close,
	// "close" => method Self::qs_close
}

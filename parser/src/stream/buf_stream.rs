use crate::Result;
use crate::stream::{Context, Contexted, Stream};
use std::io::{self, Cursor, Seek, SeekFrom, Stdin, BufReader, BufRead};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::convert::TryFrom;

/// A [`Stream`](trait.Stream.html) based around a [`BufRead`](#)-able typpe
#[derive(Debug, PartialEq, Eq, Clone, Default, Hash)]
pub struct BufStream<B: BufRead> {
	/// The data to read from.
	data: B,
	/// The current context we're in.
	context: Context
}

impl<B: BufRead> Seek for BufStream<B> {
	/// Seek to the given position **on the current line**. 
	///
	/// This means that once a `\n` is encountered, we're not able to seek back to the previous line.
	///
	/// # Errors
	///
	/// No errors are returned from this function, as no reads from the underlying buffer are
	/// performed whilst seeking.
	///
	/// # Panics
	///
	/// This function panics if the position to seek to is either before `0`, or after the line's
	/// ending.
	fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
		let pos =
			match pos {
				SeekFrom::Start(n) => n as i64,
				SeekFrom::Current(n) => self.context.column as i64 + n,
				SeekFrom::End(n) => self.current_line_len() as i64 + n,
			};

		if pos < 0 || pos > self.current_line_len() as _ {
			unreachable!(
				"seeking before or beyond current line. pos={}, lineno={}, column={}, line={}",
				pos, self.context.lineno, self.context.column, self.context.line)
		}

		self.context.column = pos as usize;

		Ok(self.context.column as u64)
	}
}

impl<S: BufRead> Iterator for BufStream<S> {
	type Item = Result<char>;
	/// Get the next character in the stream.
	///
	/// If we're currently at the end of a line (or we haven't started reading), this will try to
	/// read an entire line from the data it's given. At no other time will it read a line.
	fn next(&mut self) -> Option<Result<char>> {
		if let Err(err) = self.read_next_line_if_applicable() {
			Some(Err(err))
		} else if let Some(chr) = self.context.line.chars().nth(self.context.column) {
			self.context.column += 1;
			Some(Ok(chr))
		} else {
			None
		}
	}
}

impl<B: BufRead> Contexted for BufStream<B> {
	fn context(&self) -> &Context {
		&self.context
	}
}

impl<B: BufRead> Stream for BufStream<B> {
	fn starts_with(&mut self, s: &str) -> Result<bool> {
		self.line().map(|line| line.starts_with(s))
	}
}

impl<B: BufRead> BufStream<B> {
	/// Create a new [`BufStream`](#) for the given data, with an optional file being passed to
	/// [`Context`](#)
	pub fn new(data: B, file: Option<PathBuf>) -> Self {
		BufStream { data, context: Context::new(file) }
	}

	/// Get the current line
	fn line(&mut self) -> Result<&str> {
		self.read_next_line_if_applicable()?;
		let mut iter = self.context.line.chars();

		if self.context.column != 0 {
			// offset the the iter if we aren't at the end yet.
			iter.by_ref().nth(self.context.column - 1);
		}
		Ok(iter.as_str())
	}

	fn current_line_len(&self) -> usize {
		self.context.line.chars().count()
	}

	/// If our [`context`](#structfield.context) is at the end of a line, read another line in.
	fn read_next_line_if_applicable(&mut self) -> Result<()> {
		use std::mem::{take, swap};

		// if we're at the end of a line, try read a new line and update the lineno and column
		if self.current_line_len() <= self.context.column {
			// keep track of the old line in case we aren't able to read a new one (for err msgs)
			let mut old_line = take(&mut self.context.line);

			match self.data.read_line(&mut self.context.line) {
				// if there's nothing left to read, just keep the old line.
				Ok(0) => swap(&mut old_line, &mut self.context.line),
				Ok(_) => {
					self.context.lineno += 1;
					self.context.column = 0;
				},
				Err(err) => {
					swap(&mut old_line, &mut self.context.line);
					return Err(parse_error!(self, CantReadStream(err)));
				}
			}
		}

		Ok(())
	}
}

impl BufStream<BufReader<Stdin>> {
	/// Create a new [`BufStream`](#) from stdin.
	pub fn stdin() -> Self {
		BufStream::new(BufReader::new(io::stdin()), Some("-".into()))
	}
}

impl<T: AsRef<[u8]>> From<T> for BufStream<Cursor<T>> {
	/// Create a new [`BufStream`](#) from the given input.
	///
	/// This assumes that `data` comes from a non-file source. If a `file` is desired,
	/// [`BufStream::new`](#) should be used.
	fn from(data: T) -> Self {
		BufStream::new(Cursor::new(data), None)
	}
}

impl TryFrom<&'_ Path> for BufStream<BufReader<File>> {
	type Error = io::Error;

	/// Try to construct a new [`BufStream`](#) from the given path, raising an error if we're not
	/// able to access the file for some reason.
	fn try_from(path: &Path) -> io::Result<Self> {
		Ok(BufStream::new(BufReader::new(File::open(path)?), Some(path.into())))
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	macro_rules! assert_next_eq {
		($buf:expr, $chr:literal) => {
			assert_next_eq!($buf, Some($chr));
		};

		($buf:expr, $chr:expr) => {
			assert_eq!($buf.next().transpose().expect("buf.next failed"), $chr);
		};
	}

	macro_rules! assert_start_with {
		($buf:expr, $what:expr) => {{
			let buf = &mut $buf;
			let what = &$what;
			assert!(buf.starts_with(what).expect("buf.starts_with failed"),
				"\nbuf: {:?}\n!=   {:?}", buf.line().unwrap(), what);
		}};
	}

	#[test]
	fn new() {
		use std::io::Cursor;
		let buf = BufStream::new(Cursor::new("_"), Some("/plato/the_symposium.txt".into()));
		assert_eq!(buf.context, Context::new(Some("/plato/the_symposium.txt".into())));
	}

	#[test]
	fn from_str() {
		let _: BufStream<_> = BufStream::from("plato's dialogues");
	} 

	#[test]
	fn from_path() {
		let tmpfile = tempfile::NamedTempFile::new().expect("couldn't make tempfile");
		let _: BufStream<_> = BufStream::try_from(tmpfile.path()).expect("couldn't make bufstream");

		BufStream::try_from(Path::new("/plato/teh_republic.txt"))
			.expect_err("invalid file still passed");
	}

	#[test]
	fn from_stdin() {
		let _: BufStream<_> = BufStream::stdin();
	}

	#[test]
	#[ignore]
	fn read_next_line_if_applicable() {
		/* should I even write this? */
		unimplemented!()
	}

	#[test]
	fn iterator() {
		let mut buf = BufStream::new(Cursor::new("this\nis\nsparta"), None);
		macro_rules! assert_next_start_with {
			($chr:literal $what:literal) => {
				assert_next_eq!(buf, $chr);
				assert_start_with!(buf, $what);
			};
		}

		buf.read_next_line_if_applicable().expect("buf.read_next_line_if_applicable failed");

		assert_start_with!(buf, "this");
		assert_next_start_with!('t' "his");
		assert_next_start_with!('h' "is");
		assert_next_start_with!('i' "s");
		assert_next_eq!(buf, 's');

		assert_next_start_with!('\n' "is");
		assert_next_start_with!('i' "s");
		assert_next_eq!(buf, 's');

		assert_next_start_with!('\n' "sparta");
		assert_next_start_with!('s' "parta");
		assert_next_start_with!('p' "arta");
		assert_next_start_with!('a' "rta");
		assert_next_start_with!('r' "ta");
		assert_next_start_with!('t' "a");
		assert_next_eq!(buf, 'a');
		assert_next_eq!(buf, None);
	}

	#[test]
	fn contexted() -> Result<()> {
		let mut buf = BufStream::from("the\n\t\n\napology");
		assert_eq!(*buf.context(), Context::default());

		macro_rules! assert_next_context_eq {
			($chr:literal $lineno:literal $column:literal $line:literal) => {
				assert_next_eq!(buf, $chr);
				assert_eq!(
					*buf.context(),
					Context { file: None, lineno: $lineno, column: $column, line: $line.into() }
				);
			};
		}

		assert_next_context_eq!('t' 1 1 "the\n");
		assert_next_context_eq!('h' 1 2 "the\n");
		assert_next_context_eq!('e' 1 3 "the\n");
		assert_next_context_eq!('\n' 1 4 "the\n");
		assert_next_context_eq!('\t' 2 1 "\t\n");
		assert_next_context_eq!('\n' 2 2 "\t\n");
		assert_next_context_eq!('\n' 3 1 "\n");
		assert_next_context_eq!('a' 4 1 "apology");
		assert_next_context_eq!('p' 4 2 "apology");
		assert_next_context_eq!('o' 4 3 "apology");
		assert_next_context_eq!('l' 4 4 "apology");
		assert_next_context_eq!('o' 4 5 "apology");
		assert_next_context_eq!('g' 4 6 "apology");
		assert_next_context_eq!('y' 4 7 "apology");

		assert_next_eq!(buf, None);
		assert_eq!(
			*buf.context(),
			Context { file: None, lineno: 4, column: 7, line: "apology".to_string() }
		);

		Ok(())
	}

	#[test]
	fn seek() {
		use std::io::{Seek, SeekFrom};
		let mut buf = BufStream::from("timaeus\nand\ncritias");

		macro_rules! seek {
			($from:ident($where:expr)) => { buf.seek(SeekFrom::$from($where)).expect("can't seek") };
		}

		// normal start
		assert_start_with!(buf, "timaeus\n");
		assert_next_eq!(buf, 't');
		assert_start_with!(buf, "imaeus\n");

		// seek forward a bit
		seek!(Current(3));
		assert_start_with!(buf, "eus\n");
		assert_next_eq!(buf, 'e');
		assert_start_with!(buf, "us\n");

		// reverse what we just looked at
		seek!(Current(-1));
		assert_start_with!(buf, "eus\n");
		assert_next_eq!(buf, 'e');
		assert_start_with!(buf, "us\n");

		// uh oh, back up more
		seek!(Current(-2));
		assert_start_with!(buf, "aeus\n");
		assert_next_eq!(buf, 'a');
		assert_start_with!(buf, "eus\n");

		// done with this line
		seek!(End(0));

		// try next line, make sure `Start` works
		assert_start_with!(buf, "and\n");
		assert_next_eq!(buf, 'a');
		assert_start_with!(buf, "nd\n");
		seek!(Start(0));

		assert_start_with!(buf, "and\n");
		assert_next_eq!(buf, 'a');
		assert_start_with!(buf, "nd\n");

		seek!(End(0));
		assert_start_with!(buf, "critias");
		assert_next_eq!(buf, 'c');
		assert_start_with!(buf, "ritias");
	}

	#[test]
	#[should_panic(expected="seeking before or beyond current line")]
	fn before_first_line() {
		use std::io::{Seek, SeekFrom};
		BufStream::from("the last\ndays of\nsocrates").seek(SeekFrom::Current(-1)).unwrap();
	}

	#[test]
	#[should_panic(expected="seeking before or beyond current lin")]
	fn after_first_line() {
		use std::io::{Seek, SeekFrom};
		BufStream::from("the last\ndays of\nsocrates").seek(SeekFrom::End(1)).unwrap();
	}

	#[test]
	#[should_panic(expected="seeking before or beyond current line. pos=-1, lineno=2, column=0")]
	fn before_second_line() {
		let mut buf = BufStream::from("the last\ndays of\nsocrates");
		use std::io::{Seek, SeekFrom};

		assert_start_with!(buf, "the last\n");
		buf.seek(SeekFrom::End(-1)).unwrap();

		assert_start_with!(buf, "\n");
		assert_next_eq!(buf, '\n');
		assert_start_with!(buf, "days of\n");

		buf.seek(SeekFrom::Current(-1)).unwrap();
	}
}

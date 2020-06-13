use crate::{Token, Result};
use crate::stream::{Context, Stream};

use std::io::{self, Cursor, Seek, SeekFrom, BufReader, BufRead};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct BufStream<B: BufRead> {
	data: B,
	context: Context
}


impl<B: BufRead> Seek for BufStream<B> {
	fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
		self.context.column =
			match pos {
				SeekFrom::Start(n) if n > self.context.line.len() as u64 => {
					unreachable!("Code shouldn't be seeking past the line they're on?");
				},
				SeekFrom::Start(n) => n as usize,
				SeekFrom::Current(n) => (self.context.column as i64 + n) as usize,
				SeekFrom::End(n) => (self.context.line.len() as i64 + n) as usize,
			};
		Ok(self.context.column as u64)
	}
}

impl<B: BufRead> Stream for BufStream<B> {
	fn context(&self) -> &Context {
		&self.context
	}

	fn next_char(&mut self) -> Result<Option<char>> {
		let chr_opt = self.peek_char()?;

		if chr_opt.is_some() {
			self.context.column += 1;
		}

		Ok(chr_opt)
	}

	fn peek_char(&mut self) -> Result<Option<char>> {
		self.read_next_line_if_applicable()?;
		Ok(self.context.line.chars().nth(self.context.column))
	}
}

impl<B: BufRead> BufStream<B> {
	fn read_next_line_if_applicable(&mut self) -> Result<()> {
		use std::mem::{take, swap};

		if self.context.line.len() <= self.context.column {
			// keep track of the old line in case we aren't able to read a new one (for err msgs)
			let mut old_line = take(&mut self.context.line);

			match self.data.read_line(&mut self.context.line) {
				// if we reached end-of-line, swap line back (although do we want to do this?)
				// Ok(0) => swap(&mut old_line, &mut self.context.line),
				Ok(_) => {
					self.context.lineno += 1;
					self.context.column = 0;
				}
				Err(err) => {
					// if we got an error when reading the line, restore the old one.
					swap(&mut old_line, &mut self.context.line);
					return Err(parse_error!(self, CantReadStream(err)));
				}
			}
		}

		Ok(())
	}
}
// Creation Impls
impl<B: BufRead> BufStream<B> {
	pub fn new(data: B, file: Option<PathBuf>) -> Self {
		BufStream {
			data,
			context: Context::new(file)
		}
	}

	pub fn context(&self) -> &Context {
		&self.context
	}

	pub fn peek_str(&mut self) -> Result<&str> {
		self.load_line()?;

		let mut iter = self.context.line.chars();
		if self.context.column != 0 {
			iter.by_ref().nth(self.context.column - 1);
		}
		Ok(iter.as_str())
	}

	pub fn shift_str(&mut self, s: &str) -> Result<()> {
		assert!(self.peek_str()?.starts_with(s), "'{}' doesn't start with '{}'", self.peek_str()?, s);

		self.context.column += s.len();
		Ok(())
	}

	pub fn unshift_char(&mut self, chr: char) {
		assert_ne!(self.context.column, 0, "todo: unseek characters at the start of the line");
		self.context.column -= 1;
		assert_eq!(self.context.line.chars().nth(self.context.column), Some(chr));
	}

	fn load_line(&mut self) -> Result<()> {
		// if the column's too far...
		if self.context.line.len() <= self.context.column {
			// keep track of the old line in case we aren't able to read a new one (for err msgs)
			let mut old_line = std::mem::take(&mut self.context.line);
			match self.data.read_line(&mut self.context.line) {
				Ok(0) => std::mem::swap(&mut old_line, &mut self.context.line),
				Ok(_) => {
					self.context.lineno += 1;
					self.context.column = 0;
				}
				Err(err) => {
					std::mem::swap(&mut old_line, &mut self.context.line);
					return Err(
						crate::Error::new(
							self.context.clone(),
							crate::ErrorType::CantReadStream(err)
						)
					)?
				}
			}
		}

		Ok(())
	}
}

impl<'a> BufStream<Cursor<&'a str>> {
	pub fn new_from_str(data: &'a str) -> Self {
		BufStream::new(Cursor::new(data), None)
	}
}

impl<'a> From<&'a str> for BufStream<Cursor<&'a str>> {
	fn from(data: &'a str) -> Self {
		BufStream::new_from_str(data)
	}
}

impl BufStream<BufReader<File>> {
	pub fn new_from_path<P: Into<PathBuf>>(path: P) -> io::Result<Self> {
		let path = path.into();
		Ok(BufStream::new(BufReader::new(File::open(&path)?), Some(path)))
	}
}

impl TryFrom<&'_ Path> for BufStream<BufReader<File>> {
	type Error = io::Error;

	fn try_from(path: &Path) -> io::Result<Self> {
		BufStream::new_from_path(path)
	}

}


impl<S: BufRead> Iterator for BufStream<S> {
	type Item = Result<Token>;
	fn next(&mut self) -> Option<Self::Item> {
		Token::try_parse_old(self).transpose()
	}
}

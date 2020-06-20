use crate::{Result, Error, ErrorType};
use crate::stream::{Context, Contexted, Stream};

use std::io::{self, Cursor, Seek, SeekFrom, Stdin, BufReader, BufRead};
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

impl<S: BufRead> Iterator for BufStream<S> {
	type Item = Result<char>;
	fn next(&mut self) -> Option<Result<char>> {
		let chr_res = self.peek()?;

		if chr_res.is_ok() {
			self.context.column += 1;
		}

		Some(chr_res)
	}
}

impl<B: BufRead> Contexted for BufStream<B> {
	fn context(&self) -> &Context {
		&self.context
	}
}

impl<B: BufRead> Stream for BufStream<B> {
	fn peek(&mut self) -> Option<Result<char>> {
		if let Err(err) = self.read_next_line_if_applicable() {
			Some(Err(err))
		} else {
			self.context.line.chars().nth(self.context.column).map(Ok)
		}
	}

	fn starts_with(&mut self, s: &str) -> Result<bool> {
		self.read_next_line_if_applicable()?;
		Ok(self.as_ref().starts_with(s))
	}
}

impl<B: BufRead> AsRef<str> for BufStream<B> {
	fn as_ref(&self) -> &str {
		let mut iter = self.context.line.chars();
		if self.context.column != 0 {
			iter.by_ref().nth(self.context.column - 1);
		}
		iter.as_str()
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
					return Err(Error::new(
						self.context().clone(),
						ErrorType::CantReadStream(err)
					));
				}
			}
		}

		Ok(())
	}
}

impl<B: BufRead> BufStream<B> {
	pub fn new(data: B, file: Option<PathBuf>) -> Self {
		BufStream {
			data,
			context: Context::new(file)
		}
	}
}

impl BufStream<BufReader<Stdin>> {
	pub fn stdin() -> Self {
		BufStream::new(BufReader::new(io::stdin()), Some("-".into()))
	}
}

impl From<String> for BufStream<Cursor<String>> {
	fn from(data: String) -> Self {
		BufStream::new(Cursor::new(data), None)
	}
}

impl TryFrom<&'_ Path> for BufStream<BufReader<File>> {
	type Error = io::Error;

	fn try_from(path: &Path) -> io::Result<Self> {
		let path = path.into();
		Ok(BufStream::new(BufReader::new(File::open(&path)?), Some(path)))
	}
}


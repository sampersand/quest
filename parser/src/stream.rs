use crate::token::{self, Token, Parsable};
use std::io::{self, Cursor, BufReader, BufRead};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct Context {
	pub file: Option<PathBuf>,
	pub lineno: usize,
	pub column: usize,
	pub line: String,
}

#[derive(Debug)]
pub struct Stream<B: BufRead> {
	data: B,
	context: Context
}

impl Context {
	pub fn line(&self) -> &str {
		self.line.as_str()
	}
}

// Creation Impls
impl<B: BufRead> Stream<B> {
	pub fn new(data: B, file: Option<PathBuf>) -> Self {
		Stream {
			data,
			context: Context { file, lineno: 0, column: 0, line: String::new() }
		}
	}

	pub fn context(&self) -> &Context {
		&self.context
	}

	pub fn peek_char(&mut self) -> token::Result<Option<char>> {
		self.load_line()?;

		Ok(self.context.line.chars().nth(self.context.column))
	}

	pub fn unshift_char(&mut self, chr: char) {
		assert_ne!(self.context.column, 0, "todo: unseek characters at the start of the line");
		self.context.column -= 1;
		assert_eq!(self.context.line.chars().nth(self.context.column), Some(chr));
	}

	pub fn next_char(&mut self) -> token::Result<Option<char>> {
		self.load_line()?;

		let chr_opt = self.context.line.chars().nth(self.context.column);

		if chr_opt.is_some() {
			self.context.column += 1;
		}

		Ok(chr_opt)
	}


	fn load_line(&mut self) -> token::Result<()> {
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
						token::Error::new(
							self.context.clone(),
							token::ErrorType::CantReadStream(err)
						)
					)?
				}
			}
		}

		Ok(())
	}
}

impl<'a> Stream<Cursor<&'a str>> {
	pub fn new_from_str(data: &'a str) -> Self {
		Stream::new(Cursor::new(data), None)
	}
}

impl<'a> From<&'a str> for Stream<Cursor<&'a str>> {
	fn from(data: &'a str) -> Self {
		Stream::new_from_str(data)
	}
}

impl Stream<BufReader<File>> {
	pub fn new_from_path<P: Into<PathBuf>>(path: P) -> io::Result<Self> {
		let path = path.into();
		Ok(Stream::new(BufReader::new(File::open(&path)?), Some(path)))
	}
}

impl TryFrom<&'_ Path> for Stream<BufReader<File>> {
	type Error = io::Error;

	fn try_from(path: &Path) -> io::Result<Self> {
		Stream::new_from_path(path)
	}

}


impl<S: BufRead> Iterator for Stream<S> {
	type Item = token::Result<Token>;
	fn next(&mut self) -> Option<Self::Item> {
		Token::try_parse(self).transpose()
	}
}

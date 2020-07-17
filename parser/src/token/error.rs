use super::{number, regex, text};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
	Number(number::Error),
	Regex(regex::Error),
	Text(text::Error),
	UnterminatedBlockComment,
	UnknownTokenStart(char),
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::Number(err) => Display::fmt(&err, f),
			Self::Regex(err) => Display::fmt(&err, f),
			Self::Text(err) => Display::fmt(&err, f),
			Self::UnterminatedBlockComment => write!(f, "unterminated block comment"),
			Self::UnknownTokenStart(chr) => write!(f, "unknown token start char '{}'", chr),
		}
	}
}

impl From<number::Error> for Error {
	#[inline]
	fn from(err: number::Error) -> Self {
		Self::Number(err)
	}
}

impl From<regex::Error> for Error {
	#[inline]
	fn from(err: regex::Error) -> Self {
		Self::Regex(err)
	}
}

impl From<text::Error> for Error {
	#[inline]
	fn from(err: text::Error) -> Self {
		Self::Text(err)
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Self::Number(ref err) => Some(err),
			Self::Regex(ref err) => Some(err),
			Self::Text(ref err) => Some(err),
			_ => None
		}
	}
}
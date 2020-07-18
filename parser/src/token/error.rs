use super::{number, regex, text, stackpos};
use std::fmt::{self, Display, Formatter};

/// Any errors that could occur whilst parsing a token.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
	/// Can't parse a [`Number`](number::Number).
	Number(number::Error),

	/// Can't parse a [`Regex`](regex::Regex).
	Regex(regex::Error),

	/// Can't parse a [`Text`](text::Text).
	Text(text::Error),

	/// Can't parse a [`Stackpos`](stackpos::Stackpos).
	Stackpos(stackpos::Error),

	/// A block comment was started but not terminated
	UnterminatedBlockComment,

	/// An unknown token was encountered.
	UnknownTokenStart(char),
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::Number(err) => Display::fmt(&err, f),
			Self::Regex(err) => Display::fmt(&err, f),
			Self::Text(err) => Display::fmt(&err, f),
			Self::Stackpos(err) => Display::fmt(&err, f),
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

impl From<stackpos::Error> for Error {
	#[inline]
	fn from(err: stackpos::Error) -> Self {
		Self::Stackpos(err)
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Self::Number(ref err) => Some(err),
			Self::Regex(ref err) => Some(err),
			Self::Text(ref err) => Some(err),
			Self::Stackpos(ref err) => Some(err),
			_ => None
		}
	}
}
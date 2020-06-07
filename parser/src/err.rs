use std::{char::CharTryFromError, io};
use crate::{expression, token};

#[derive(Debug)]
pub enum Error {
	BadChar(CharTryFromError),
	IoError(io::Error),
	UnknownTokenStart(char),
	UnterminatedQuote,
	UnknownEscape(char),
	Message(String),
	ExpressionError(expression::Error),
	BadClosingParen(token::ParenType, token::ParenType),
	DanglingClosingParen(token::ParenType),
	NoClosingParen,
}

impl From<expression::Error> for Error {
	fn from(err: expression::Error) -> Self {
		Error::ExpressionError(err)
	}
}

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Self {
		Error::IoError(err)
	}
}

impl From<CharTryFromError> for Error {
	fn from(err: CharTryFromError) -> Self {
		Error::BadChar(err)
	}
}

pub type Result<T> = std::result::Result<T, Error>;
use std::{char::CharTryFromError, io};

#[derive(Debug)]
pub enum Error {
	BadChar(CharTryFromError),
	IoError(io::Error),
	UnknownTokenStart(char),
	UnterminatedQuote,
	UnknownEscape(char),
	Message(String)
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
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
	Quest(quest_core::Error),
	Parser(quest_parser::Error),
	Io(std::io::Error)
}

impl From<std::io::Error> for Error {
	fn from(error: std::io::Error) -> Self {
		Error::Io(error)
	}
}

impl From<quest_core::Error> for Error {
	fn from(error: quest_core::Error) -> Self {
		Error::Quest(error)
	}
}

impl From<quest_parser::Error> for Error {
	fn from(error: quest_parser::Error) -> Self {
		Error::Parser(error)
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Error::Quest(err) => Display::fmt(&err, f),
			Error::Parser(err) => Display::fmt(&err, f),
			Error::Io(err) => Display::fmt(&err, f),
		}
	}
}
impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Error::Quest(err) => Some(err),
			Error::Parser(err) => Some(err),
			Error::Io(err) => Some(err)
		}
	}
}

pub type Result<T> = std::result::Result<T, Error>;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
	Quest(quest_core::Error),
	Parser(quest_parser::Error),
	Io(std::io::Error)
}

impl From<std::io::Error> for Error {
	fn from(error: std::io::Error) -> Self {
		Self::Io(error)
	}
}

impl From<quest_core::Error> for Error {
	fn from(error: quest_core::Error) -> Self {
		Self::Quest(error)
	}
}

impl From<quest_parser::Error> for Error {
	fn from(error: quest_parser::Error) -> Self {
		Self::Parser(error)
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::Quest(err) => Display::fmt(&err, f),
			Self::Parser(err) => Display::fmt(&err, f),
			Self::Io(err) => Display::fmt(&err, f),
		}
	}
}
impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Self::Quest(err) => Some(err),
			Self::Parser(err) => Some(err),
			Self::Io(err) => Some(err)
		}
	}
}

pub type Result<T> = std::result::Result<T, Error>;
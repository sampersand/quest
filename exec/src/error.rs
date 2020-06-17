#[derive(Debug)]
pub enum Error {
	Quest(quest::Error),
	Parser(quest_parser::Error),
	Io(std::io::Error)
}

impl From<std::io::Error> for Error {
	fn from(error: std::io::Error) -> Self {
		Error::Io(error)
	}
}

impl From<quest::Error> for Error {
	fn from(error: quest::Error) -> Self {
		Error::Quest(error)
	}
}

impl From<quest_parser::Error> for Error {
	fn from(error: quest_parser::Error) -> Self {
		Error::Parser(error)
	}
}

pub type Result<T> = std::result::Result<T, Error>;
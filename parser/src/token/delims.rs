use crate::{Result, Stream};
use crate::token::Parsable;
use std::fmt::{self, Display, Formatter};
use std::io::BufRead;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) enum Delims {
	Comma,
	Endline
}

impl Display for Delims {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Delims::Comma => write!(f, ","),
			Delims::Endline => write!(f, ";"),
		}
	}
}

impl Parsable for Delims {
	type Item = Self;
	fn try_parse<S: BufRead>(stream: &mut Stream<S>) -> Result<Option<Self>> {
		match stream.next_char()? {
			Some('(') => Ok(Some(Delims::Left(ParenType::Round))),
			Some(')') => Ok(Some(Delims::Right(ParenType::Round))),
			Some('[') => Ok(Some(Delims::Left(ParenType::Square))),
			Some(']') => Ok(Some(Delims::Right(ParenType::Square))),
			Some('{') => Ok(Some(Delims::Left(ParenType::Curly))),
			Some('}') => Ok(Some(Delims::Right(ParenType::Curly))),
			Some(chr) => { stream.unshift_char(chr); Ok(None) },
			None => Ok(None)
		}
	}
}

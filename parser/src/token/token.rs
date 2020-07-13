use crate::Result;
use crate::stream::Stream;

use super::parenthesis::Parenthesis;
use super::parenthesis::ParenType;
use super::operator::Operator;
use super::primative::Primative;
use super::tokenizable::{Tokenizable, TokenizeResult};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
	Primative(Primative),
	Operator(Operator),
	Left(ParenType),
	Right(ParenType),
	Endline,
	Comma
}

impl Display for Token {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Token::Primative(p) => Display::fmt(p, f),
			Token::Operator(o) => Display::fmt(o, f),
			Token::Left(t) => Display::fmt(&t.left(), f),
			Token::Right(t) => Display::fmt(&t.right(), f),
			Token::Endline => Display::fmt(&";", f),
			Token::Comma => Display::fmt(&",", f),
		}		
	}
}


impl From<!> for Token {
	fn from(_: !) -> Self {
		unreachable!()
	}
}

impl Token {
	pub fn try_parse<S: Stream>(stream: &mut S) -> Result<Option<Self>> {
		use super::{whitespace::Whitespace, comment::Comment};
		macro_rules! try_tokenize {
			($($ty:ty),*) => {
				$(
					#[allow(unreachable_code)]
					match <$ty>::try_tokenize(stream)? {
						TokenizeResult::Some(val) => return Ok(Some(val.into())),
						TokenizeResult::RestartParsing => return Token::try_parse(stream),
						TokenizeResult::StopParsing => return Ok(None),
						TokenizeResult::None => { /* do nothing, go to the next one */ }
					}
				)*
			};
		}

		try_tokenize!(Whitespace, Comment, Primative, Parenthesis, Operator);

		match stream.next().transpose()? {
			Some(';') => Ok(Some(Token::Endline)),
			Some(',') => Ok(Some(Token::Comma)),
			Some(chr) => Err(parse_error!(stream, UnknownTokenStart(chr))),
			None => Ok(None)
		}
	}
}

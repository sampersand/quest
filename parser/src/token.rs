use crate::Result;
use crate::stream::{BufStream, Stream};
use std::io::BufRead;

mod literal;
mod operator;
mod tokenizable;
mod whitespace;
mod comment;
mod parenthesis;

use self::parenthesis::Parenthesis;
pub use self::parenthesis::ParenType;
pub use self::operator::Operator;
pub use self::literal::Literal;
pub use self::tokenizable::{Tokenizable, TokenizeResult};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum Token {
	Literal(Literal),
	Operator(Operator),
	Left(ParenType),
	Right(ParenType),
	Endline,
	Comma
}

impl Display for Token {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Token::Literal(l) => Display::fmt(l, f),
			Token::Operator(o) => Display::fmt(o, f),
			Token::Left(t) => Display::fmt(&t.left(), f),
			Token::Right(t) => Display::fmt(&t.right(), f),
			Token::Endline => write!(f, ";"),
			Token::Comma => write!(f, ","),
		}		
	}
}


impl Token {
	pub fn try_parse<S: Stream>(stream: &mut S) -> Result<Option<Self>> {
		use self::{whitespace::Whitespace, comment::Comment};
		macro_rules! try_tokenize {
			($($ty:ty),*) => {
				$(
					match <$ty>::try_tokenize(stream)? {
						TokenizeResult::Some(val) => return Ok(Some(val.into())),
						TokenizeResult::RestartParsing => return Token::try_parse(stream),
						TokenizeResult::StopParsing => return Ok(None),
						TokenizeResult::None => { /* do nothing, go to the next one */ }
					}
				)*
			};
		}

		try_tokenize!(Whitespace, Comment, Literal, Parenthesis, Operator);

		match stream.next().transpose()? {
			Some(';') => Ok(Some(Token::Endline)),
			Some(',') => Ok(Some(Token::Comma)),
			Some(chr) => Err(parse_error!(stream, UnknownTokenStart(chr))),
			None => Ok(None)
		}
	}
}





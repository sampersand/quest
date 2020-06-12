use crate::{Stream, Result};
use std::io::BufRead;

macro_rules! parse_error {
	(context=$context:expr, $type:ident $($tt:tt)*) => {
		crate::Error::new($context, crate::ErrorType::$type$($tt)*)
	};
	($stream:expr, $type:ident $($tt:tt)*) => {
		parse_error!(context=$stream.context().clone(), $type$($tt)*)
	};
}

mod literal;
mod operator;
mod parsable;
mod whitespace;
mod comment;
mod parenthesis;

use self::parenthesis::Parenthesis;
pub use self::parenthesis::ParenType;
pub use self::operator::Operator;
pub use self::literal::Literal;
pub use self::parsable::{Parsable, ParseResult};
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
	pub fn try_parse<S: BufRead>(stream: &mut Stream<S>) -> Result<Option<Self>> {
		use self::{whitespace::Whitespace, comment::Comment};
		macro_rules! try_parse {
			($($ty:ty),*) => {
				$(
					match <$ty>::try_parse(stream)? {
						ParseResult::Some(val) => return Ok(Some(val.into())),
						ParseResult::RestartParsing => return Token::try_parse(stream),
						ParseResult::StopParsing => return Ok(None),
						ParseResult::None => { /* do nothing, go to the next one */ }
					}
				)*
			};
		}

		// it's important whitespace is first, as it'll delete any extra whitespace before other
		// parsables see them as starting tokens.
		try_parse!(Whitespace, Comment, Literal, Operator, Parenthesis);

		match stream.next_char()? {
			Some(';') => Ok(Some(Token::Endline)),
			Some(',') => Ok(Some(Token::Comma)),
			Some(chr) => Err(parse_error!(stream, UnknownTokenStart(chr))),
			None => Ok(None)
		}
	}
}





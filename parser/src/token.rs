use crate::Stream;
use std::convert::TryFrom;
use std::io::{self, BufRead};

macro_rules! parse_error {
	(context=$context:expr, $type:ident $($tt:tt)*) => {
		crate::token::Error::new($context, crate::token::ErrorType::$type$($tt)*)
	};
	($stream:expr, $type:ident $($tt:tt)*) => {
		parse_error!(context=$stream.context().clone(), $type$($tt)*)
	};
}

mod error;
mod literal;
mod operator;
mod parsable;
mod whitespace;
mod comment;
mod parenthesis;

pub use self::error::{Error, ErrorType, Result};
use self::parenthesis::Parenthesis;
pub use self::parenthesis::ParenType;
pub use self::operator::Operator;
pub use self::literal::Literal;
pub use self::parsable::Parsable;

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

impl From<Literal> for Token {
	fn from(lit: Literal) -> Token {
		Token::Literal(lit)
	}
}

impl From<Operator> for Token {
	fn from(op: Operator) -> Token {
		Token::Operator(op)
	}
}

impl Parsable for Token {
	type Item = Self;

	fn try_parse<S: BufRead>(stream: &mut Stream<S>) -> Result<Option<Self>> {
		macro_rules! try_parse {
			($($ty:ty),*) => {
				$(
					if let Some(what) = <$ty>::try_parse(stream)? {
						return Ok(Some(what.into()));
					}
				)*
			};
		}

		// it's important whitespace is first, as it'll delete any extra whitespace before other
		// parsables see them as starting tokens.
		try_parse!(whitespace::Whitespace, comment::Comment, Literal, Operator, Parenthesis);

		match stream.next_char()? {
			Some(chr) => Err(Error::new(stream.context().clone(), ErrorType::UnknownTokenStart(chr))),
			None => Ok(None)
		}
	}
}








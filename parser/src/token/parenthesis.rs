use crate::token::{Token, Parsable, ParseResult};
use crate::{Result, Stream};
use std::fmt::{self, Display, Formatter};
use std::io::BufRead;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ParenType {
	Round, Square, Curly
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) struct Parenthesis;

impl Display for ParenType {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}{}", self.left(), self.right())
	}
}

impl ParenType {
	pub fn left(&self) -> char {
		match self {
			ParenType::Round  => '(',
			ParenType::Square => '[',
			ParenType::Curly  => '{'
		}
	}
	pub fn right(&self) -> char {
		match self {
			ParenType::Round  => ')',
			ParenType::Square => ']',
			ParenType::Curly  => '}'
		}
	}
}

impl From<ParenType> for quest::types::Text {
	fn from(paren_type: ParenType) -> Self {
		Self::from(paren_type.to_string())
	}
}

impl Parsable for Parenthesis {
	type Item = Token;
	fn try_parse<S: BufRead>(stream: &mut Stream<S>) -> Result<ParseResult<Token>> {
		match stream.next_char()? {
			Some('(') => Ok(ParseResult::Some(Token::Left(ParenType::Round))),
			Some(')') => Ok(ParseResult::Some(Token::Right(ParenType::Round))),
			Some('[') => Ok(ParseResult::Some(Token::Left(ParenType::Square))),
			Some(']') => Ok(ParseResult::Some(Token::Right(ParenType::Square))),
			Some('{') => Ok(ParseResult::Some(Token::Left(ParenType::Curly))),
			Some('}') => Ok(ParseResult::Some(Token::Right(ParenType::Curly))),
			Some(chr) => {
				stream.unshift_char(chr);
				Ok(ParseResult::None)
			},
			None => Ok(ParseResult::None)
		}
	}
}

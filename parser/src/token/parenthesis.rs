use crate::token::{Token, Tokenizable, TokenizeResult, Operator};
use crate::{Result, Stream};
use std::fmt::{self, Display, Formatter};
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ParenType {
	Round, Square, Curly
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub(super) struct Parenthesis;

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

impl From<ParenType> for Operator {
	fn from(paren_type: ParenType) -> Self {
		match paren_type {
			ParenType::Round => Operator::Call,
			ParenType::Square => Operator::Index,
			ParenType::Curly => Operator::WithBlock,
		}
	}
}

impl TryFrom<Operator> for ParenType {
	type Error = Operator;
	fn try_from(op: Operator) -> std::result::Result<Self, Operator> {
		match op {
			Operator::Call => Ok(ParenType::Round),
			Operator::Index => Ok(ParenType::Square),
			Operator::WithBlock => Ok(ParenType::Curly),
			other => Err(other)
		}
	}
}

impl Tokenizable for Parenthesis {
	type Item = Token;
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Token>> {
		match stream.next().transpose()? {
			Some('(') => Ok(TokenizeResult::Some(Token::Left(ParenType::Round))),
			Some(')') => Ok(TokenizeResult::Some(Token::Right(ParenType::Round))),
			Some('[') => Ok(TokenizeResult::Some(Token::Left(ParenType::Square))),
			Some(']') => Ok(TokenizeResult::Some(Token::Right(ParenType::Square))),
			Some('{') => Ok(TokenizeResult::Some(Token::Left(ParenType::Curly))),
			Some('}') => Ok(TokenizeResult::Some(Token::Right(ParenType::Curly))),
			Some(_) => {
				try_seek!(stream, Current(-1));
				Ok(TokenizeResult::None)
			},
			None => Ok(TokenizeResult::None)
		}
	}
}

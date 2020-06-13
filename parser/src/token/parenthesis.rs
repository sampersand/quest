use crate::token::{Token, Tokenizable, TokenizeResult};
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
			Some(chr) => {
				use std::io::{Seek, SeekFrom};
				stream.seek(SeekFrom::Current(-1))
					.map_err(|err| parse_error!(stream, CantReadStream(err)))?;
				Ok(TokenizeResult::None)
			},
			None => Ok(TokenizeResult::None)
		}
	}
}

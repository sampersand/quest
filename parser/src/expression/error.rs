use crate::token::{Token, Parenthesis};

use std::fmt::{self, Display, Formatter};

/// Represents an error that can happen when trying to construct an [`Expression`](super::Expression)
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
	/// An unexpected token was encountered
	UnexpectedToken(Token),
	/// An expression was expected, but no more tokens were left to parse.
	ExpectedExpression,
	/// A closing parenthesis was missing
	MissingClosingParen(Parenthesis),
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::UnexpectedToken(tkn) => write!(f, "unexpected token '{}'", tkn),
			Self::MissingClosingParen(paren) => write!(f, "missing closing paren '{}'", paren.right()),
			Self::ExpectedExpression => write!(f, "expected an expression"),
		}
	}
}

impl std::error::Error for Error {}
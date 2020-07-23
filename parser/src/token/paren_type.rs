use crate::token::Operator;
use std::fmt::{self, Display, Formatter};
use std::convert::TryFrom;
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ParenType {
	Round, Square, Curly
}

impl Display for ParenType {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}{}", self.left(), self.right())
	}
}

impl ParenType {
	#[must_use]
	pub fn left(self) -> char {
		match self {
			Self::Round  => '(',
			Self::Square => '[',
			Self::Curly  => '{'
		}
	}

	#[must_use]
	pub fn right(self) -> char {
		match self {
			Self::Round  => ')',
			Self::Square => ']',
			Self::Curly  => '}'
		}
	}
}

impl From<ParenType> for Operator {
	fn from(paren_type: ParenType) -> Self {
		match paren_type {
			ParenType::Round => Self::Call,
			ParenType::Square => Self::Index,
			ParenType::Curly => Self::WithBlock,
		}
	}
}

impl TryFrom<Operator> for ParenType {
	type Error = Operator;
	fn try_from(op: Operator) -> std::result::Result<Self, Operator> {
		match op {
			Operator::Call => Ok(Self::Round),
			Operator::Index => Ok(Self::Square),
			Operator::WithBlock => Ok(Self::Curly),
			other => Err(other)
		}
	}
}

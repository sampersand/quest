use crate::token::Operator;
use std::fmt::{self, Display, Formatter};
use std::convert::TryFrom;

/// Represents a parenthesis  in quest
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Parenthesis {
	/// Round (`()`) parenthesis.
	Round,
	/// Square (`[]`) parenthesis.
	Square,
	/// Curly (`{}`) parenthesis.
	Curly
}

impl Display for Parenthesis {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}{}", self.left(), self.right())
	}
}

impl Parenthesis {
	/// Gets the `char` that represents the left parenthesis.
	#[must_use]
	pub const fn left(self) -> char {
		match self {
			Self::Round  => '(',
			Self::Square => '[',
			Self::Curly  => '{'
		}
	}

	/// Gets the `char` that represents the right parenthesis.
	#[must_use]
	pub const fn right(self) -> char {
		match self {
			Self::Round  => ')',
			Self::Square => ']',
			Self::Curly  => '}'
		}
	}
}

impl From<Parenthesis> for Operator {
	fn from(paren: Parenthesis) -> Self {
		match paren {
			Parenthesis::Round => Self::Call,
			Parenthesis::Square => Self::Index,
			Parenthesis::Curly => Self::WithBlock,
		}
	}
}

impl TryFrom<Operator> for Parenthesis {
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

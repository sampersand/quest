use std::fmt::{self, Display, Formatter};

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

use crate::literals::Literal;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub enum TypeError {
	WrongType { expected: &'static str, got: &'static str },
	ConversionReturnedBadType { func: Literal, expected: &'static str, got: &'static str }
}

impl From<TypeError> for super::Error {
	fn from(key_error: TypeError) -> Self {
		Self::TypeError(key_error)
	}
}

impl Display for TypeError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "type error: ")?;
		match self {
			TypeError::WrongType { expected, got } => 
				write!(f, "expected type '{}' but got type '{}'", expected, got),
			TypeError::ConversionReturnedBadType { func, expected, got } =>
				write!(f, "'{}' returned non-{} type '{}'", func, expected, got),
		}
	}
}

use crate::literal::Literal;
use std::fmt::{self, Display, Formatter};

/// The type was wrong for a given operation.
#[derive(Debug, Clone)]
pub enum TypeError {
	/// An invalid type was used.
	// todo: remove this entirely.
	WrongType { /** */ expected: &'static str, /** */ got: &'static str },
	/// A conversion returned a bad type.
	ConversionReturnedBadType {
		/// The conversion function.
		func: Literal,
		/// The type that was expected after the function was called.
		expected: &'static str,
		/// The type that was actually received after the function was called.
		got: &'static str
	},
	/// A number was used and wasn't actually an integer.
	NotAnInteger(crate::types::Number),
	/// Some other message.
	Messaged(String)
}

impl From<TypeError> for super::Error {
	#[inline]
	fn from(err: TypeError) -> Self {
		Self::TypeError(err)
	}
}

impl Display for TypeError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "type error: ")?;
		match self {
			Self::WrongType { expected, got } => 
				write!(f, "expected type '{}' but got type '{}'", expected, got),
			Self::ConversionReturnedBadType { func, expected, got } =>
				write!(f, "'{}' returned non-{} type '{}'", func, expected, got),
			Self::NotAnInteger(num) => write!(f, "'{}' isn't an integer", num),
			Self::Messaged(ref msg) => Display::fmt(msg, f),
		}
	}
}

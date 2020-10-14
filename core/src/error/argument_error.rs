use std::fmt::{self, Display, Formatter};

/// The type was correct, but its value was incorrect.
#[derive(Debug, Clone)]
pub enum ArgumentError {
	/// When a more specific error isn't available
	InvalidLength { given: usize, expected: usize }
}

impl From<ArgumentError> for super::Error {
	#[inline]
	fn from(err: ArgumentError) -> Self {
		Self::ArgumentError(err)
	}
}

impl Display for ArgumentError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "argument error: ")?;
		match self {
			ArgumentError::InvalidLength { given, expected }
				=> write!(f, "wrong number of arguments (given {}, expected {})", given, expected)
		}
	}
}

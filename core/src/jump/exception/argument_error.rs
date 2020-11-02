use std::fmt::{self, Display, Formatter};

/// Raised when there's a problem when passing arguments, such as when too few are given.
#[derive(Debug, Clone, PartialEq)]
pub struct ArgumentError {
	message: String
}

impl ArgumentError {
	/// Creates a new `ArgumentError` with the given message.
	#[inline]
	pub fn new<M: Into<String>>(message: M) -> Self {
		Self { message: message.into() }
	}
}

impl std::error::Error for ArgumentError {}
impl Display for ArgumentError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "argument error: {}", self.message)
	}
}

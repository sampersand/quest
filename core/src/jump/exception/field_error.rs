use std::fmt::{self, Display, Formatter};

/// Raised when there's a problem when accessing a field for an [`Object`](crate::Object).
#[derive(Debug, Clone, PartialEq)]
pub struct FieldError {
	message: String
}

impl FieldError {
	/// Creates a new `FieldError` with the given message.
	#[inline]
	pub fn new<M: Into<String>>(message: M) -> Self {
		Self { message: message.into() }
	}
}

impl From<String> for FieldError {
	#[inline]
	fn from(val: String) -> Self {
		Self(val)
	}
}

impl std::error::Error for FieldError {}
impl Display for FieldError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "field error: {}", self.message)
	}
}

use std::fmt::{self, Display, Formatter};

/// The type was correct, but its value was incorrect.
#[derive(Debug, Clone)]
pub enum ValueError {
	/// When a more specific error isn't available
	Messaged(String)
}

impl From<ValueError> for super::Error {
	#[inline]
	fn from(err: ValueError) -> Self {
		Self::ValueError(err)
	}
}

impl Display for ValueError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "value error: ")?;
		match self {
			ValueError::Messaged(msg) => Display::fmt(&msg, f),
		}
	}
}

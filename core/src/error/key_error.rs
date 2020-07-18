use std::fmt::{self, Display, Formatter};
use crate::Object;

#[derive(Debug, Clone)]
pub enum KeyError {
	OutOfBounds { idx: isize, len: usize },
	BadSlice { slice: String, len: usize },
	DoesntExist { attr: Object, obj: Object },
}

impl From<KeyError> for super::Error {
	fn from(key_error: KeyError) -> Self {
		Self::KeyError(key_error)
	}
}

impl Display for KeyError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "key error: ")?;

		match self {
			KeyError::OutOfBounds { idx, len } => 
				write!(f, "index '{}' out of bounds (max: {})", idx, len),
			KeyError::BadSlice { slice, len } => 
				write!(f, "slice '{}' out of bounds (max: {})", slice, len),
			KeyError::DoesntExist { attr, obj } => 
				write!(f, "attr {:?} doesn't exist for {:?}", attr, obj),
		}
	}
}

use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub enum KeyError {
	OutOfBounds { idx: usize, len: usize },
	BadSlice { slice: String, len: usize },
	CantIndexByZero,
	NoThisSupplied
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
			KeyError::CantIndexByZero => write!(f, "indexing by 0 is not supported"),
			KeyError::NoThisSupplied =>  write!(f, "no '__this__' supplied"),
		}
	}
}

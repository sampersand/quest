use std::fmt::{self, Display, Formatter};
use crate::Object;

/// There was a problem with accessing a field.
///
/// In the future, this may be removed entirely, if I decide that accessing an invalid field just
/// simply returns [`null`](crate::types::Null).
#[derive(Debug, Clone)]
pub enum KeyError {
	/// The index was out of bounds for an array.
	OutOfBounds {
		/// The index that was out of bounds
		idx: isize,
		/// The length of the array that was being accessed
		len: usize
	},

	/// The slice was out of bounds for an array.
	BadSlice {
		/// A string repr of the length
		range: String,
		/// The length of the array
		len: usize
	},

	/// The attribute doesn't exist for the given object.
	DoesntExist {
		/// The attribute that doens't exist.
		attr: Object,
		/// The object that we're trying to get the attribute of
		obj: Object
	},

	// /// The attribute doesn't exist for the given object.
	// DoesntExist1 {
	// 	/// The attribute that doens't exist.
	// 	attr: crate::object::ObjectAttempt,
	// 	/// The object that we're trying to get the attribute of
	// 	obj: crate::object::ObjectAttempt
	// },
}

impl From<KeyError> for super::Error {
	#[inline]
	fn from(err: KeyError) -> Self {
		Self::KeyError(err)
	}
}

impl Display for KeyError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "key error: ")?;

		match self {
			KeyError::OutOfBounds { idx, len } => 
				write!(f, "index '{}' out of bounds (max: {})", idx, len),
			KeyError::BadSlice { range, len } => 
				write!(f, "range '{}' out of bounds (max: {})", range, len),
			KeyError::DoesntExist { attr, obj } => 
				write!(f, "attr {:?} doesn't exist for {:?}", attr, obj),
			// KeyError::DoesntExist1 { attr, obj } => 
			// 	write!(f, "attr {:?} doesn't exist for {:?}", attr, obj),
		}
	}
}

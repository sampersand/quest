use super::{Value, Tag};
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Null;


/// Methods relating to [`Null`].
impl Value {
	/// Creates a null [`Value`].
	pub fn new_null() -> Self {
		Self::NULL
	}

	/// Checks to see if `self` is null.
	pub fn is_null(&self) -> bool {
		self.bits() == Self::NULL.bits()
	}

	// NB: There's not `as_null`/`as_null_unchecked` because null is a ZST.
}


impl From<Null> for Value {
	fn from(_: Null) -> Self {
		Self::NULL
	}
}

impl TryFrom<Value> for Null {
	type Error = Value;
	fn try_from(value: Value) -> Result<Self, Self::Error> {
		if value.is_null() {
			Ok(Null)
		} else {
			Err(value)
		}
	}
}

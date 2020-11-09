use super::Value;
use std::convert::TryFrom;
use quest_core::types::Boolean;

impl Value {
	/// Creates a boolean [`Value`].
	pub const fn new_bool(value: Boolean) -> Self {
		if value.into_inner() {
			Self::TRUE
		} else {
			Self::FALSE
		}
	}

	/// Checks to see if `self` is a boolean.
	pub fn is_bool(&self) -> bool {
		self.as_bool().is_some()
	}

	// NB: There's not `as_bool_unchecked` because we must check all bits to determine if something's a boolean.

	/// Retrieves the underlying boolean value of `self`, if `self` is a boolean.
	pub fn as_bool(&self) -> Option<Boolean> {
		if *self == Self::TRUE {
			Some(Boolean::TRUE)
		} else if *self == Self::FALSE {
			Some(Boolean::FALSE)
		} else {
			None
		}
	}
}

impl From<Boolean> for Value {
	#[inline]
	fn from(bool: Boolean) -> Self {
		Self::new_bool(bool)
	}
}

impl TryFrom<Value> for Boolean {
	type Error = Value;
	fn try_from(value: Value) -> Result<Self, Self::Error> {
		value.as_bool().ok_or(value)
	}
}

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	// mod 
// }

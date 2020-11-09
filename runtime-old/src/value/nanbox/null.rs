use super::Value;
use std::convert::TryFrom;
use quest_core::types::Null;

/// Methods relating to [`Null`].
impl Value {
	/// Creates a null [`Value`].
	pub const fn new_null() -> Self {
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

#[cfg(test)]
mod tests {
	use super::*;
	use super::super::Tag;

	#[test]
	fn only_eqls_itself() {
		assert_eq!(Value::NULL, Value::NULL);
		assert_ne!(Value::NULL, Value::TRUE);
	}

	#[test]
	fn tag_is_const() {
		assert_eq!(Value::NULL.tag(), Tag::CONSTS);
	}

	#[test]
	fn new_null_returns_null() {
		assert_eq!(Value::new_null(), Value::NULL);
	}

	#[test]
	fn is_null() {
		assert!(Value::NULL.is_null());
		assert!(!Value::FALSE.is_null());
	}
}

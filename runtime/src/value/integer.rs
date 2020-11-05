use super::{Value, ValueInner, Tag};
use std::convert::TryFrom;
use crate::Integer;

impl Value {
	/// Creates a new integer `Value`.
	pub const fn new_int(int: Integer) -> Self {
		// SAFETY: all `Integer`s are valid b/c all bit patterns are valid for `INT`.
		unsafe {
			Self::new_tagged(Tag::INT, int.to_bytes())
		}
	}

	// SAFETY: `0` is a valid bit pattern.
	pub const ZERO: Self = Self::new_int(Integer::ZERO);

	/// Checks to see if `self` is an `int48`.
	pub fn is_int(&self) -> bool {
		self.tag() == Tag::INT
	}

	/// Converts `self` to a `Integer` without checking the tag first.
	///
	/// # Safety
	/// The caller must ensure this is called on a Integer value.
	pub unsafe fn as_int_unchecked(&self) -> Integer {
		debug_assert_eq!(self.tag(), Tag::INT, "bad tag encountered for int48. self={:?}", self);

		Integer::new_unchecked(self.masked_data() as i64)
	}

	/// Attempts to convert `self` to a float.
	pub fn as_int(&self) -> Option<Integer> {
		if self.is_int() {
			// SAFETY: we just verified it was a int48, so we can just interpret it as a `i64` safely.
			Some(unsafe { self.as_int_unchecked() })
		} else {
			None
		}
	}
}

impl From<i64> for Value {
	#[inline]
	fn from(int: i64) -> Self {
		if let Some(int48) = Integer::new(int) {
			Self::new_int(int48)
		} else {
			Self::new_object(int)
		}
	}
}

impl TryFrom<Value> for Integer {
	type Error = Value;
	fn try_from(value: Value) -> Result<Self, Self::Error> {
		value.as_int().ok_or(value)
	}
}

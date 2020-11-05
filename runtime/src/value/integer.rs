use super::{Value, ValueInner, Tag};
use std::convert::TryFrom;

pub type Int48 = i64;

const I48_MAX: u64 = 0xffff_ffff_ffff;

impl Value {
	/// Creates a new integer `Value`.
	///
	/// Values that are 
	pub fn new_integer(int: Int48) -> Self {
		if int >> 48 != 0 {
			Self::new_object(int)
		} else  {
			// SAFETY: We just verified that we're within the bounds.
			unsafe {
				Self::new_integer_unchecked(int)
			}
		}
	}

	/// Creates a new integer 
	pub unsafe fn new_integer_unchecked(int: Int48) -> Self {
		let [u1, u2, data @ ..] = int.to_be_bytes();
		debug_assert_eq!(u1 + u2, 0, "Attempted to create an int48 with an invalid int: {:x}", int);

		// SAFETY: All bit patterns are valid values for INT48.
		unsafe { 
			Self::new_tagged(Tag::INT48, data)
		}
	}

	/// Checks to see if `self` is an `integer`.
	pub fn is_integer(&self) -> bool {
		self.tag() == Tag::INT48
	}

	/// Converts `self` to a `Int48` without checking the tag first.
	///
	/// # Safety
	/// The caller must ensure this is called on a Int48 value.
	pub unsafe fn as_integer_unchecked(&self) -> Int48 {
		debug_assert_eq!(self.tag(), Tag::INT48, "bad tag encountered for int48. self={:?}", self);

		self.masked_data() as i64
	}

	/// Attempts to convert `self` to a float.
	pub fn as_integer(&self) -> Option<Int48> {
		if self.is_integer() {
			// SAFETY: we just verified it was a integer, so we can just interpret it as a `i64` safely.
			Some(unsafe { self.as_integer_unchecked() })
		} else {
			None
		}
	}
}

impl From<Int48> for Value {
	#[inline]
	fn from(int48: Int48) -> Self {
		Self::new_integer(int48)
	}
}

impl TryFrom<Value> for Int48 {
	type Error = Value;
	fn try_from(value: Value) -> Result<Self, Self::Error> {
		value.as_integer().ok_or(value)
	}
}

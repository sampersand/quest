use crate::value::{Value, ValueConvertable};
use std::fmt::{self, Display, Formatter};

/// The null value in Quest.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Null;

impl Null {
	const NULL_U64: u64 = 0;
}

// SAFETY: No valid pointers return `0`, and none of the other values are defined to use `0`.
unsafe impl ValueConvertable for Null {
	#[inline]
	fn into_value(self) -> Value {
		// SAFETY: `NULL_U64` _is_ `Null` as a u64.
		unsafe {
			Value::from_inner_unchecked(Self::NULL_U64)
		}
	}

	#[inline]
	fn is_value(value: &Value) -> bool {
		value.inner() == Self::NULL_U64
	}

	#[inline]
	unsafe fn from_value_unchecked(value: Value) -> Self {
		debug_assert!(Self::is_value(&value), "invalid value given: {:#?}", value);

		Self
	}
}

impl Display for Null {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&"null", f)
	}
}

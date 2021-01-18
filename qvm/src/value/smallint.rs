use crate::value::{Value, ValueType, Literal, NamedType};
use std::fmt::{self, Display, Formatter};

/// A small number in Quest, ie fittable within a [`Value`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SmallInt(i64);

impl SmallInt {
	/// The maximum value a [`SmallInt`] can hold.
	pub const MAX: Self = Self(i64::MAX >> 1);

	/// The minimum value a [`SmallInt`] can hold.
	pub const MIN: Self = Self(i64::MIN >> 1);

	/// The value zero.
	pub const ZERO: Self = Self(0);

	/// checks to see if a value's within bounds.
	#[inline]
	const fn within_bounds(number: i64) -> bool {
		Self::MIN.0 <= number && number <= Self::MAX.0
	}

	/// Creates a new number from `number`, returning `None` if it's out of bounds.
	pub fn new(number: i64) -> Option<Self> {
		if Self::within_bounds(number) {
			// SAFETY: We know we're within bounds as we just checked.
			unsafe {
				Some(Self::new_unchecked(number))
			}
		} else {
			None
		}
	}

	/// Creates a new [`SmallInt`] without checking that `number` is within bounds.
	///
	/// # Safety
	/// Tjhe caller must ensure that `number` is actually within bounds.
	#[inline]
	pub unsafe fn new_unchecked(number: i64) -> Self {
		debug_assert!(Self::within_bounds(number), "bad number given to new_unchecked: {}", number);

		Self(number)
	}

	/// Unwraps `self` and returns its enclosed value.
	pub fn into_inner(self) -> i64 {
		self.0
	}

}

impl NamedType for SmallInt {
	#[inline(always)]
	fn typename() -> &'static str {
		"Number"
	}
}

const SMALLINT_TAG:   u64 = 0b0001;
const SMALLINT_MASK:  u64 = 0b0001;
const SMALLINT_SHIFT: u64 = 1;

// SAFETY: The way this is stored internally is by shifting the `i64` one left and `OR`ing it with `1`.
// This works because all of the allocated objects are pointers, which haven an alignment of 8, and thus cannot have a
// least-significant-bit of `1`. No other types are defined to have odd values.
unsafe impl ValueType for SmallInt {
	#[inline]
	fn into_value(self) -> Value {
		// SAFETY: We are defining what it means to be a valid smallint here.
		unsafe {
			Value::from_bits_unchecked(((self.0 as u64) << SMALLINT_SHIFT) | SMALLINT_TAG)
		}
	}

	fn is_value_a(value: &Value) -> bool {
		value.bits() & SMALLINT_MASK == SMALLINT_TAG
	}

	unsafe fn value_into_unchecked(value: Value) -> Self {
		debug_assert!(value.is_a::<Self>(), "invalid value given: {:#?}", value);

		// SAFETY: we know this is safe because all (validly constructed) values are valid small ints.
		unsafe {
			Self::new_unchecked((value.bits() as i64) >> SMALLINT_SHIFT)
		}
	}
}

impl AsRef<i64> for SmallInt {
	#[inline]
	fn as_ref(&self) -> &i64 {
		&self.0
	}
}

impl Display for SmallInt {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

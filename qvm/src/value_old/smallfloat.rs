use crate::value::{Value, ValueConvertable};
use std::fmt::{self, Display, Formatter};


/// A small float in Quest.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SmallFloat(f32);

impl SmallFloat {
	/// The value zero.
	pub const ZERO: Self = Self(0.0);

	/// Creates a new [`SmallFloat`] from `float`.
	pub fn new(float: f32) -> Self {
		Self(float)
	}

	/// Unwraps `self` and returns its enclosed value.
	pub fn into_inner(self) -> f32 {
		self.0
	}
}

// SAFETY: SmallFloats are defined as `0b0...0110'.
unsafe impl ValueConvertable for SmallFloat {
	#[inline]
	fn into_value(self) -> Value {
		let inner = ((self.0.to_bits() as u64) << 3) | 0b110;

		// SAFETY: We just constructed a valid `inner`, so we know that it's a valid `Value`.
		unsafe {
			Value::from_inner_unchecked(inner)
		}
	}

	fn is_value(value: &Value) -> bool {
		value.inner() & 0b111 == 0b110
	}

	unsafe fn from_value_unchecked(value: Value) -> Self {
		debug_assert!(Self::is_value(&value), "invalid value given: {:#?}", value);

		Self::new(f32::from_bits((value.inner() >> 3) as u32))
	}
}

impl AsRef<f32> for SmallFloat {
	#[inline]
	fn as_ref(&self) -> &f32 {
		&self.0
	}
}

impl AsMut<f32> for SmallFloat {
	#[inline]
	fn as_mut(&mut self) -> &mut f32 {
		&mut self.0
	}
}

impl Display for SmallFloat {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

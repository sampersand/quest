use crate::value::{Value, Literal, ValueType};


/// The floating point type.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Named)]
#[repr(transparent)]
#[quest(crate_name="crate", name="Number")]
pub struct Float(f32);

impl Float {
	pub const fn new(float: f32) -> Self {
		Self(float)
	}
}

const FLOAT_TAG: u64   = 0b0110;
const FLOAT_MASK: u64  = 0b0111;
const FLOAT_SHIFT: u64 =      3;

unsafe impl ValueType for Float {
	#[inline]
	fn into_value(self) -> Value {
		// SAFETY: This is the definition of a valid float.
		unsafe {
			Value::from_bits_unchecked(((self.0.to_bits() as u64) << FLOAT_SHIFT) | FLOAT_TAG)
		}
	}

	#[inline]
	fn is_value_a(value: &Value) -> bool {
		(value.bits() & FLOAT_MASK) == FLOAT_TAG
	}

	unsafe fn value_into_unchecked(value: Value) -> Self {
		debug_assert!(value.is_a::<Self>());
		debug_assert_eq!(0, (value.bits() >> FLOAT_SHIFT) & !(u32::MAX as u64));

		Self(f32::from_bits((value.bits() >> FLOAT_SHIFT) as u32))
	}
}

impl From<f32> for Float {
	#[inline]
	fn from(float: f32) -> Self {
		Self(float)
	}
}

impl From<Float> for f32 {
	#[inline]
	fn from(float: Float) -> Self {
		float.0
	}
}

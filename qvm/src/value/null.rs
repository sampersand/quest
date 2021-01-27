use crate::value::{Value, ValueType, Literal};

/// The null type.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Named)]
#[quest(crate_name="crate")]
pub struct Null;

pub(super) const NULL_BITS: u64 = 0b0100;

unsafe impl ValueType for Null {
	#[inline]
	fn into_value(self) -> Value {
		// SAFETY: This is the definition of a valid float.
		unsafe {
			Value::from_bits_unchecked(NULL_BITS)
		}
	}

	#[inline]
	fn is_value_a(value: &Value) -> bool {
		value.bits() == NULL_BITS
	}

	#[inline]
	fn try_value_into(value: Value) -> Result<Self, Value>  {
		if Self::is_value_a(&value) {
			Ok(Self)
		} else {
			Err(value)
		}
	}

	#[inline]
	unsafe fn value_into_unchecked(value: Value) -> Self {
		debug_assert!(value.is_a::<Self>());

		Self
	}
}

use crate::value::{Value, QuestValue};

pub type Boolean = bool;

// Note: It's defined as `0` so it can easily be cast to false in Rust.
const FALSE_BITS: u64 = 0b00;
const TRUE_BITS: u64 =  0b10;

unsafe impl QuestValue for Boolean {
	#[inline]
	fn into_value(self) -> Value {
		// SAFETY: This is the definition of a valid boolean.
		unsafe {
			if self {
				Value::from_bits_unchecked(TRUE_BITS)
			} else {
				Value::from_bits_unchecked(FALSE_BITS)
			}
		}
	}

	#[inline]
	fn is_value_a(value: &Value) -> bool {
		// just little optimization :D
		(value.bits() & !TRUE_BITS) == 0
	}

	#[inline]
	unsafe fn value_into_unchecked(value: Value) -> Self {
		debug_assert!(value.is_a::<Self>());

		value.bits() != FALSE_BITS
	}
}


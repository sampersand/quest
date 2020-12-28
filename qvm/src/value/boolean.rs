use crate::value::{Value, QuestValue, Literal};

pub type Boolean = bool;

// Note: It's defined as `0` so it can easily be cast to false in Rust.
pub(super) const FALSE_BITS: u64 = 0b00;
pub(super) const TRUE_BITS: u64 =  0b10;

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

	fn get_attr(&self, attr: Literal) -> Option<&Value> {
		todo!()
	}

	fn get_attr_mut(&mut self, attr: Literal) -> Option<&mut Value> {
		todo!()
	}

	fn del_attr(&mut self, attr: Literal) -> Option<Value> {
		todo!()
	}

	fn set_attr(&mut self, attr: Literal, value: Value) {
		todo!()
	}
}


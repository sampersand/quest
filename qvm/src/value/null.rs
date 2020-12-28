use crate::value::{Value, QuestValue, Literal};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Null;

pub(super) const NULL_BITS: u64 = 0b00100;

unsafe impl QuestValue for Null {
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
	unsafe fn value_into_unchecked(value: Value) -> Self {
		debug_assert!(value.is_a::<Self>());

		Self
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

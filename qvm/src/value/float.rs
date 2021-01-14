use crate::value::{Value, Literal, QuestValue};

pub type Float = f32;

const FLOAT_TAG: u64   = 0b0110;
const FLOAT_SHIFT: u64 = 0b0100;
const FLOAT_MASK: u64  = 0b0111;

unsafe impl QuestValue for Float {
	const TYPENAME: &'static str = "qvm::Float";

	#[inline]
	fn into_value(self) -> Value {
		// SAFETY: This is the definition of a valid float.
		unsafe {
			Value::from_bits_unchecked(((self.to_bits() as u64) << FLOAT_SHIFT) | FLOAT_TAG)
		}
	}

	#[inline]
	fn is_value_a(value: &Value) -> bool {
		(value.bits() & FLOAT_MASK) == FLOAT_TAG
	}

	unsafe fn value_into_unchecked(value: Value) -> Self {
		debug_assert!(value.is_a::<Self>());
		debug_assert_eq!(0, (value.bits() >> FLOAT_SHIFT) & !(u32::MAX as u64));

		Self::from_bits((value.bits() >> FLOAT_SHIFT) as u32)
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

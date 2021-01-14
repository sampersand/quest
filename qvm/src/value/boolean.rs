use crate::value::{Value, QuestValue, Literal, QuestConvertible};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Boolean(pub bool);

impl Boolean {
	#[inline]
	pub const fn new(boolean: bool) -> Self {
		Self(boolean)
	}

	// Note: It's defined as `0` so it can easily be cast to false in Rust.
	pub(super) const FALSE_BITS: u64 = 0b0000;
	pub(super) const TRUE_BITS: u64 =  0b0010;
}

unsafe impl QuestValue for Boolean {
	const TYPENAME: &'static str = "qvm::Boolean";

	#[inline]
	fn into_value(self) -> Value {
		// SAFETY: This is the definition of a valid boolean.
		unsafe {
			if self.0 {
				Value::from_bits_unchecked(Self::TRUE_BITS)
			} else {
				Value::from_bits_unchecked(Self::FALSE_BITS)
			}
		}
	}

	#[inline]
	fn is_value_a(value: &Value) -> bool {
		// just little optimization :D
		(value.bits() & !Self::TRUE_BITS) == 0
	}

	#[inline]
	unsafe fn value_into_unchecked(value: Value) -> Self {
		debug_assert!(value.is_a::<Self>());

		Self(value.bits() != Self::FALSE_BITS)
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

impl QuestConvertible for Boolean {
	const CONVERT_FUNCTION: Literal = Literal::AT_BOOL;
}


impl Boolean {
	pub fn at_bool(&self, args: &[&Value]) -> crate::Result<Value> {
		Ok(Value::new(*self))
	}

}
// mod fns {
// 	use super::*;

// 	pub fn at_bool(value: &Value, args: &[&Value]) -> crate::Result<Value> {
// 		if let Some(boolean) = value.downcast_call::<Self>() {
// 			return Ok(Value::new(boolean))

// 		}
// 	}

// 		if let _ = value, Self, "invalid `self` given.") {

// 		}
// 		// strict_arguments_check!(value: Null);

// 		panic!();
// 	}
// }

// }

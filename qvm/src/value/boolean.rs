use crate::value::{Value, ValueType, Literal, QuestConvertible};

/// The Boolean type.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Named)]
#[repr(transparent)]
#[quest(crate_name="crate")]
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

impl From<Boolean> for Value {
	#[inline]
	fn from(boolean: Boolean) -> Self {
		// SAFETY: This is the definition of a valid boolean.
		unsafe {
			if boolean.0 {
				Value::from_bits_unchecked(Boolean::TRUE_BITS)
			} else {
				Value::from_bits_unchecked(Boolean::FALSE_BITS)
			}
		}
	}
}

unsafe impl ValueType for Boolean {
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

use crate::value::{Value, Literal, QuestValue};
use std::fmt::{self, Debug, Display, Formatter};

// type Func = fn(Value, Value) -> Value; 
type Func = fn(&Value, &[&Value]) -> crate::Result<Value>;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct RustFn(fn(&Value, &[&Value]) -> crate::Result<Value>);

impl RustFn {
	pub fn new<SELF>(name: Literal, func: Func) -> Self {
		// TODO: Map names to `Literal`s.
		let _ = name;

		// SAFETY: we just added ourselves to the map. (tho we didnt...)
		unsafe {
			Self::new_unchecked(func)
		}
	}

	#[inline]
	pub unsafe fn new_unchecked(func: Func) -> Self {
		Self(func)
	}

	fn name(&self) -> &'static str {
		todo!();
	}
}

impl Debug for RustFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_tuple("RustFn").field(&self.name()).finish()
	}
}

impl Display for RustFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(self.name(), f)
	}
}

impl Eq for RustFn {}
impl PartialEq for RustFn {
	fn eq(&self, rhs: &Self) -> bool {
		// if two functions have the same address, we define them as the same RustFn.
		(self.0 as u64) == (rhs.0 as u64)
	}
}

const RUSTFN_TAG: u64   = 0b00000110;
const RUSTFN_SHIFT: u64 = 0b00000100;
const RUSTFN_MASK: u64  = 0b00000111;

unsafe impl QuestValue for RustFn {
	const TYPENAME: &'static str = "qvm::RustFn";

	#[inline]
	fn into_value(self) -> Value {
		// SAFETY: This is the definition of a valid float.
		unsafe {
			Value::from_bits_unchecked(((self.0 as u64) << RUSTFN_SHIFT) | RUSTFN_TAG)
		}
	}

	#[inline]
	fn is_value_a(value: &Value) -> bool {
		(value.bits() & RUSTFN_MASK) == RUSTFN_TAG
	}

	/// Note the value has to have been a valid rustfn.
	unsafe fn value_into_unchecked(value: Value) -> Self {
		debug_assert!(value.is_a::<Self>());

		let bits: u64 = value.bits() >> RUSTFN_SHIFT;
		debug_assert_ne!(0, bits, "null function encountered.");

		// SAFETY: if `value` was previously a `RustFn`, we know it's valid.
		Self::new_unchecked(unsafe {
			std::mem::transmute::<usize, Func>(bits as usize)
		})
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

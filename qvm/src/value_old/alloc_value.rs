use std::mem::{size_of, align_of};
use crate::value::*;

#[repr(C, align(8))]
pub union AllocValue {
	text: [u8; size_of::<Text>()],
	object: [u8; size_of::<Object>()],
}

/// A hack to get the size of `AllocValue`, as recursive definitions aren't allowed.
pub const ALLOC_SIZE: usize = 64;
pub const ALLOC_ALIGN: usize = 8;

const_assert_eq!(ALLOC_SIZE, size_of::<AllocValue>());
const_assert_eq!(ALLOC_ALIGN, align_of::<AllocValue>());

// SAFETY: Since all our pointers are 8-aligned, we reserve multiples of 8 for pointers.
unsafe impl ValueConvertable for *const AllocValue {
	fn into_value(self) -> Value {
		let inner = self as u64;
		debug_assert_eq!(inner & (ALLOC_ALIGN as u64 - 1), 0, "invalid pointer: {:p}", self);
		debug_assert!(!self.is_null(), "attempted to convert a null pointer!");

		// SAFETY: We know that `inner`'s a multiple of 8 due tot he `const_assert_eq`s above.
		unsafe {
			Value::from_inner_unchecked(inner)
		}
	}

	fn is_value(value: &Value) -> bool {
		value.inner() & (ALLOC_ALIGN as u64 - 1) == 0
	}

	unsafe fn from_value_unchecked(value: Value) -> Self {
		debug_assert!(Self::is_value(&value), "invalid value given: {:#?}", value);

		value.inner() as Self
	}
}

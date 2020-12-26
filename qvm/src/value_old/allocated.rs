/*#[cfg(debug_assertions)]
use std::any::TypeId;
use crate::value::{Value, Basic, ValueConvertable};


/// The type that represents allocated data on the stack.
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Allocated(Box<Inner>);

#[derive(Debug, Clone)]
#[repr(C, align(8))]
struct Inner {
	basic: Basic,
	data: *mut (),	
	drop: fn(*mut ()),
	#[cfg(debug_assertions)]
	typeid: TypeId
}

impl Allocated {
	pub fn new<T: Drop + 'static>(data: T) -> Self {
		Self::with_basic(Basic::default(), data)
	}

	pub fn with_basic<T: Drop + 'static>(basic: Basic, data: T) -> Self {
		Self(Box::new(Inner {
			basic,
			data: Box::into_raw(Box::new(data)),
			// SAFETY: we know `x` is a `T` because we never change `data`'s type.
			drop: |x| unsafe { std::ptr::drop_in_place(x as *mut T) },
			#[cfg(debug_assertions)]
			typeid: TypeId::of::<T>()
		}))
	}
}

impl<T> Drop for Allocated<T> {
	fn drop(&mut self) {
		self.drop(self.data)
	}
}

// SAFETY: Since all our pointers are 8-aligned, we reserve multiples of 8 for pointers.
unsafe impl<T> ValueConvertable for Allocated<T> {
	fn into_value(self) -> Value {
		let ptr = Box::into_raw(self.0) as u64;

		debug_assert_eq!(ptr & 0b111, 0, "invalid lower bits for pointer: {:p}", ptr as *const ());
		debug_assert!(!(ptr as *const ()).is_null(), "attempted to convert a null pointer!");

		// SAFETY: We know that `ptr`'s a multiple of 8 due tot he `const_assert_eq`s above.
		unsafe {
			Value::from_inner_unchecked(ptr)
		}
	}

	fn is_value(value: &Value) -> bool {
		value.inner() != 0 && value.inner() & (0b111) == 0
	}

	unsafe fn from_value_unchecked(value: Value) -> Self {
		debug_assert!(Self::is_value(&value), "invalid value given: {:#?}", value);

		// value.inner() as 
		todo!();

	}
}

// impl Allocated {
*/

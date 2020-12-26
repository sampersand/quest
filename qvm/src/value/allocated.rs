mod text;
mod custom;
mod object;

pub use text::*;
pub use custom::*;
pub use object::*;

use crate::value::{Value, QuestValue, QuestValueRef};
use std::fmt::{self, Debug, Display, Formatter};
use std::mem::ManuallyDrop;

#[repr(C, align(8))]
pub struct Allocated {
	flags: u64,
	data: Data
}

#[repr(C, align(16))] // 16's an arbitrary number: todo pick a better one
union Data {
	raw: [u8; 32],
	text: (),
	bignum: (),
	regex: (),
	list: (),
	map: (),
	object: (),
	class: (),
	custom: ManuallyDrop<Custom>,
}

const FLAG_CLASS_MASK: u64 =   0b00111111;
const FLAG_CLASS_TEXT: u64 =   0b00000000;
const FLAG_CLASS_BIGNUM: u64 = 0b00000001;
const FLAG_CLASS_REGEX: u64 =  0b00000010;
const FLAG_CLASS_LIST: u64 =   0b00000011;
const FLAG_CLASS_MAP: u64 =    0b00000100;
const FLAG_CLASS_OBJECT: u64 = 0b00000101;
const FLAG_CLASS_CLASS: u64 =  0b00000110;
const FLAG_CLASS_CUSTOM: u64 = 0b00000111;

/// A trait representing any value within Quest.
///
/// # Safety
/// The implementor must ensure that:
/// - Every [`into_value()`] produces a unique [`Value`], which no other implementation will return.
/// - [`is_value_a()`] will always return `true` if the value was constructed via [`into_value`] and `false` otherwise.
/// - [`try_value_into()`] must return `Ok(Self)` when the given `value` was constructed via [`Self::into_value()`], and
///   return the original [`Value`] if the value isn't a `Self`.
/// - [`value_into_unchecked()`] must return valid results for any [`Value`] constructed via `Self::into_value`.
///
/// If left unchanged, the default implementation of [`AllocatedType`] does all this correctly.
pub unsafe trait AllocatedType : Debug + Sized {
	/// Convert `self` into a [`Value`].
	fn into_allocated(self) -> Value {
		Allocated::new(self).into()
	}

	/// Checks to see if an [`Allocated`] is a `Self`.
	fn is_allocated_a(value: &Value) -> bool {
		// allocated.try_as_ref::<Custom>().map_or(Allocated)
		// value.try_as_ref::<Allocated>().map_or(false, Allocated::is_alloc_a::<Self>)
		todo!()
	}

	/// Tries to unpack `value` into `Self`, returning `Err(Value)` if the value's not the right type
	///
	/// Implementations generally won't need to override this, as the default behaviour is in terms of
	/// [`is_value_a`] and [`value_into_unchecked`].
	fn try_value_into(value: Value) -> Result<Self, Value>  {
		// if Self::is_value_a(&value) {
		// 	// SAFETY: we just checked that `value` is a valid `Self`.
		// 	Ok(unsafe { Self::value_into_unchecked(value) })
		// } else {
		// 	Err(value)
		// }
		todo!()

	}

	/// Converts a `value` to `Self` without checking `value`'s type.
	///
	/// # Safety
	/// The `value` must be a valid `Self`.
	unsafe fn value_into_unchecked(value: Value) -> Self {
		todo!()
		// debug_assert!(Self::is_value_a(&value), "invalid value given to `value_into_unchecked`: {:?}", value);

		// Allocated::value_into_unchecked(value).into_unchecked()
	}
}

// TODO: allocate pages, and use those, instead of allocating individual pointers.
impl Allocated {
	pub fn new<T>(data: T) -> Self {
		todo!()
	}

	pub fn into_ptr(self) -> *mut () {
		Box::into_raw(Box::new(self)) as *mut ()
	}

	pub fn is_alloc_a<T>(&self) -> bool {
		// T::is_alloc_a(self )
		// self.inner().
		false
	}

	pub unsafe fn from_ptr_ref<'a>(pointer: *const ()) -> &'a Self {
		&*(pointer as *const Self)
	}

	pub unsafe fn from_ptr_mut<'a>(pointer: *mut ()) -> &'a mut Self {
		&mut *(pointer as *mut Self)
	}

	pub unsafe fn from_ptr(ptr: *mut ()) -> Self {
		*Box::from_raw(ptr as *mut Self)
	}

	pub unsafe fn into_unchecked<T>(self) -> T {
		todo!()
	}
}

const ALLOC_MASK: u64  = 0b00000111;
const ALLOC_TAG: u64   = 0b00000000;
const ALLOC_SHIFT: u64 = 0b00000000;

unsafe impl QuestValue for Allocated {
	fn into_value(self) -> Value {
		// SAFETY: This is the definition of a valid pointer.
		unsafe {
			Value::from_bits_unchecked(((self.into_ptr() as u64) << ALLOC_SHIFT) | ALLOC_TAG)
		}
	}

	fn is_value_a(value: &Value) -> bool {
		value.bits() != 0 && (value.bits() & ALLOC_MASK) == ALLOC_TAG
	}

	unsafe fn value_into_unchecked(value: Value) -> Self {
		debug_assert!(value.is_a::<Self>());

		Self::from_ptr(value.bits() as *mut ())
	}
}

unsafe impl QuestValueRef for Allocated {
	unsafe fn value_as_ref_unchecked(value: &Value) -> &Self {
		debug_assert!(value.is_a::<Self>());

		Self::from_ptr_ref(value.bits() as *const ())
	}

	unsafe fn value_as_mut_unchecked(value: &mut Value) -> &mut Self {
		debug_assert!(value.is_a::<Self>());

		Self::from_ptr_mut(value.bits() as *mut ())
	}
}

impl Display for Allocated {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		// std::fmt::Debug::fmt(self, f)
		todo!()
	}
}

impl Debug for Allocated {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		// std::fmt::Debug::fmt(self, f)
		todo!()
	}
}

impl Allocated {
	pub fn try_clone(&self) -> crate::Result<Self> {
		todo!()
	}

	pub fn try_eq(&self, rhs: &Self) -> crate::Result<bool> {
		todo!()
	}
}

impl Drop for Allocated {
	fn drop(&mut self) {
		todo!();
	}
}

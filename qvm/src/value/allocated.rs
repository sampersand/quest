mod text;
mod list;
mod object;

pub use text::*;
pub use list::*;
pub use object::*;

use crate::Literal;
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
	list: ManuallyDrop<List>,
	map: (),
	class: (),
	object: ManuallyDrop<Object>,
}

const TYPE_FLAG_MASK: u64 = 0b11111111;
const FLAG_CLASS_MASK: u64 =   0b00111111;
const FLAG_CLASS_OBJECT: u64 = 0b00000000;
const FLAG_CLASS_BIGNUM: u64 = 0b00000001;
const FLAG_CLASS_REGEX: u64 =  0b00000010;
const FLAG_CLASS_LIST: u64 =   0b00000011;
const FLAG_CLASS_MAP: u64 =    0b00000100;
const FLAG_CLASS_TEXT: u64 =   0b00000101;
const FLAG_CLASS_CLASS: u64 =  0b00000110;
const FLAG_CLASS_CUSTOM: u64 = 0b00000111;

/// A trait that represents objects that are allocated on the heap in Quest.
///
/// External crates should use the [`Object`] struct and [`ObjectType`] trait for custom types; [`ObjectType`] already
/// implements this trait, so you won't need to derive it yourself. The documentation here is used to document internal
/// invariants, but is required to be public due to type bounds.
///
/// # Safety
/// The implementor must ensure that:
/// - Their [`into_alloc()`] will produce a unique [`Allocated`], for which no other type's [`is_alloc_a`] will return
///   `true`.
/// - [`is_alloc_a()`] will always return `true` if the `alloc` was constructed via [`Self::into_alloc`] and `false` 
///   otherwise.
/// - The "`try_`" functions (ie [`try_alloc_into`], [`try_alloc_as_ref`], and [`try_alloc_as_mut`]) should only
///   return an `Ok(Self)` or `Some(self)` if the provided `alloc` was constructed via [`Self::into_alloc`].
/// - The "`_unchecked`" functions (ie [`alloc_into_unchecked`], [`alloc_as_ref_unchecked`],
///   [`alloc_as_mut_unchecked`])'s safety invariants should be satisfied.
///
/// If left unchanged, the default implementation of [`AllocatedType`] does all this correctly.
pub unsafe trait AllocatedType : Debug + Sized {
	/// Converts `self` into an [`Allocated`].
	fn into_alloc(self) -> Allocated;

	/// Checks to see if `alloc` is a `Self`.
	///
	/// See the safety on the trait itself for requirements.
	fn is_alloc_a(alloc: &Allocated) -> bool;

	/// Attempts to convert the `alloc` into a `Self`, returning `Err(alloc)` if it can't.
	fn try_alloc_into(alloc: Allocated) -> Result<Self, Allocated> {
		if Self::is_alloc_a(&alloc) {
			// SAFETY: As long as the trait's implemented properly, we know `alloc` is a `Self`.
			Ok(unsafe { Self::alloc_into_unchecked(alloc) })
		} else {
			Err(alloc)
		}
	}

	/// Converts an `alloc` into `Self`, without verifying that `alloc` is a `Self`.
	///
	/// # Safety
	/// The caller must ensure that `alloc` is a valid `Self`. See [`try_alloc_into`] for a safe version.
	unsafe fn alloc_into_unchecked(alloc: Allocated) -> Self;

	/// Tries to convert an `alloc` reference to a `Self` reference, returning `None` if `alloc` isn't a `Self`.
	fn try_alloc_as_ref(alloc: &Allocated) -> Option<&Self> {
		if Self::is_alloc_a(alloc) {
			// SAFETY: As long as the trait's implemented properly, we know `alloc` is a `Self`.
			Some(unsafe { Self::alloc_as_ref_unchecked(alloc) })
		} else {
			None
		}
	}

	/// Converts an `alloc` reference into a `Self` reference, without verifying that `alloc` is a `Self`.
	///
	/// # Safety
	/// The caller must ensure that `alloc` is a valid `Self`. See [`try_alloc_as_ref`] for a safe version.
	unsafe fn alloc_as_ref_unchecked(alloc: &Allocated) -> &Self;


	/// Tries to convert a mutable `alloc` reference to a mutable `Self` reference, returning `None` if `alloc` isn't a
	/// `Self`.
	fn try_alloc_as_mut(alloc: &mut Allocated) -> Option<&mut Self> {
		if Self::is_alloc_a(alloc) {
			// SAFETY: As long as the trait's implemented properly, we know `alloc` is a `Self`.
			Some(unsafe { Self::alloc_as_mut_unchecked(alloc) })
		} else {
			None
		}
	}

	/// Converts a mutable `alloc` reference into a mutable `Self` reference, without verifying that `alloc` is a `Self`.
	///
	/// # Safety
	/// The caller must ensure that `alloc` is a valid `Self`. See [`try_alloc_as_mut`] for a safe version.
	unsafe fn alloc_as_mut_unchecked(alloc: &mut Allocated) -> &mut Self;
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

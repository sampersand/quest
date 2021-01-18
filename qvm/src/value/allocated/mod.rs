mod allocated;
mod text;
mod list;
mod extern_data;
mod class;
mod bignum;
mod regex;
mod map;
mod paging;

pub(crate) use allocated::Allocated;
pub use class::*;
pub use bignum::*;
pub use regex::*;
pub use map::*;
pub use text::*;
pub use list::*;
pub use extern_data::*;
pub(crate) use paging::initialize;

use crate::Literal;
use crate::value::{Value, ValueType};
use std::fmt::Debug;

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

unsafe impl<T: AllocatedType> ValueType for T {
	fn into_value(self) -> Value {
		self.into_alloc().into_value()
	}

	fn is_value_a(value: &Value) -> bool {
		value.downcast::<Allocated>().map_or(false, Self::is_alloc_a)
	}

	unsafe fn value_into_unchecked(value: Value) -> Self {
		debug_assert!(Self::is_value_a(&value), "invalid value given to `value_into_unchecked`: {:?}", value);

		Allocated::value_into_unchecked(value).into_unchecked()
	}
}

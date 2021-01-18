mod text;
mod list;
mod extern_data;
mod class;
mod bignum;
mod regex;
mod map;
mod paging;

pub use class::*;
pub use bignum::*;
pub use regex::*;
pub use map::*;
pub use text::*;
pub use list::*;
pub use extern_data::*;
pub(crate) use paging::initialize;

use crate::{Literal, ShallowClone, DeepClone};
use crate::value::{Value, ValueType, ValueTypeRef, NamedType};
use std::fmt::{self, Debug, Display, Formatter};
use try_traits::cmp::TryPartialEq;
use std::ptr::NonNull;


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
/// - The "`_unchecked`" functions (ie [`alloc_as_ref_unchecked`] and
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
		value.downcast_copy::<Allocated>().map_or(false, |t| Self::is_alloc_a(&t))
	}

	unsafe fn value_into_unchecked(value: Value) -> Self {
		debug_assert!(Self::is_value_a(&value), "invalid value given to `value_into_unchecked`: {:?}", value);

		// Allocated::value_into_unchecked(value).clone()
		todo!()
	}
}

#[doc(hidden)]
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Allocated(NonNull<AllocData>);

#[repr(align(8))]
struct AllocData {
	flags: u8,
	data: AllocType
}

#[repr(u8)]
enum AllocType {
	Text(Text),
	BigNum(BigNum),
	Regex(Regex),
	List(List),
	Map(Map),
	Class(Class),
	Extern(ExternData)
}

// TODO: allocate pages, and use those, instead of allocating individual pointers.
impl Allocated {
	fn new(data: AllocType) -> Self {
		let mut ptr = paging::allocate();

		unsafe {
			ptr.as_ptr().write(AllocData { flags: 0, data });
		}

		Self(ptr)
	}

	pub fn typename(&self) -> &'static str {
		match self.inner().data {
			AllocType::Text(_) => Text::typename(),
			AllocType::BigNum(_) => BigNum::typename(),
			AllocType::Regex(_) => Regex::typename(),
			AllocType::List(_) => List::typename(),
			AllocType::Map(_) => Map::typename(),
			AllocType::Class(_) => Class::typename(),
			AllocType::Extern(ref externdata) => externdata.typename()
		}
	}
}

impl Allocated {
	fn inner(&self) -> &AllocData {
		// SAFETY: All `Allocated`s point to valid objects by definition,
		// so we're able to dereferenec them
		unsafe {
			self.0.as_ref()
		}
	}

	fn inner_mut(&mut self) -> &mut AllocData {
		// SAFETY: All `Allocated`s point to valid objects by definition,
		// so we're able to dereferenec them
		unsafe {
			self.0.as_mut()
		}
	}
}

const ALLOC_TAG: u64   = 0b0000;
const ALLOC_MASK: u64  = 0b0111;
const ALLOC_SHIFT: u64 = 0;

unsafe impl ValueType for Allocated {
	fn into_value(self) -> Value {
		// SAFETY: This is the definition of a valid pointer.
		unsafe {
			Value::from_bits_unchecked(((self.0.as_ptr() as u64) << ALLOC_SHIFT) | ALLOC_TAG)
		}
	}

	fn is_value_a(value: &Value) -> bool {
		value.bits() != 0 && (value.bits() & ALLOC_MASK) == ALLOC_TAG
	}

	unsafe fn value_into_unchecked(value: Value) -> Self {
		debug_assert!(value.is_a::<Self>());
		debug_assert_ne!(value.bits(), 0);

		Self(NonNull::new_unchecked(value.bits() as *mut AllocData))
	}
}

impl Display for Allocated {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		// match self.data {
		// 	AllocType::Text(text) => Display::fmt(&text, f),
		// 	AllocType::BigNum(bignum) => Display::fmt(&bignum, f),
		// 	AllocType::Regex(regex) => Display::fmt(&regex, f),
		// 	AllocType::List(list) => Display::fmt(&list, f),
		// 	AllocType::Map(map) => Display::fmt(&map, f),
		// 	AllocType::Class(class) => Display::fmt(&class, f),
		// 	AllocType::Extern(externdata) => Display::fmt(&externdata, f)
		// }
		todo!();
	}
}

impl Debug for Allocated {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self.inner().data {
			AllocType::Text(ref text) => Debug::fmt(text, f),
			AllocType::BigNum(ref bignum) => Debug::fmt(bignum, f),
			AllocType::Regex(ref regex) => Debug::fmt(regex, f),
			AllocType::List(ref list) => Debug::fmt(list, f),
			AllocType::Map(ref map) => Debug::fmt(map, f),
			AllocType::Class(ref class) => Debug::fmt(class, f),
			AllocType::Extern(ref externdata) => Debug::fmt(externdata, f)
		}
	}
}

impl DeepClone for Allocated {
	fn deep_clone(&self) -> crate::Result<Self> {
		debug_assert_eq!(self.inner().flags, 0, "todo: nonzero flags when cloning");

		let data =
			match self.inner().data {
				AllocType::Text(ref text) => AllocType::Text(text.clone()),
				AllocType::BigNum(ref bignum) => AllocType::BigNum(bignum.clone()),
				AllocType::Regex(ref regex) => AllocType::Regex(regex.clone()),
				AllocType::List(ref list) => AllocType::List(list.shallow_clone()?),
				AllocType::Map(ref map) => AllocType::Map(map.clone()),
				AllocType::Class(ref class) => AllocType::Class(class.clone()),
				AllocType::Extern(ref externdata) => AllocType::Extern(externdata.deep_clone()?),
			};

		Ok(Self::new(data))
	}
}

impl ShallowClone for Allocated {
	fn shallow_clone(&self) -> crate::Result<Self> {
		debug_assert_eq!(self.inner().flags, 0, "todo: nonzero flags when cloning");

		let data =
			match self.inner().data {
				AllocType::Text(ref text) => AllocType::Text(text.clone()),
				AllocType::BigNum(ref bignum) => AllocType::BigNum(bignum.clone()),
				AllocType::Regex(ref regex) => AllocType::Regex(regex.clone()),
				AllocType::List(ref list) => AllocType::List(list.shallow_clone()?),
				AllocType::Map(ref map) => AllocType::Map(map.clone()),
				AllocType::Class(ref class) => AllocType::Class(class.clone()),
				AllocType::Extern(ref externdata) => AllocType::Extern(externdata.shallow_clone()?),
			};

		Ok(Self::new(data))
	}
}

impl TryPartialEq for Allocated {
	type Error = crate::Error;

	fn try_eq(&self, rhs: &Self) -> crate::Result<bool> {	
		todo!()
	}
}

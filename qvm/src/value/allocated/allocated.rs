use super::*;
use crate::{Literal, ShallowClone, DeepClone};
use crate::value::{Value, ValueType, ValueTypeRef, NamedType};
use std::fmt::{self, Debug, Display, Formatter};
use std::mem::ManuallyDrop;
use std::sync::Arc;
use try_traits::cmp::TryPartialEq;

#[doc(hidden)]
#[repr(transparent)]
pub struct Allocated(Arc<Inner>);

#[repr(align(8))]
struct Inner {
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
	ExternData(ExternData)
}

// TODO: allocate pages, and use those, instead of allocating individual pointers.
impl Allocated {
	pub(super) fn new<T: AllocatedType>(data: T) -> Self {
		data.into_alloc()
	}

	pub fn typename(&self) -> &'static str {
		match self.0.data {
			AllocType::Text(_) => Text::typename(),
			AllocType::BigNum(_) => BigNum::typename(),
			AllocType::Regex(_) => Regex::typename(),
			AllocType::List(_) => List::typename(),
			AllocType::Map(_) => Map::typename(),
			AllocType::Class(_) => Class::typename(),
			AllocType::ExternData(ref externdata) => externdata.typename()
		}
	}
}

impl Clone for Allocated {
	fn clone(&self) -> Self {
		todo!()
	}
}

impl Allocated {
	pub(super) fn is_alloc_a<T: AllocatedType>(&self) -> bool {
		T::is_alloc_a(self)
	}

	pub unsafe fn from_ptr_ref<'a>(pointer: *const ()) -> &'a Self {
		&*(pointer as *const Self)
	}

	pub unsafe fn from_ptr_mut<'a>(pointer: *mut ()) -> &'a mut Self {
		&mut *(pointer as *mut Self)
	}

	pub unsafe fn from_ptr(ptr: *mut ()) -> Self {
		// *Arc::from_raw(ptr as *mut Self)
		todo!()
	}

	pub unsafe fn into_unchecked<T>(self) -> T {
		todo!()
	}
}

const ALLOC_TAG: u64   = 0b0000;
const ALLOC_MASK: u64  = 0b0111;
const ALLOC_SHIFT: u64 = 0;

unsafe impl ValueType for Allocated {
	fn into_value(self) -> Value {
		// SAFETY: This is the definition of a valid pointer.
		unsafe {
			Value::from_bits_unchecked(((Arc::into_raw(self.0) as u64) << ALLOC_SHIFT) | ALLOC_TAG)
		}
	}

	fn is_value_a(value: &Value) -> bool {
		value.bits() != 0 && (value.bits() & ALLOC_MASK) == ALLOC_TAG
	}

	unsafe fn value_into_unchecked(value: Value) -> Self {
		debug_assert!(value.is_a::<Self>());

		Self(Arc::from_raw(value.bits() as *mut Inner))
	}
}

unsafe impl ValueTypeRef for Allocated {
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
		// match self.data {
		// 	AllocType::Text(text) => Display::fmt(&text, f),
		// 	AllocType::BigNum(bignum) => Display::fmt(&bignum, f),
		// 	AllocType::Regex(regex) => Display::fmt(&regex, f),
		// 	AllocType::List(list) => Display::fmt(&list, f),
		// 	AllocType::Map(map) => Display::fmt(&map, f),
		// 	AllocType::Class(class) => Display::fmt(&class, f),
		// 	AllocType::ExternData(externdata) => Display::fmt(&externdata, f)
		// }
		todo!();
	}
}

impl Debug for Allocated {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self.0.data {
			AllocType::Text(ref text) => Debug::fmt(text, f),
			AllocType::BigNum(ref bignum) => Debug::fmt(bignum, f),
			AllocType::Regex(ref regex) => Debug::fmt(regex, f),
			AllocType::List(ref list) => Debug::fmt(list, f),
			AllocType::Map(ref map) => Debug::fmt(map, f),
			AllocType::Class(ref class) => Debug::fmt(class, f),
			AllocType::ExternData(ref externdata) => Debug::fmt(externdata, f)
		}
	}
}

impl DeepClone for Allocated {
	fn deep_clone(&self) -> crate::Result<Self> {
		debug_assert_eq!(self.0.flags, 0, "todo: nonzero flags when cloning");

		let data =
			match self.0.data {
				AllocType::Text(ref text) => AllocType::Text(text.clone()),
				AllocType::BigNum(ref bignum) => AllocType::BigNum(bignum.clone()),
				AllocType::Regex(ref regex) => AllocType::Regex(regex.clone()),
				AllocType::List(ref list) => AllocType::List(list.shallow_clone()?),
				AllocType::Map(ref map) => AllocType::Map(map.clone()),
				AllocType::Class(ref class) => AllocType::Class(class.clone()),
				AllocType::ExternData(ref externdata) => AllocType::ExternData(externdata.deep_clone()?),
			};

		Ok(Self(Arc::new(Inner { flags: 0, data })))
	}
}

impl ShallowClone for Allocated {
	fn shallow_clone(&self) -> crate::Result<Self> {
		debug_assert_eq!(self.0.flags, 0, "todo: nonzero flags when cloning");

		let data =
			match self.0.data {
				AllocType::Text(ref text) => AllocType::Text(text.clone()),
				AllocType::BigNum(ref bignum) => AllocType::BigNum(bignum.clone()),
				AllocType::Regex(ref regex) => AllocType::Regex(regex.clone()),
				AllocType::List(ref list) => AllocType::List(list.shallow_clone()?),
				AllocType::Map(ref map) => AllocType::Map(map.clone()),
				AllocType::Class(ref class) => AllocType::Class(class.clone()),
				AllocType::ExternData(ref externdata) => AllocType::ExternData(externdata.shallow_clone()?),
			};

		Ok(Self(Arc::new(Inner { flags: 0, data })))
	}
}

impl TryPartialEq for Allocated {
	type Error = crate::Error;

	fn try_eq(&self, rhs: &Self) -> crate::Result<bool> {
		
		todo!()
	}
}

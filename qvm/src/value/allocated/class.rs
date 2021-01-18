use super::{AllocatedType, AllocType, Allocated};
use crate::value::{Literal, Value, ValueType, NamedType};
use std::fmt::{self, Debug, Formatter};

/// A Class is really just an instance of an object, but with
/// the data value being `*mut ()`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct Class {
	name: &'static str,
}

impl Debug for Class {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_tuple("Class").field(&self.name).finish()
	}
}

impl Class {
	#[inline]
	pub fn new(name: &'static str) -> Self {
		Self { name }
	}
}

impl NamedType for Class {
	#[inline(always)]
	fn typename() -> &'static str {
		"Class"
	}
}

unsafe impl AllocatedType for Class {
	#[inline]
	fn into_alloc(self) -> Allocated {
		Allocated::new(AllocType::Class(self))
	}

	#[inline]
	fn is_alloc_a(alloc: &Allocated) -> bool {
		matches!(alloc.inner().data, AllocType::Class(_))
	}

	#[inline]
	unsafe fn alloc_as_ref_unchecked(alloc: &Allocated) -> &Self {
		debug_assert!(Self::is_alloc_a(alloc), "invalid value given: {:#?}", alloc);

		if let AllocType::Class(ref class) = alloc.inner().data {
			class
		} else {
			std::hint::unreachable_unchecked()
		}
	}

	#[inline]
	unsafe fn alloc_as_mut_unchecked(alloc: &mut Allocated) -> &mut Self {
		debug_assert!(Self::is_alloc_a(alloc), "invalid value given: {:#?}", alloc);

		if let AllocType::Class(ref mut class) = alloc.inner_mut().data {
			class
		} else {
			std::hint::unreachable_unchecked()
		}
	}
}

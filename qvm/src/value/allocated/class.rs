use super::{AllocatedType, AllocType, Allocated};
use crate::value::{Literal, Value, ValueType, NamedType};
use std::fmt::{self, Debug, Formatter};

/// A Class is really just an instance of an object, but with
/// the data value being `*mut ()`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Hash, Named)]
#[quest(crate_name="crate")]
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

impl_allocated_type!(for Class);
impl_allocated_value_type_ref!(for Class);

impl crate::ShallowClone for Class {
	fn shallow_clone(&self) -> crate::Result<Self> {
		Ok(self.clone())
	}
}

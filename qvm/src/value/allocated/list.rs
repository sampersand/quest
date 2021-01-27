use crate::{Value, ShallowClone};
use crate::value::{NamedType, HasAttrs, ValueType, ValueTypeRef};
use crate::value::allocated::{Allocated, AllocatedType, AllocType};

/// A list of [`Values`].
#[derive(Debug, Clone, Named)]
#[quest(crate_name="crate")]
pub struct List(Vec<Value>);

impl List {
	/// Creates a new list from the given iterable.
	#[inline]
	pub fn new<I: IntoIterator<Item=Value>>(iter: I) -> Self {
		Self(iter.into_iter().collect())
	}
}

// impl From<Vec<Value>> for List {
// 	#[inline]
// 	fn from(list: Vec<Value>) -> Self {
// 		Self(list)
// 	}
// }

// impl From<List> for Vec<Value> {
// 	#[inline]
// 	fn from(list: List) -> Self {
// 		list.0
// 	}
// }

// impl AsRef<[Value]> for Vec<Value> {
// 	fn as_ref(&self) -> &[Value] {
// 		self.0.as_ref()
// 	}
// }

// impl AsMut<[Value]> for Vec<Value> {
// 	fn as_ref(&self) -> &[Value] {
// 		self.0.as_ref()
// 	}
// }

impl std::iter::FromIterator<Value> for List {
	#[inline]
	fn from_iter<I: IntoIterator<Item=Value>>(iter: I) -> Self {
		Self::new(iter)
	}
}

impl ShallowClone for List {
	fn shallow_clone(&self) -> crate::Result<Self> {
		self.0.iter()
			.map(ShallowClone::shallow_clone)
			.collect::<crate::Result<_>>()
			.map(Self)
	}
}

impl crate::TryPartialEq for List {
	fn try_eq(&self, rhs: &Self) -> crate::Result<bool> {
		if self.0.len() != rhs.0.len() {
			return Ok(false);
		}

		for (l, r) in self.0.iter().zip(rhs.0.iter()) {
			if !l.try_eq(r)? {
				return Ok(false);
			}
		}

		Ok(true)
	}
}

impl_allocated_type!(for List);
impl_allocated_value_type_ref!(for List);

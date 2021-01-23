use crate::value::NamedType;
use crate::{Value, ShallowClone};

/// A list of [`Values`].
#[derive(Debug, Clone)]
pub struct List(Vec<Value>);

impl ShallowClone for List {
	fn shallow_clone(&self) -> crate::Result<Self> {
		self.0.iter()
			.map(ShallowClone::shallow_clone)
			.collect::<crate::Result<_>>()
			.map(Self)
	}
}

impl NamedType for List {
	const TYPENAME: &'static str = "List";
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

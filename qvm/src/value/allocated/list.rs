use crate::value::NamedType;
use crate::{Value, ShallowClone};

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
	#[inline(always)]
	fn typename() -> &'static str {
		"List"
	}
}

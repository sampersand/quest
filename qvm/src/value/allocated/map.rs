use crate::value::NamedType;

/// A Map of [`Values`]to [`Values`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct Map {
	// todo
}

impl NamedType for Map {
	#[inline(always)]
	fn typename() -> &'static str {
		"Map"
	}
}

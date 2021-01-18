use crate::value::NamedType;

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

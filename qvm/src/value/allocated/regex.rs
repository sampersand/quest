use crate::value::NamedType;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct Regex {
	// todo
}

impl NamedType for Regex {
	#[inline(always)]
	fn typename() -> &'static str {
		"Regex"
	}
}

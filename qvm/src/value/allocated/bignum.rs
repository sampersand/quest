use crate::value::NamedType;

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub struct BigNum {
	// todo
}

impl NamedType for BigNum {
	#[inline(always)]
	fn typename() -> &'static str {
		"Number"
	}
}

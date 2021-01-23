use crate::value::NamedType;

/// A heap-allocated number that can represent any numeric value Quest supports.
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub struct BigNum {
	// todo
}

impl NamedType for BigNum {
	const TYPENAME: &'static str = "Number";
}

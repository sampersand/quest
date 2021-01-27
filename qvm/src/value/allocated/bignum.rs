use crate::value::NamedType;

/// A heap-allocated number that can represent any numeric value Quest supports.
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Named)]
#[quest(crate_name="crate", name="Number")]
pub struct BigNum {
	// todo
}

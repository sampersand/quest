use crate::value::NamedType;

/// The type used to represent text.
///
/// Text is just an arbitrary sequence of bytes, and doesn't have to be utf8-compatible or null terminated.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Named)]
#[quest(crate_name="crate")]
pub struct Text {
	data: Vec<u8>
}

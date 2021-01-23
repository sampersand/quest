use crate::value::NamedType;

/// The type used to represent text.
///
/// Text is just an arbitrary sequence of bytes, and doesn't have to be utf8-compatible or null terminated.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct Text {
	data: Vec<u8>
}

impl NamedType for Text {
	const TYPENAME: &'static str = "Text";
}

// impl ValueType for Text {
	
// }

use crate::value::NamedType;

/// The type used to represent text.
///
/// Text is just an arbitrary sequence of bytes, and doesn't have to be utf8-compatible or null terminated.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Named)]
#[quest(crate_name="crate")]
pub struct Text {
	data: Vec<u8>
}

impl crate::ShallowClone for Text {
	fn shallow_clone(&self) -> crate::Result<Self> {
		Ok(self.clone())
	}
}

impl crate::DeepClone for Text {
	fn deep_clone(&self) -> crate::Result<Self> {
		Ok(self.clone())
	}
}


impl_allocated_type!(for Text);
impl_allocated_value_type_ref!(for Text);

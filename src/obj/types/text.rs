use crate::obj::{Mapping, Object, types::ObjectType};
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
enum TextEnum {
	Owned(String),
	Static(&'static str)
}

#[derive(Clone, PartialEq, Eq)]
pub struct Text(TextEnum);

impl Debug for Text {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			write!(f, "Text({:?})", self.as_ref())
		} else {
			Debug::fmt(&self.as_ref(), f)
		}
	}
}

impl From<&'static str> for Text {
	fn from(txt: &'static str) -> Text {
		Text(TextEnum::Static(txt)).into()
	}
}

impl From<String> for Text {
	fn from(txt: String) -> Text {
		Text(TextEnum::Owned(txt)).into()
	}
}

impl From<String> for crate::obj::Object {
	fn from(txt: String) -> Self {
		Text(TextEnum::Owned(txt)).into()
	}
}

impl From<&'static str> for crate::obj::Object {
	fn from(txt: &'static str) -> Self {
		Text(TextEnum::Static(txt)).into()
	}
}



impl AsRef<str> for Text {
	fn as_ref(&self) -> &str {
		match self.0 {
			TextEnum::Owned(ref txt) => txt.as_ref(),
			TextEnum::Static(ref txt) => txt.as_ref()
		}
	}
}

impl_object_type!{for Text, super::Basic;
	// "==" => (|args| todo!())//	Ok(args[0].as_text().map(|x| x == this.as_text().unwrap()).unwrap_or(false).into()))
}

use crate::obj::{DataEnum, Mapping, Object, types::ObjectType};
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};

type InternalRepr = String;

#[derive(Clone, PartialEq, Eq)]
pub struct Text(InternalRepr);

impl Text {
	pub fn new<T: Into<InternalRepr>>(num: T) -> Self {
		Text(num.into())
	}
}

impl Debug for Text {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			write!(f, "Text({:?})", self.0)
		} else {
			Debug::fmt(&self.0, f)
		}
	}
}

impl From<InternalRepr> for crate::obj::Object {
	fn from(txt: InternalRepr) -> Self {
		Text::new(txt).into()
	}
}

impl From<&'static str> for crate::obj::Object {
	fn from(txt: &'static str) -> Self {
		Text::new(txt.to_string()).into()
	}
}

impl Object {
	pub fn call_into_text(&self) -> Result<InternalRepr, Object> {
		self.call("@text", &[])?.into_text().ok_or_else(|| "not a text".into())
	}

	pub fn as_text(&self) -> Option<&InternalRepr> {
		if let DataEnum::Text(ref t) = self.0.data {
			Some(t.as_ref())
		} else {
			None
		}
	}

	pub fn into_text(&self) -> Option<InternalRepr> {
		self.as_text().map(Clone::clone)
	}
}



impl AsRef<str> for Text {
	fn as_ref(&self) -> &str {
		self.0.as_ref()
	}
}

impl AsRef<InternalRepr> for Text {
	fn as_ref(&self) -> &InternalRepr {
		&self.0
	}
}

impl From<Text> for DataEnum {
	fn from(this: Text) -> DataEnum {
		DataEnum::Text(this)
	}
}

impl ObjectType for Text {
	fn mapping() -> Arc<RwLock<Mapping>> {
		// use std::sync::Once;
		// static MAPPING: Mapping = {
		Arc::new(RwLock::new(Mapping::new(None)))
		// m.insert()
		// };

		// MAPPING
	}
}
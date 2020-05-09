use crate::obj::{DataEnum, Mapping, Object, types::ObjectType};
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};

type InternalRepr = bool;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Boolean(InternalRepr);

impl Boolean {
	pub fn new<T: Into<InternalRepr>>(num: T) -> Self {
		Boolean(num.into())
	}
}

impl From<Boolean> for DataEnum {
	fn from(this: Boolean) -> DataEnum {
		DataEnum::Boolean(this)
	}
}

impl AsRef<InternalRepr> for Boolean {
	fn as_ref(&self) -> &InternalRepr {
		&self.0
	}
}

impl Object {
	pub fn call_into_bool(&self) -> Result<InternalRepr, Object> {
		self.call("@bool", &[])?.try_into_bool()
	}

	pub fn try_into_bool(&self) -> Result<InternalRepr, Object> {
		self.try_as_bool().map(Clone::clone)
	}

	pub fn into_bool(&self) -> Option<InternalRepr> {
		self.as_bool().map(Clone::clone)
	}

	pub fn try_as_bool(&self) -> Result<&InternalRepr, Object> {
		self.as_bool().ok_or_else(|| "not a bool".to_owned().into())
	}

	pub fn as_bool(&self) -> Option<&InternalRepr> {
		if let DataEnum::Boolean(ref t) = self.0.data {
			Some(t.as_ref())
		} else {
			None
		}
	}

}

impl From<InternalRepr> for Object {
	fn from(n: InternalRepr) -> Object {
		Boolean::new(n).into()
	}
}

impl Debug for Boolean {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			write!(f, "Boolean({:?})", self.0)
		} else {
			Debug::fmt(&self.0, f)
		}
	}
}


impl ObjectType for Boolean {
	fn mapping() -> Arc<RwLock<Mapping>> {
		// use std::sync::Once;
		// static MAPPING: Mapping = {
		let mut m = Mapping::new(None);
		m.insert(
			"@bool".to_owned().into(),
			super::RustFn::new("@bool", (|x, _| {
				println!("Boolean::@bool({:?})", x);
				x.call("clone", &[])
			})).into());

		m.insert(
			"clone".to_owned().into(),
			super::RustFn::new("clone", (|x, _| Ok((*x.try_as_bool()?).into()))).into());
		// m.insert(
		// 	super::Text::new("+").into(),
		// 	super::RustFn::new("+",
		// 		(|x, y| Ok(x.clone()))
		// 	).into()
		// );
		Arc::new(RwLock::new(m))
		// m.insert()
		// };

		// MAPPING
	}
}

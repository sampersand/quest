use crate::obj::{DataEnum, Mapping, types::ObjectType};
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};

type InternalRepr = i64;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Number(InternalRepr);

impl Number {
	pub fn new<T: Into<InternalRepr>>(num: T) -> Self {
		Number(num.into())
	}
}

impl From<Number> for DataEnum {
	fn from(this: Number) -> DataEnum {
		DataEnum::Number(this)
	}
}

impl From<InternalRepr> for crate::obj::Object {
	fn from(n: InternalRepr) -> crate::obj::Object {
		Number::new(n).into()
	}
}

impl Debug for Number {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			write!(f, "Number({:?})", self.0)
		} else {
			Debug::fmt(&self.0, f)
		}
	}
}


impl ObjectType for Number {
	fn mapping() -> Arc<RwLock<Mapping>> {
		// use std::sync::Once;
		// static MAPPING: Mapping = {
		use crate::obj::Object;
		let mut m = Mapping::new(None);
		m.insert(
			super::Text::new("+").into(),
			super::RustFn::new("+",
				(|x, y| Ok(x.clone()))
			).into()
		);
		Arc::new(RwLock::new(m))
		// m.insert()
		// };

		// MAPPING
	}
}

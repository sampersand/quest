use std::convert::TryFrom;

use crate::obj::{Object, Result, types::{self, RustFn}};

#[derive(Debug, Clone)]
pub enum Value {
	Object(Object),
	RustFn(RustFn),
}

impl From<Object> for Value {
	fn from(object: Object) -> Self {
		Value::Object(object)
	}
}

impl From<RustFn> for Value {
	fn from(rustfn: RustFn) -> Self {
		Value::RustFn(rustfn)
	}
}

impl From<Value> for Object {
	fn from(value: Value) -> Self {
		match value {
			Value::Object(object) => object,
			Value::RustFn(rustfn) => rustfn.into()
		}
	}
}

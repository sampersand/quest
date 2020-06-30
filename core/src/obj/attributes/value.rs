use crate::types::RustFn;
use crate::{Object, Args, Result};

#[derive(Debug, Clone)]
pub enum Value {
	RustFn(RustFn),
	Object(Object)
}

impl Value {
	pub fn call(&self, owner: &Object, args: Args) -> Result<Object> {
		match self {
			Value::RustFn(rustfn) => rustfn.call(owner, args),
			Value::Object(object) => {
				let bound_attr = Object::new(crate::types::BoundFunction);
				bound_attr.set_attr_lit("__bound_object_owner__", owner.clone());
				bound_attr.set_attr_lit("__bound_object__", object.clone());
				bound_attr.call_attr_lit("()", args)
			}
		}
	}
}

impl From<Value> for Object {
	#[inline]
	fn from(val: Value) -> Self {
		/// we should have a COW here in case the rustfn is modified by the user.
		match val {
			Value::RustFn(rustfn) => rustfn.into(),
			Value::Object(obj) => obj
		}
	}
}

impl From<RustFn> for Value {
	#[inline]
	fn from(rustfn: RustFn) -> Self {
		Value::RustFn(rustfn)
	}
}

impl From<Object> for Value {
	#[inline]
	fn from(obj: Object) -> Self {
		Value::Object(obj)
	}
}


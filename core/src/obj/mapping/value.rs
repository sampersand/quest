use crate::{Object, ToObject, Result, Args, types::RustFn};

#[derive(Debug, Clone)]
pub enum Value {
	Object(Object),
	RustFn(RustFn),
}

impl Value {
	pub fn call(&self, owner: &Object, args: Args) -> Result<Object> {

		match self {
			Value::RustFn(rustfn) => rustfn.call(owner, args),
			Value::Object(object) => {
				let bound_attr = Object::new(crate::types::BoundFunction);
				bound_attr.set_attr("__bound_object_owner__", owner.clone())?;
				bound_attr.set_attr("__bound_object__", object.clone())?;
				bound_attr.call_attr("()", args)
			}
		}
	}

}

impl From<Object> for Value {
	fn from(object: Object) -> Self {
		Value::Object(object)
	}
}

impl From<Vec<Object>> for Value {
	fn from(list: Vec<Object>) -> Self {
		Value::Object(list.into())
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

impl ToObject for Value {
	fn to_object(&self) -> Object {
		self.clone().into()
	}
}
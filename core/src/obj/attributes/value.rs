use crate::types::RustFn;
use crate::{Object, Args, Result, Literal};

/// A value.
///
/// This exists because it is _so_ much faster to call `RustFn`s directly than to try to downcast.
#[derive(Debug, Clone)]
pub enum Value {
	RustFn(RustFn),
	Object(Object)
}

impl Value {
	/// Calls this value, returning the result.
	pub fn call<'o>(&self, owner: &'o Object, args: Args<'_, 'o>) -> Result<Object> {
		use std::borrow::Cow;
		match self {
			Value::RustFn(rustfn) => rustfn.call(owner, args),
			Value::Object(object) => {
				let args = 
					match args.into_inner() {
						Cow::Borrowed(slice) => {
							let mut args = Vec::with_capacity(slice.len() + 1);
							args.push(owner);
							args.extend(slice);
							args
						}
						Cow::Owned(mut args) => {
							args.insert(0, owner);
							args
						}
					};

				object.call_attr_lit(&Literal::CALL, args)
			}
		}
	}
}

impl From<Value> for Object {
	fn from(val: Value) -> Self {
		// we should have a COW here in case the rustfn is modified by the user.
		match val {
			Value::RustFn(rustfn) => rustfn.into(),
			Value::Object(obj) => obj,
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
		Self::Object(obj)
	}
}

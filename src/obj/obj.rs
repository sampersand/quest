use crate::obj::{self, Mapping, DataEnum, types::ObjectType};
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};

#[derive(Clone)]
pub struct Object(pub(super) Arc<Internal>);

pub(super) struct Internal {
	mapping: Arc<RwLock<Mapping>>,
	pub(super) data: DataEnum
}

impl Debug for Object {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			f.debug_struct("Object")
				.field("data", &self.0.data)
				.field("mapping", &*self.0.mapping.read().expect("cant read in obj"))
				.finish()
		} else {
			f.debug_tuple("Object")
				.field(&self.0.data)
				.finish()
		}
	}
}

impl Object {
	pub fn new<T: ObjectType>(data: T) -> Self {
		Object(Arc::new(Internal {
			mapping: Arc::new(RwLock::new(Mapping::new(Some(T::mapping())))),
			data: data.into()
		}))
	}

	pub fn get_attr(&self, attr: &Object) -> obj::Result {
		self.0.mapping.read().expect("cannot read").get(attr, self)
	}

	pub fn call_attr(&self, attr: &Object, args: &[&Object]) -> obj::Result {
		if attr.as_text().map(|x| x == "==").unwrap_or(false) {
			if args.is_empty() {
				Err("need at least 1 arg for `==`".into())
			} else {
				Ok((self.0.data == args[0].0.data).into())
			}
		} else {
			self.get_attr(attr)?.call("()", args)
		}
	}

	pub fn call(&self, txt: &'static str, args: &[&Object]) -> obj::Result {
		self.call_attr(&txt.into(), args)
	}
}

impl<T: ObjectType> From<T> for Object {
	fn from(data: T) -> Self {
		Self::new(data)
	}
}









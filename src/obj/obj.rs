use crate::obj::{self, Mapping, DataEnum, types::ObjectType};
use std::sync::{Arc, RwLock, atomic::{self, AtomicUsize}};
use std::fmt::{self, Debug, Formatter};

#[derive(Clone)]
pub struct Object(pub(super) Arc<Internal>);

pub(super) struct Internal {
	mapping: Arc<RwLock<Mapping>>,
	pub(super) id: usize,
	// in the future, might we want this to be reference counted?
	pub(super) data: DataEnum
}

impl Debug for Object {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			f.debug_struct("Object")
				.field("id", &self.0.id)
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
	pub fn new_with_parent<T: Into<DataEnum>>(data: T, parent: Option<Object>) -> Self {
		static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
		Object(Arc::new(Internal {
			id: ID_COUNTER.fetch_add(1, atomic::Ordering::Relaxed),
			mapping: Arc::new(RwLock::new(Mapping::new(parent))),
			data: data.into()
		}))
	}

	pub fn new<T: ObjectType>(data: T) -> Self {
		Object::new_with_parent(data, Some(T::mapping()))
	}

	pub fn get_attr(&self, attr: &Object) -> obj::Result {
		self.0.mapping.read().expect("cannot read").get(attr)
	}

	pub fn set_attr(&self, attr: Object, val: Object) -> obj::Result {
		self.0.mapping.write().expect("cannot write").insert(attr, val)
	}

	pub fn del_attr(&self, attr: &Object) -> obj::Result {
		self.0.mapping.write().expect("cannot write").remove(attr)
	}

	pub fn call_attr(&self, attr: &Object, args: &[&Object]) -> obj::Result {
		// println!("Object::call_attr({:?}, {:?}, {:?})", self, attr, args);
		if let Some(rfn) = self.as_rustfn_obj() {
			if attr.as_text().map(|x| x == "()").unwrap_or(false) {
				return rfn.call(args)
			}
		}

		if attr.as_text().map(|x| x == "==").unwrap_or(false) {
			if args.is_empty() {
				Err("need at least 1 arg for `==`".into())
			} else {
				Ok((self.0.data == args[0].0.data).into())
			}
		} else {
			let mut v = vec![self];
			v.extend_from_slice(args);
			self.get_attr(attr)?.call("()", &v)
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









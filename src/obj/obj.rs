use crate::obj::{self, Mapping, DataEnum, types::{self, ObjectType}};
use std::sync::{Arc, RwLock, atomic::{self, AtomicUsize}};
use std::fmt::{self, Debug, Formatter};
use std::any::{Any, TypeId};

#[derive(Clone)]
pub struct Object(pub(super) Arc<Internal>);

pub(super) struct Internal {
	mapping: Arc<RwLock<Mapping>>,
	pub(super) id: usize,
	pub(super) data: Arc<RwLock<dyn Any>>,
	dbg: fn(&dyn Any, &mut Formatter) -> fmt::Result
}

impl Debug for Object {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		struct DataDebug<'a>(&'a dyn Any, fn(&dyn Any, &mut Formatter) -> fmt::Result);
		impl Debug for DataDebug<'_> {
			fn fmt(&self, f: &mut Formatter) -> fmt::Result {
				(self.1)(self.0, f)
			}
		}

		if f.alternate() {
			f.debug_struct("Object")
				.field("id", &self.0.id)
				.field("data", &DataDebug(&*self.0.data.read().expect("poisoned"), self.0.dbg))
				.field("mapping", &*self.0.mapping.read().expect("cant read in obj"))
				.finish()
		} else {
			f.debug_tuple("Object")
				.field(&DataDebug(&*self.0.data.read().expect("poisoned"), self.0.dbg))
				.finish()
		}
	}
}

impl<T: Any + ObjectType> From<T> for Object {
	fn from(data: T) -> Object {
		Object::new(data)
	}
}
impl Object {
	pub fn new_with_parent<T: Any + Debug>(data: T, parent: Option<Object>) -> Self {
		static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
		Object(Arc::new(Internal {
			id: ID_COUNTER.fetch_add(1, atomic::Ordering::Relaxed),
			mapping: Arc::new(RwLock::new(Mapping::new(parent))),
			data: Arc::new(RwLock::new(data)),
			dbg: (|x, f| <T as Debug>::fmt(x.downcast_ref::<T>().expect("bad val givent to debug"), f))
		}))
	}

	pub fn new<T: ObjectType>(data: T) -> Self {
		Object::new_with_parent(data, Some(T::mapping()))
	}

	pub fn try_downcast_ref<'a, T: Any>(&'a self) -> Result<impl std::ops::Deref<Target = T> + 'a, Object> {
		self.downcast_ref::<T>().ok_or_else(|| types::Text::from(format!("not a {:?}", TypeId::of::<T>())).into())
	}

	pub fn is_type<T: Any>(&self) -> bool {
		self.0.data.read().expect("poison error").is::<T>()
	}

	pub fn downcast_clone<T: Any + Clone>(&self) -> Option<T> {
		self.downcast_ref::<T>().map(|x| x.clone())
	}

	pub fn downcast_ref<'a, T: Any>(&'a self) -> Option<impl std::ops::Deref<Target=T> + 'a> {
		use std::{sync::RwLockReadGuard, marker::PhantomData, ops::{Deref, DerefMut}};
		struct Caster<'a, T>(RwLockReadGuard<'a, dyn Any>, PhantomData<T>);
		impl<'a, T: 'static> Deref for Caster<'a, T> {
			type Target = T;
			fn deref(&self) -> &T {
				self.0.downcast_ref().unwrap()
			}
		}

		let data = self.0.data.read().expect("poison error");
		if data.is::<T>() {
			Some(Caster::<'a, T>(data, PhantomData))
		} else {
			None
		}
	}

	pub fn downcast_mut<'a, T: Any>(&'a self) -> Option<impl std::ops::DerefMut<Target=T> + 'a> {
		use std::{sync::RwLockWriteGuard, marker::PhantomData, ops::{Deref, DerefMut}};
		struct Caster<'a, T>(RwLockWriteGuard<'a, dyn Any>, PhantomData<T>);
		impl<'a, T: 'static> std::ops::Deref for Caster<'a, T> {
			type Target = T;
			fn deref(&self) -> &T {
				self.0.downcast_ref().unwrap()
			}
		}
		impl<'a, T: 'static> DerefMut for Caster<'a, T> {
			fn deref_mut(&mut self) -> &mut T {
				self.0.downcast_mut().unwrap()
			}
		}

		let data = self.0.data.write().expect("poison error");
		if data.is::<T>() {
			Some(Caster::<'a, T>(data, PhantomData))
		} else {
			None
		}
	}

	pub fn get_attr(&self, attr: &Object) -> obj::Result<Object> {
		// println!("Object::get_attr(self={:?}, attr={:?})", self, attr);
		self.0.mapping.read().expect("cannot read").get(attr)
	}

	pub fn set_attr(&self, attr: Object, val: Object) -> obj::Result<Object> {
		self.0.mapping.write().expect("cannot write").insert(attr, val)
	}

	pub fn del_attr(&self, attr: &Object) -> obj::Result<Object> {
		self.0.mapping.write().expect("cannot write").remove(attr)
	}

	pub fn call_attr(&self, attr: &Object, args: &[&Object]) -> obj::Result<Object> {
		// static mut X: usize = 0;
		// unsafe { if X > 100 { panic!("X too big");} X += 1;}
		// println!("Object::call_attr(self={:?}, attr={:?}, args={:?})", self, attr, args);
		if let (Some(rustfn), Some(txt_attr)) = (self.downcast_ref::<types::RustFn>(), attr.downcast_ref::<types::Text>()) {
			if (txt_attr.as_ref() == "()") {
				return rustfn.call(args)
			}
		}

		if let Some(txt_attr) = attr.downcast_ref::<types::Text>() {
			if (txt_attr.as_ref() == "==") {
				if args.is_empty() {
					return Err(types::Text::from("need at least 1 arg for `==`").into())
				} else if let (Some(lhs), Some(rhs)) = (self.downcast_ref::<types::Text>(),
				                                        args[0].downcast_ref::<types::Text>()) {
					return Ok(types::Boolean::from(*lhs == *rhs).into())
				}
			}
		}

		let mut v = vec![self];
		v.extend_from_slice(args);
		self.get_attr(attr)?.call("()", &v)
	}

	pub fn call(&self, txt: &'static str, args: &[&Object]) -> obj::Result<Object> {
		self.call_attr(&txt.into(), args)
	}
}





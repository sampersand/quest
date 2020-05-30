use crate::obj::{self, Mapping, Args, types::{self, ObjectType, rustfn::Binding}};
use std::sync::{Arc, RwLock, atomic::{self, AtomicUsize}};
use std::fmt::{self, Debug, Formatter};
use std::any::{Any, TypeId};

#[derive(Clone)]
pub struct Object(pub(super) Arc<Internal>);

impl Default for Object {
	fn default() -> Self {
		Object::new(types::Null::new())
	}
}

pub(super) struct Internal {
	mapping: Arc<RwLock<Mapping>>,
	id: usize,
	pub(super) data: Arc<RwLock<dyn Any + Send + Sync>>,
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
	pub fn new_with_parent<T: Any + Debug + Send + Sync>(data: T, parent: Option<Object>) -> Self {
		static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
		let id = ID_COUNTER.fetch_add(1, atomic::Ordering::Relaxed);
		Object(Arc::new(Internal {
			id: id,
			mapping: Arc::new(RwLock::new(Mapping::new(parent))),
			data: Arc::new(RwLock::new(data)),
			dbg: (|x, f| <T as Debug>::fmt(x.downcast_ref::<T>().expect("bad val givent to debug"), f))
		}))
	}

	pub fn id(&self) -> usize {
		self.0.id
	}

	pub fn is_identical(&self, rhs: &Object) -> bool {
		Arc::ptr_eq(&self.0, &rhs.0)
	}

	pub fn new<T: ObjectType>(data: T) -> Self {
		Object::new_with_parent(data, Some(T::mapping()))
	}

	pub fn try_downcast_ref<'a, T: Any>(&'a self) -> obj::Result<impl std::ops::Deref<Target = T> + 'a> {
		self.downcast_ref::<T>().ok_or_else(||
			types::Text::from(format!("{:?} is not a {:?}", self, TypeId::of::<T>())).into()
		)
	}

	pub fn is_a<T: Any>(&self) -> bool {
		self.0.data.read().expect("poison error").is::<T>()
	}

	pub fn downcast_clone<T: Any + Clone>(&self) -> Option<T> {
		self.downcast_ref::<T>().map(|x| x.clone())
	}

	pub fn try_downcast_clone<T: Any + Clone>(&self) -> obj::Result<T> {
		self.downcast_ref::<T>().map(|x| x.clone()).ok_or_else(|| types::Text::from(format!("not a {:?}", TypeId::of::<T>())).into())
	}

	pub fn downcast_ref<'a, T: Any>(&'a self) -> Option<impl std::ops::Deref<Target=T> + 'a> {
		use std::{sync::RwLockReadGuard, marker::PhantomData, ops::{Deref, DerefMut}};
		struct Caster<'a, T>(RwLockReadGuard<'a, dyn Any + Send + Sync>, PhantomData<T>);
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
		struct Caster<'a, T>(RwLockWriteGuard<'a, dyn Any + Send + Sync>, PhantomData<T>);
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

	pub fn get_attr(&self, attr: &Object, binding: &Binding) -> obj::Result<Object> {
		self.0.mapping.read().expect("cannot read").get(attr, binding, self)
	}

	pub fn set_attr(&self, attr: Object, val: Object, binding: &Binding) -> obj::Result<Object> {
		self.0.mapping.write().expect("cannot write").insert(attr, val, binding)
	}

	pub fn del_attr(&self, attr: &Object, binding: &Binding) -> obj::Result<Object> {
		self.0.mapping.write().expect("cannot write").remove(attr, binding)
	}


	pub fn call_attr(&self, attr: &Object, mut args: Args) -> obj::Result<Object> {
		if let (Some(rustfn), Some(txt_attr)) = (self.downcast_ref::<types::RustFn>(), attr.downcast_ref::<types::Text>()) {
			if (txt_attr.as_ref() == "()") {
				return rustfn.call(args);
			}
		}

		if let Some(txt_attr) = attr.downcast_ref::<types::Text>() {
			if (txt_attr.as_ref() == "==") {
				if args.as_ref().is_empty() {
					return Err(types::Text::from("need at least 1 arg for `==`").into())
				} else if let (Some(lhs), Some(rhs)) = (self.downcast_ref::<types::Text>(),
				                                        args.get_downcast::<types::Text>(0).ok()) {
					return Ok(types::Boolean::from(*lhs == *rhs).into())
				}
			}
		}

		args.add_this(self.clone());
		self.get_attr(attr, args.binding())?.call("()", args)
	}

	pub fn call(&self, txt: &'static str, args: Args) -> obj::Result<Object> {
		self.call_attr(&txt.into(), args)
	}
}





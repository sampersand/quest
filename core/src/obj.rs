use crate::{literals, Result, Args, types::{self, ObjectType}, EqResult};

use std::sync::{Arc, RwLock, atomic::{self, AtomicUsize}};
use std::fmt::{self, Debug, Formatter};
use std::any::{Any, TypeId};

mod mapping;
use self::mapping::Mapping;
pub use self::mapping::Key;
pub use self::mapping::Value;

#[derive(Clone)]
pub struct Object(pub(super) Arc<Internal>);

impl Default for Object {
	fn default() -> Self {
		Object::new(types::Null::new())
	}
}

pub(super) struct Internal {
	pub(super) mapping: Arc<RwLock<Mapping>>,
	id: usize,
	// binding: Binding,
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
				.field(&self.0.id)
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

impl EqResult for Object {
	fn equals(&self, rhs: &Object) -> Result<bool> {
		Ok(self.call_attr(literals::EQL, vec![rhs.clone()])?
			.downcast_ref::<types::Boolean>()
			.map(|x| bool::from(*x))
			.unwrap_or(false))
	}

	fn into_object(&self) -> Object {
		self.clone()
	}
}

impl EqResult<Key> for Object {
	fn equals(&self, rhs: &Key) -> Result<bool> {
		rhs.equals(self)
	}

	fn into_object(&self) -> Object {
		self.clone()
	}
}


impl Object {
	pub fn new_with_parent<T, P>(data: T, parents: P) -> Self 
	where
		T: Any + Debug + Send + Sync,
		P: Into<mapping::Parents>
	{
		static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
		//println!("making object ({}) = {:?}", id, data);
		Object(Arc::new(Internal {
			id: ID_COUNTER.fetch_add(1, atomic::Ordering::Relaxed),
			// binding: Binding::instance(),
			mapping: Arc::new(RwLock::new(Mapping::new(parents))),
			data: Arc::new(RwLock::new(data)),
			dbg: (|x, f| <T as Debug>::fmt(x.downcast_ref::<T>().expect("bad val givent to debug"), f))
		}))
	}

	pub fn new<T: ObjectType>(data: T) -> Self {
		Object::new_with_parent(data, vec![T::mapping()])
	}

	pub fn id(&self) -> usize {
		self.0.id
	}

	pub fn is_identical(&self, rhs: &Object) -> bool {
		Arc::ptr_eq(&self.0, &rhs.0)
	}
}

impl Object {
	pub fn is_a<T: Any>(&self) -> bool {
		self.0.data.read().expect("poison error").is::<T>()
	}

	pub fn try_downcast_clone<T: Any + Clone>(&self) -> Result<T> {
		self.downcast_clone().ok_or_else(|| types::Text::from(format!("not a {:?}", TypeId::of::<T>())).into())
	}

	pub fn downcast_clone<T: Any + Clone>(&self) -> Option<T> {
		#[allow(clippy::map_clone)]
		self.downcast_ref::<T>().map(|x| x.clone())
	}

	pub fn try_downcast_ref<'a, T: Any>(&'a self) -> Result<impl std::ops::Deref<Target = T> + 'a> {
		self.downcast_ref::<T>().ok_or_else(||
			types::Text::from(format!("{:?} is not a {:?}", self, std::any::type_name::<T>())).into()
		)
	}

	pub fn downcast_ref<'a, T: Any>(&'a self) -> Option<impl std::ops::Deref<Target=T> + 'a> {
		use std::{sync::RwLockReadGuard, marker::PhantomData, ops::Deref};
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

	pub fn try_downcast_mut<'a, T: Any>(&'a self) -> Result<impl std::ops::DerefMut<Target = T> + 'a> {
		self.downcast_mut::<T>().ok_or_else(||
			types::Text::from(format!("{:?} is not a {:?}", self, std::any::type_name::<T>())).into()
		)
	}


	pub fn downcast_mut<'a, T: Any>(&'a self) -> Option<impl std::ops::DerefMut<Target=T> + 'a> {
		use std::{sync::RwLockWriteGuard, marker::PhantomData, ops::{Deref, DerefMut}};
		struct Caster<'a, T>(RwLockWriteGuard<'a, dyn Any + Send + Sync>, PhantomData<T>);
		impl<'a, T: 'static> Deref for Caster<'a, T> {
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

	pub fn dot_get_attr<K>(&self, attr: &K) -> Result<Object>
	where
		K: Debug + ?Sized + EqResult<Key>
	{
		let result = self.get_attr(attr)?;
		// by removing `Block`, we completely change the functionality. remove this hack?
		if result.is_a::<types::RustFn>() || //result.is_a::<types::Block>() ||
				result.is_a::<types::BoundFunction>() {
			let bound_res = Object::new(crate::types::BoundFunction);
			bound_res.set_attr("__bound_object_owner__", self.clone())?;
			bound_res.set_attr("__bound_object__", result)?;
			Ok(bound_res)	
		} else {
			Ok(result)
		}
	}

	pub fn get_attr<K>(&self, attr: &K) -> Result<Object>
	where
		K: Debug + ?Sized + EqResult<Key>
	{
		self.0.mapping.read().expect("cannot read").get(attr)?
			.ok_or_else(|| format!("attr {:?} does not exist for {:?}", attr, self).into())
	}

	pub fn has_attr<K>(&self, attr: &K) -> Result<Object>
	where
		K: Debug + ?Sized + EqResult<Key>
	{
		// println!("{:#?}", &*self.0.mapping.read().unwrap());
		self.0.mapping.read().expect("cannot read").has(attr).map(bool::into)
	}

	pub fn set_attr_possibly_parents<K, V>(&self, attr: K, value: V) -> Result<Object>
	where
		K: Into<Key>,
		V: Into<Value> + Into<mapping::Parents>
	{
		self.0.mapping.write().expect("cannot write").insert(attr.into(), value)
	}

	pub fn set_attr<K, V>(&self, attr: K, value: V) -> Result<Object>
	where
		K: Into<Key>,
		V: Into<Value>
	{
		self.0.mapping.write().expect("cannot write").insert_not_parents(attr.into(), value)
	}

	pub fn del_attr<K>(&self, attr: &K) -> Result<Object>
	where
		K: Debug + ?Sized + EqResult<Key>
	{
		self.0.mapping.write().expect("cannot write").remove(attr)
	}

	pub fn add_parent(&self, val: Object) -> Result<()> {
		self.0.mapping.write().expect("cannot write").add_parent(val)
	}

	pub fn call_attr<'a, K, A>(&self, attr: &K, args: A) -> Result<Object>
	where
		K: Debug + ?Sized + EqResult<Key>, A: Into<Args<'a>>
	{
		let mut args = args.into();

		if self.downcast_ref::<types::BoundFunction>().is_some() && attr.equals(&"()".into())? {
			args.add_this(self.clone());
			return crate::types::bound_function::impls::call(args);
		}

		if let Some(rustfn) = self.downcast_ref::<types::RustFn>() {
			if attr.equals(&"()".into())? {
				return rustfn.call(args);	
			}
		}

		let bound_attr = Object::new(crate::types::BoundFunction);
		bound_attr.set_attr("__bound_object_owner__", self.clone())?;
		bound_attr.set_attr("__bound_object__", self.get_attr(attr)?)?;
		bound_attr.call_attr("()", args)
		// args.add_this(self.clone());
		// attr.call_attr("()", args)

		// if attr.equals(&".".into())? {
		// 	return unimplemented!();
		// }
		// self.call_attr("."), vec![attr.into_object()])?
		// 	.call_attr("()", args)
		// Object::call_attr::<Key, Args>(&self.get_attr(attr)?, &"()".into(), args)
		// args.add_this(self.clone());
		// let result = self.get_attr(attr)?;
		// let bound_res = Object::new(crate::types::BoundFunction);
		// bound_res.set_attr("__bound_object_owner__", self.clone())?;
		// bound_res.set_attr("__bound_object__", result);
		// bound_res.call_attr("()", args.args(..)?)

		// println!(">> {:?} {:?} {:?}", self, attr, self.get_attr(attr));
		// Object::call_attr::<Key, Args>(&self.get_attr(attr)?, &"()".into(), args)
	}

}

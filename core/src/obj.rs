use crate::{Result, Args, ArgsOld};
use crate::error::{TypeError, KeyError};
use crate::types::{self, ObjectType};

use std::sync::{Arc, RwLock, atomic::{self, AtomicUsize}};
use std::fmt::{self, Debug, Formatter};
use std::any::Any;
use std::ops::{Deref, DerefMut};

pub mod mapping;
use mapping::{Mapping, Value};
pub use mapping::{Key, EqKey};

pub trait ToObject {
	fn to_object(&self) -> Object;
}

#[derive(Clone)]
pub struct Object(pub(super) Arc<Internal>);

impl Default for Object {
	fn default() -> Self {
		Object::new(types::Null::new())
	}
}

pub(super) struct Internal {
	mapping: RwLock<Mapping>,
	id: usize,
	// binding: Binding,
	data: RwLock<Box<dyn Any + Send + Sync>>,
	dbg: fn(&dyn Any, &mut Formatter) -> fmt::Result,
	#[cfg(debug_assertions)]
	typename: &'static str
}

impl Debug for Object {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(&self.0, f)
	}
}

impl Debug for Internal {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		struct DataDebug<'a>(&'a dyn Any, fn(&dyn Any, &mut Formatter) -> fmt::Result);
		impl Debug for DataDebug<'_> {
			fn fmt(&self, f: &mut Formatter) -> fmt::Result {
				(self.1)(self.0, f)
			}
		}

		if f.alternate() {
			f.debug_struct("Object")
				.field("id", &self.id)
				.field("data", &DataDebug(&**self.data.read().expect("poisoned"), self.dbg))
				.field("mapping", &*self.mapping.read().expect("cant read in obj"))
				.finish()
		} else {
			f.debug_tuple("Object")
				.field(&DataDebug(&**self.data.read().expect("poisoned"), self.dbg))
				.field(&self.typename)
				.field(&self.id)
				.finish()
		}
	}
}

// do we want this trait even?
impl ToObject for Key {
	fn to_object(&self) -> Object { self.clone().into() }
}

impl ToObject for Object {
	#[inline]
	fn to_object(&self) -> Object {
		self.clone()
	}
}

impl<T: Any + ObjectType> From<T> for Object {
	fn from(data: T) -> Object {
		Object::new(data)
	}
}

impl Object {
	pub fn new_with_parent<T, P>(data: T, parents: P) -> Self 
	where
		T: Any + Debug + Send + Sync,
		P: Into<mapping::Parents>
	{
		static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
		let id = ID_COUNTER.fetch_add(1, atomic::Ordering::Relaxed);
		// println!("making object ({}) = {:?}", id, data);
		let obj = Object(Arc::new(Internal {
			id: id,
			// id: ID_COUNTER.fetch_add(1, atomic::Ordering::Relaxed),
			// binding: Binding::instance(),
			mapping: RwLock::new(Mapping::new(parents)),
			data: RwLock::new(Box::new(data)),
			#[cfg(debug_assertions)]
			typename: std::any::type_name::<T>(),
			dbg: |x, f| T::fmt(x.downcast_ref::<T>().expect("bad val givent to debug"), f)
		}));

		obj.0.mapping.write().unwrap().obj = Arc::downgrade(&obj.0);
		obj
	}

	pub fn new<T: ObjectType>(data: T) -> Self {
		Object::new_with_parent(data, vec![T::mapping()])
	}

	#[inline]
	pub fn id(&self) -> usize {
		self.0.id
	}

	#[inline]
	pub fn typename(&self) -> &'static str {
		self.0.typename
	}

	#[inline]
	pub fn is_identical(&self, rhs: &Object) -> bool {
		Arc::ptr_eq(&self.0, &rhs.0)
	}

	pub fn eq_obj(&self, rhs: &Object) -> Result<bool> {
		self.call_attr_old("==", &[rhs.clone()])
			.map(|res| res.downcast_ref::<types::Boolean>()
				.map(|b| bool::from(*b))
				.unwrap_or(false))
	}
}

impl Object {
	pub fn is_a<T: Any>(&self) -> bool {
		self.0.data.read().expect("poison error").is::<T>()
	}

				#[allow(unreachable_code)]
	pub fn try_downcast_clone<T: Any + Clone>(&self) -> Result<T> {
		self.downcast_clone()
			.ok_or_else(|| TypeError::WrongType {
				expected: std::any::type_name::<T>(),
				#[cfg(debug_assertions)]
				got: self.0.typename,
				#[cfg(not(debug_assertions))]
				got: "<todo: typename impl>"
			}.into())
	}

	pub fn downcast_clone<T: Any + Clone>(&self) -> Option<T> {
		#[allow(clippy::map_clone)]
		self.downcast_ref::<T>().map(|x| x.clone())
	}

	pub fn try_downcast_ref<'a, T: Any>(&'a self) -> Result<impl Deref<Target = T> + 'a> {
		self.downcast_ref::<T>()
			.ok_or_else(|| TypeError::WrongType {
				expected: std::any::type_name::<T>(),
				#[cfg(debug_assertions)]
				got: self.0.typename,
				#[cfg(not(debug_assertions))]
				got: "<todo: typename impl>"
			}.into())
	}

	pub fn downcast_ref<'a, T: Any>(&'a self) -> Option<impl Deref<Target=T> + 'a> {
		if self.is_a::<T>() {
			Some(unsafe { self.downcast_ref_unchecked() })
		} else {
			None
		}
	}

	pub unsafe fn downcast_ref_unchecked<'a, T: Any>(&'a self) -> impl Deref<Target=T> + 'a {
		use std::sync::RwLockReadGuard;
		use std::marker::PhantomData;

		struct Caster<'a, T>(RwLockReadGuard<'a, Box<dyn Any + Send + Sync>>, PhantomData<T>);
		impl<'a, T: 'static> Deref for Caster<'a, T> {
			type Target = T;
			fn deref(&self) -> &T {
				self.0.downcast_ref::<T>().unwrap()
			}
		}

		Caster::<'a, T>(self.0.data.read().expect("poison error"), PhantomData)
	}

	pub fn try_downcast_mut<'a, T: Any>(&'a self) -> Result<impl DerefMut<Target = T> + 'a> {
		self.downcast_mut::<T>()
			.ok_or_else(|| TypeError::WrongType {
				expected: std::any::type_name::<T>(),
				#[cfg(debug_assertions)]
				got: self.0.typename,
				#[cfg(not(debug_assertions))]
				got: "<todo: typename mut impl>"
			}.into())
	}


	pub fn downcast_mut<'a, T: Any>(&'a self) -> Option<impl DerefMut<Target=T> + 'a> {
		if self.is_a::<T>() {
			Some(unsafe { self.downcast_mut_unchecked() })
		} else {
			None
		}
	}

	pub unsafe fn downcast_mut_unchecked<'a, T: Any>(&'a self) -> impl DerefMut<Target=T> + 'a {
		use std::{sync::RwLockWriteGuard, marker::PhantomData};

		struct Caster<'a, T>(RwLockWriteGuard<'a, Box<dyn Any + Send + Sync>>, PhantomData<T>);
		impl<'a, T: 'static> Deref for Caster<'a, T> {
			type Target = T;
			fn deref(&self) -> &T {
				match self.0.downcast_ref::<T>() {
					Some(t) => t,
					None => unreachable!()
				}
			}
		}

		impl<'a, T: 'static> DerefMut for Caster<'a, T> {
			fn deref_mut(&mut self) -> &mut T {
				match self.0.downcast_mut::<T>() {
					Some(t) => t,
					None => unreachable!()
				}
			}
		}

		Caster::<'a, T>(self.0.data.write().expect("poison error"), PhantomData)
	}
}

impl Object {
	pub fn dot_get_attr<K: ?Sized>(&self, attr: &K) -> Result<Object>
	where
		K: Debug + EqKey + ToObject
	{
		let result = self.get_attr(attr)?;

		// remove this hack? lol
		if result.is_a::<types::RustFn>() || format!("{:?}", result).starts_with("Object(Block") ||
				result.is_a::<types::BoundFunction>() {
			let bound_res = Object::new(crate::types::BoundFunction);
			bound_res.set_attr("__bound_object_owner__", self.clone())?;
			bound_res.add_parent(result.clone())?;
			bound_res.set_attr("__bound_object__", result)?;
			Ok(bound_res)	
		} else {
			Ok(result)
		}
	}

	fn get_value<K: ?Sized>(&self, attr: &K) -> Result<Value>
	where
		K: EqKey + ToObject
	{
		// TODO: attr missing should be within `mapping`
		self.0.mapping.read().expect("cannot read")
			.get(attr)?
			.ok_or_else(|| KeyError::DoesntExist { attr: attr.to_object(), obj: self.clone() }.into())
	}

	#[inline]
	pub fn get_attr<K: ?Sized>(&self, attr: &K) -> Result<Object>
	where
		K: EqKey + ToObject 
	{
		self.get_value(attr).map(|x| x.into())
	}

	#[inline]
	pub fn has_attr<K: ?Sized>(&self, attr: &K) -> Result<bool>
	where
		K: EqKey
	{
		self.0.mapping.read().expect("cannot read").has(attr)
	}

	#[inline]
	pub fn set_attr<K, V>(&self, attr: K, value: V) -> Result<()>
	where
		K: Into<Key>,
		V: Into<Value>
	{
		self.0.mapping.write().expect("cannot write").insert(attr.into(), value.into())
	}

	#[inline]
	pub fn del_attr<K: ?Sized>(&self, attr: &K) -> Result<Object>
	where
		K: EqKey + ToObject
	{
		self.0.mapping.write().expect("cannot write").remove(attr)?
			.ok_or_else(|| KeyError::DoesntExist { attr: attr.to_object(), obj: self.clone() }.into())
	}

	pub fn call_attr<'s, 'o: 's, K: ?Sized, A>(&self, attr: &K, args: A) -> Result<Object>
	where
		K: EqKey + ToObject,
		A: Into<Args<'s, 'o>>
	{
		match self.get_value(attr)? {
			Value::RustFn(rustfn) => {
				rustfn.call(self, args.into())
			},

			Value::Object(object) => {
				let bound_attr = Object::new(crate::types::BoundFunction);
				bound_attr.set_attr("__bound_object_owner__", self.clone())?;
				bound_attr.set_attr("__bound_object__", object)?;
				bound_attr.call_attr_old("()", args.into())
			}
		}
	}
	pub fn call_attr_old<'a, K: ?Sized, A>(&self, attr: &K, args: A) -> Result<Object>
	where
		K: EqKey + ToObject,
		A: Into<ArgsOld<'a>>
	{
		match self.get_value(attr)? {
			Value::RustFn(rustfn) => {
				let mut args = args.into();
				args.add_this(self.clone());
				rustfn.call_old(args)
			},

			Value::Object(object) => {
				let bound_attr = Object::new(crate::types::BoundFunction);
				bound_attr.set_attr("__bound_object_owner__", self.clone())?;
				bound_attr.set_attr("__bound_object__", object)?;
				bound_attr.call_attr_old("()", args)
			}
		}
	}

	pub fn add_parent(&self, val: Object) -> Result<()> {
		self.0.mapping.write().expect("cannot write").add_parent(val)
	}

	pub fn mapping_keys(&self, include_parents: bool) -> Vec<Key> {
		let mut keys = self.0.mapping.read().expect("cant read").keys();
		if include_parents {
			if let Ok(parents) = self.get_attr("__parents__") {
				for key in parents.downcast_call::<types::List>().unwrap().into_iter()
					.map(|x| x.mapping_keys(true))
					.flatten()
				{
					if !keys.iter().any(|k| k.eq_key(&key).unwrap_or(false)) {
						keys.push(key);
					}
				}
			}
		}

		keys
	}
}

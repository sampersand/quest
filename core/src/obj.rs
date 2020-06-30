use crate::{Result, Args, ArgsOld};
use crate::error::{TypeError, KeyError};
use crate::types::{self, ObjectType};
use crate::literals::Literal;

use std::borrow::Borrow;
use std::hash::Hash;
use std::sync::Arc;
use std::fmt::{self, Debug, Formatter};
use std::any::Any;
use std::ops::{Deref, DerefMut};

mod data;
mod attributes;
use attributes::{Attributes, Value};
pub use data::Data;

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
	attrs: Attributes,
	data: Data,
}

impl Debug for Object {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(&self.0, f)
	}
}


impl From<!> for Object {
	fn from(_: !) -> Self {
		unsafe { std::hint::unreachable_unchecked() }
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
				.field("data", &self.data)
				.field("attrs", &self.attrs)
				.finish()
		} else {
			f.debug_tuple("Object")
				.field(&self.data)
				.field(&self.attrs.id())
				.finish()
		}
	}
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
	#[inline]
	pub fn new_with_parent<T, P>(data: T, parents: P) -> Self 
	where
		T: Any + Debug + Send + Sync + Clone,
		P: Into<attributes::Parents>
	{
		// println!("creating new object: {:?} ({:?})", data, std::any::type_name::<T>());
		Object::from_parts(Data::new(data), Attributes::new(parents))
	}

	#[inline]
	fn from_parts(data: Data, attrs: Attributes) -> Self {
		Object(Arc::new(Internal { data, attrs }))
	}

	#[inline]
	pub fn new<T: ObjectType>(data: T) -> Self {
		data.new_object()
	}

	#[inline]
	pub fn data(&self) -> &Data {
		&self.0.data
	}

	#[inline]
	pub fn id(&self) -> usize {
		self.0.attrs.id()
	}

	#[inline]
	pub fn typename(&self) -> &'static str {
		self.0.data.typename()
	}

	#[inline]
	pub fn is_identical(&self, rhs: &Object) -> bool {
		Arc::ptr_eq(&self.0, &rhs.0)
	}

	pub fn eq_obj(&self, rhs: &Object) -> Result<bool> {
		self.call_attr_lit("==", &[rhs])
			.map(|res| res.downcast_ref::<types::Boolean>()
				.map(|b| bool::from(*b))
				.unwrap_or(false))
	}

	pub fn deep_clone(&self) -> Object {
		Object::from_parts(self.0.data.clone(), self.0.attrs.clone())
	}
}

impl Object {
	#[inline]
	pub fn is_a<T: Any>(&self) -> bool {
		self.0.data.is_a::<T>()
	}

	pub fn try_downcast_clone<T: Any + Clone>(&self) -> Result<T> {
		self.downcast_clone()
			.ok_or_else(|| TypeError::WrongType {
				expected: std::any::type_name::<T>(),
				got: self.typename(),
			}.into())
	}

	#[inline]
	pub fn downcast_clone<T: Any + Clone>(&self) -> Option<T> {
		self.downcast_ref::<T>().map(|x| x.clone())
	}

	pub fn try_downcast_ref<'a, T: Any>(&'a self) -> Result<impl Deref<Target = T> + 'a> {
		self.downcast_ref::<T>()
			.ok_or_else(|| TypeError::WrongType {
				expected: std::any::type_name::<T>(),
				got: self.typename(),
			}.into())
	}

	#[inline]
	pub fn downcast_ref<'a, T: Any>(&'a self) -> Option<impl Deref<Target=T> + 'a> {
		self.0.data.downcast_ref()
	}

	#[inline]
	pub unsafe fn downcast_ref_unchecked<'a, T: Any>(&'a self) -> impl Deref<Target=T> + 'a {
		self.0.data.downcast_ref_unchecked()
	}

	pub fn try_downcast_mut<'a, T: Any>(&'a self) -> Result<impl DerefMut<Target = T> + 'a> {
		self.downcast_mut::<T>()
			.ok_or_else(|| TypeError::WrongType {
				expected: std::any::type_name::<T>(),
				got: self.typename(),
			}.into())
	}


	#[inline]
	pub fn downcast_mut<'a, T: Any>(&'a self) -> Option<impl DerefMut<Target=T> + 'a> {
		self.0.data.downcast_mut()
	}

	#[inline]
	pub unsafe fn downcast_mut_unchecked<'a, T: Any>(&'a self) -> impl DerefMut<Target=T> + 'a {
		self.0.data.downcast_mut_unchecked()
	}
}

impl Object {
	pub fn has_attr_lit<K: Hash + Eq + ?Sized>(&self, attr: &K) -> Result<bool>
	where
		for <'a> &'a str: Borrow<K>
	{
		self.0.attrs.has_lit(attr)
	}

	pub fn get_value_lit<K: Hash + Eq + ?Sized>(&self, attr: &K) -> Result<Option<Value>>
	where
		for <'a> &'a str: Borrow<K>
	{
		self.0.attrs.get_lit(attr)
	}

	pub fn get_attr_lit<K: Hash + Eq + ?Sized>(&self, attr: &K) -> Result<Object>
	where
		for <'a> &'a str: Borrow<K>,
		K: ToObject 
	{
		self.get_value_lit(attr)?
			.map(Object::from)
			.ok_or_else(|| KeyError::DoesntExist { attr: attr.to_object(), obj: self.clone() }.into())
	}

	pub fn set_attr_lit<V: Into<Value>>(&self, attr: Literal, value: V) {
		self.0.attrs.set_lit(attr, value.into())
	}

	pub fn del_attr_lit<K: Hash + Eq + ?Sized>(&self, attr: &K) -> Option<Value>
	where
		for <'a> &'a str: Borrow<K>,
	{
		self.0.attrs.del_lit(attr)
	}

	pub fn call_attr_lit<'s, 'o: 's, A, K: ?Sized>(&self, attr: &K, args: A) -> Result<Object>
	where
		for <'a> &'a str: Borrow<K>,
		K: Hash + Eq + ToObject,
		A: Into<Args<'s, 'o>>
	{
		self.get_value_lit(attr)?
			.ok_or_else(|| KeyError::DoesntExist { attr: attr.to_object(), obj: self.clone() })?
			.call(self, args.into())
	}

	pub fn has_attr(&self, attr: &Object) -> Result<bool> {
		self.0.attrs.has(attr)
	}

	pub fn get_value(&self, attr: &Object) -> Result<Option<Value>> {
		self.0.attrs.get(attr)
	}

	pub fn get_attr(&self, attr: &Object) -> Result<Object> {
		self.0.attrs.get(attr)?
			.map(Object::from)
			.ok_or_else(|| KeyError::DoesntExist { attr: attr.to_object(), obj: self.clone() }.into())
	}

	pub fn set_attr<V: Into<Value>>(&self, attr: Object, value: V) -> Result<()> {
		self.0.attrs.set(attr, value.into())
	}

	pub fn del_attr(&self, attr: &Object) -> Result<Object> {
		self.0.attrs.del(attr)?
			.map(Object::from)
			.ok_or_else(|| KeyError::DoesntExist { attr: attr.to_object(), obj: self.clone() }.into())
	}

	pub fn call_attr<'s, 'o: 's, A>(&self, attr: &Object, args: A) -> Result<Object>
	where
		A: Into<Args<'s, 'o>>
	{
		// TODO: this
		self.get_value(attr)?
			.ok_or_else(|| KeyError::DoesntExist { attr: attr.to_object(), obj: self.clone() })?
			.call(self, args.into())
	}
}

impl Object {
	pub fn dot_get_attr(&self, attr: &Object) -> Result<Object> {
		let result = self.get_attr(attr)?;

		// remove this hack? lol
		if result.is_a::<types::RustFn>() || format!("{:?}", result).starts_with("Object(Block") ||
				result.is_a::<types::BoundFunction>() {
			let bound_res = Object::new(crate::types::BoundFunction);
			bound_res.set_attr_lit("__bound_object_owner__", self.clone());
			bound_res.add_parent(result.clone())?;
			bound_res.set_attr_lit("__bound_object__", result);
			Ok(bound_res)	
		} else {
			Ok(result)
		}
	}

	pub fn call_attr_old_old<'a, K: ?Sized, A>(&self, attr: &K, args: A) -> Result<Object>
	where
		K: ToObject,
		A: Into<ArgsOld<'a>>
	{
		let a = self.get_value(&attr.to_object())?
			.ok_or_else(|| KeyError::DoesntExist { attr: attr.to_object(), obj: self.clone() })?;
		match a {
			Value::RustFn(rustfn) => {
				let mut args = args.into();
				args.add_this(self.clone());
				rustfn.call_old(args)
			},

			Value::Object(object) => {
				let bound_attr = Object::new(crate::types::BoundFunction);
				bound_attr.set_attr_lit("__bound_object_owner__", self.clone());
				bound_attr.set_attr_lit("__bound_object__", object);
				bound_attr.call_attr_old_old("()", args)
			}
		}
	}

	#[inline]
	pub fn add_parent(&self, val: Object) -> Result<()> {
		self.0.attrs.add_parent(val)
	}

	#[inline]
	pub fn mapping_keys(&self, include_parents: bool) -> Result<Vec<Object>> {
		self.0.attrs.keys(include_parents)
	}
}

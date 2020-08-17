mod data;
mod attributes;

pub use data::Data;
pub use attributes::{Attributes, Value, Parents};
use crate::types::ObjectType;
use crate::Object;
use std::sync::Arc;
use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct HeapData(Arc<Internal>);

pub struct Internal {
	/// The attributes (such as id, keys, and parents) of this object
	attrs: Attributes,
	/// The actual data of this object
	data: Data,
}

impl std::ops::Deref for HeapData {
	type Target = Internal;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

use std::fmt::{self, Debug, Formatter};

impl Debug for HeapData {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(&self.0, f)
	}
}

impl Debug for Internal {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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

impl HeapData {
	pub(super) fn new_with_parents(data: Data, attrs: Attributes) -> Self {
		HeapData(Arc::new(Internal { data, attrs }))
	}

	#[allow(unused)]
	pub fn new<T: ObjectType>(data: T) -> Self {
		Self::new_with_parents(Data::new(data), Attributes::new(vec![T::mapping()]))
		// Self(ObjectRepr::from_parts(
		// 	repr::heap_only::Data::new(data),
		// 	repr::heap_only::Attributes::new(parents)))

		// HeapData(Arc::new(Internal { data, attrs }))
	}

	#[inline]
	pub(super) fn into_raw(self) -> *const Internal {
		Arc::into_raw(self.0)
	}

	#[inline]
	pub(super) unsafe fn from_raw(ptr: *const Internal) -> Self {
		Self(Arc::from_raw(ptr))
	}

	#[inline]
	pub fn is_identical(&self, rhs: &Self) -> bool {
		Arc::ptr_eq(&self.0, &rhs.0)
	}

	#[inline]
	pub fn deep_clone(&self) -> Self {
		Self::new_with_parents(self.0.data.clone(), self.0.attrs.clone())
	}
}

impl Internal {
	#[inline]
	pub fn id(&self) -> usize {
		self.attrs.id()
	}

	#[inline]
	pub fn typename(&self) -> &'static str {
		self.data.typename()
	}

	#[inline]
	pub fn is_a<T: ObjectType>(&self) -> bool {
		self.data.is_a::<T>()
	}

	#[inline]
	pub fn downcast<'a, T: ObjectType>(&'a self) -> Option<impl Deref<Target=T> + 'a> {
		self.data.downcast()
	}

	#[inline]
	pub fn downcast_mut<'a, T: ObjectType>(&'a self) -> Option<impl DerefMut<Target=T> + 'a> {
		self.data.downcast_mut()
	}

	#[inline]
	pub fn has_lit(&self, attr: &str) -> crate::Result<bool> {
		self.attrs.has_lit(attr)
	}

	#[inline]
	pub fn get_lit(&self, attr: &str) -> crate::Result<Option<Value>> {
		self.attrs.get_lit(attr)
	}

	#[inline]
	pub fn set_lit(&self, attr: crate::Literal, value: Value) -> crate::Result<()> {
		self.attrs.set_lit(attr, value);
		Ok(())
	}

	#[inline]
	pub fn del_lit(&self, attr: &str) -> crate::Result<Option<Value>> {
		Ok(self.attrs.del_lit(attr))
	}

	#[inline]
	pub fn has(&self, attr: &Object) -> crate::Result<bool> {
		self.attrs.has(attr)
	}

	#[inline]
	pub fn get(&self, attr: &Object) -> crate::Result<Option<Value>> {
		self.attrs.get(attr)
	}

	#[inline]
	pub fn set(&self, attr: Object, value: Value) -> crate::Result<()> {
		self.attrs.set(attr, value)
	}

	#[inline]
	pub fn del(&self, attr: &Object) -> crate::Result<Option<Value>> {
		self.attrs.del(attr)
	}

	#[inline]
	pub fn add_parent(&self, val: Object) -> crate::Result<()> {
		self.attrs.add_parent(val)
	}

	#[inline]
	pub fn keys(&self, include_parents: bool) -> crate::Result<Vec<Object>> {
		self.attrs.keys(include_parents)
	}
}

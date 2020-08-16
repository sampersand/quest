mod data;
mod attributes;

pub use data::Data;
pub use attributes::{Attributes, Value, Parents};
use crate::types::ObjectType;
use crate::Object;
use std::sync::Arc;
use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct HeapOnly(Arc<Internal>);

struct Internal {
	/// The attributes (such as id, keys, and parents) of this object
	attrs: Attributes,
	/// The actual data of this object
	data: Data,
}

use std::fmt::{self, Debug, Formatter};

impl Debug for HeapOnly {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			f.debug_struct("Object")
				.field("data", &self.0.data)
				.field("attrs", &self.0.attrs)
				.finish()
		} else {
			f.debug_tuple("Object")
				.field(&self.0.data)
				.field(&self.0.attrs.id())
				.finish()
		}
	}
}

impl HeapOnly {
	pub(super) fn new(data: Data, attrs: Attributes) -> Self {
		HeapOnly(Arc::new(Internal { data, attrs }))
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
	pub fn deep_clone(&self) -> Self {
		Self::new(self.0.data.clone(), self.0.attrs.clone())
	}

	pub fn is_identical(&self, rhs: &Self) -> bool {
		Arc::ptr_eq(&self.0, &rhs.0)
	}

	#[inline]
	pub fn is_a<T: ObjectType>(&self) -> bool {
		self.0.data.is_a::<T>()
	}

	#[inline]
	pub fn downcast<'a, T: ObjectType>(&'a self) -> Option<impl Deref<Target=T> + 'a> {
		self.0.data.downcast()
	}

	#[inline]
	pub fn downcast_mut<'a, T: ObjectType>(&'a self) -> Option<impl DerefMut<Target=T> + 'a> {
		self.0.data.downcast_mut()
	}

	#[inline]
	pub fn has_lit(&self, attr: &str) -> crate::Result<bool> {
		self.0.attrs.has_lit(attr)
	}

	#[inline]
	pub fn get_lit(&self, attr: &str) -> crate::Result<Option<Value>> {
		self.0.attrs.get_lit(attr)
	}

	#[inline]
	pub fn set_lit(&self, attr: crate::Literal, value: Value) -> crate::Result<()> {
		self.0.attrs.set_lit(attr, value);
		Ok(())
	}

	#[inline]
	pub fn del_lit(&self, attr: &str) -> crate::Result<Option<Value>> {
		Ok(self.0.attrs.del_lit(attr))
	}

	#[inline]
	pub fn has(&self, attr: &Object) -> crate::Result<bool> {
		self.0.attrs.has(attr)
	}

	#[inline]
	pub fn get(&self, attr: &Object) -> crate::Result<Option<Value>> {
		self.0.attrs.get(attr)
	}

	#[inline]
	pub fn set(&self, attr: Object, value: Value) -> crate::Result<()> {
		self.0.attrs.set(attr, value)
	}

	#[inline]
	pub fn del(&self, attr: &Object) -> crate::Result<Option<Value>> {
		self.0.attrs.del(attr)
	}

	#[inline]
	pub fn add_parent(&self, val: Object) -> crate::Result<()> {
		self.0.attrs.add_parent(val)
	}

	#[inline]
	pub fn keys(&self, include_parents: bool) -> crate::Result<Vec<Object>> {
		self.0.attrs.keys(include_parents)
	}
}

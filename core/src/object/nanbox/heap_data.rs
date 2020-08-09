use crate::types::ObjectType;
use std::sync::Arc;
use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct HeapData(Arc<HeapInner>);

#[derive(Clone)]
pub struct HeapInner {
	// /// The attributes (such as id, keys, and parents) of this object
	// attrs: crate::obj::Attributes,
	// /// The actual data of this object
	// data: crate::obj::Data,

	// TODO: update this
	_obj: crate::obj::Object
}

impl<T: ObjectType> From<T> for HeapData {
	#[inline]
	fn from(data: T) -> Self {
		Self::new(data)
	}
}


impl HeapData {
	pub fn new<T: ObjectType>(data: T) -> Self {
		Self(Arc::new(HeapInner {
			_obj: data.new_object()
		}))
	}

	pub fn new_with_parents<T, P>(data: T, parents: P) -> Self
	where
		T: ObjectType,
		P: Into<crate::obj::attributes::Parents>
	{
		Self(Arc::new(HeapInner {
			_obj: crate::obj::Object::new_with_parent(data, parents)
		}))
	}

	#[inline]
	pub fn into_raw(self) -> *const HeapInner {
		Arc::into_raw(self.0)
	}

	#[inline]
	pub unsafe fn from_raw(raw: *const HeapInner) -> Self {
		Self(Arc::from_raw(raw))
	}
}

impl HeapInner {
	pub fn downcast<'a, T: 'static>(&'a self) -> Option<impl Deref<Target=T> + 'a> {
		self._obj.downcast()
	}

	pub fn downcast_mut<'a, T: 'static>(&'a self) -> Option<impl DerefMut<Target=T> + 'a> {
		self._obj.downcast_mut()
	}

	pub fn attrs(&self) -> &crate::obj::attributes::Attributes {
		self._obj._attrs()
	}
}

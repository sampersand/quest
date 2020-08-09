mod data;
mod heap_data;
mod tag;

pub(crate) use tag::Tag;
use heap_data::{HeapData, HeapInner};
use crate::types::ObjectType;
use std::ops::{Deref, DerefMut};

#[derive(PartialEq)]
#[repr(transparent)]
pub struct Data(DataInner);

#[repr(C)]
union DataInner {
	tag: Tag,
	data: u64,
}

#[test]
fn is_u64() {
	assert_eq!(std::mem::size_of::<DataInner>(), std::mem::size_of::<u64>());
}

impl Data {
	pub fn new<T: ObjectType>(data: T) -> Self {
		Self::from(HeapData::new(data))
	}

	pub fn new_with_parents<T, P>(data: T, parents: P) -> Self
	where
		T: ObjectType,
		P: Into<crate::obj::attributes::Parents>
	{
		Self::from(HeapData::new_with_parents(data, parents))
	}


	fn from_heap_untyped<'a>(&'a self) -> Option<&'a HeapInner> {
		if self.tag() == Tag::Heap {
			unsafe {
				Some(&*(self.data() as usize as *const HeapInner))
			}
		} else {
			None
		}
	}


	pub fn from_heap<'a, T: ObjectType>(&'a self) -> Option<TaggedResult<'a, T>> {
		self.from_heap_untyped().map(TaggedResult::Heap)
	}

	pub(crate) unsafe fn new_tagged(tag: Tag, data: u64) -> Self {
		debug_assert_eq!(data & 0xffff_ffff_ffff, data, "data has tag bits associated!");

		Self(DataInner { data: data | ((tag as u64) << 48) })
	}

	pub(crate) fn data_if_tag(&self, tag: Tag) -> Option<u64> {
		if self.tag() == tag {
			Some(self.data())
		} else {
			None
		}
	}

	#[inline]
	pub fn is_a<T: ObjectType>(&self) -> bool {
		T::is_a(self)
	}

	pub fn downcast<'a, T: ObjectType>(&'a self) -> Option<impl Deref<Target=T> + 'a> {
		enum TaggedDeref<T, U> {
			Data(T),
			Deref(U)
		}

		impl<T, U: Deref<Target=T>> Deref for TaggedDeref<T, U> {
			type Target = T;
			fn deref(&self) -> &Self::Target {
				match self {
					Self::Data(d) => d,
					Self::Deref(d) => &d
				}
			}
		}

		match T::from_data(self)? {
			TaggedResult::Copy(d) => Some(TaggedDeref::Data(d)),
			TaggedResult::Heap(d) => d.downcast().map(TaggedDeref::Deref)
		}
	}

	pub fn downcast_mut<'a, T: ObjectType>(&'a self) -> Option<impl DerefMut<Target=T> + 'a> {
		match T::from_data(self)? {
			TaggedResult::Copy(_d) => panic!("todo: downcast_mut"),
			TaggedResult::Heap(d) => d.downcast_mut()
		}
	}
}

/// hacky methods that will be reworked.
impl Data {
	pub fn attrs(&self) -> &crate::obj::attributes::Attributes {
		match self.tag() {
			Tag::Heap => self.from_heap_untyped().expect("heap mismatch?").attrs(),
			Tag::Null => crate::types::Null::attrs(self).expect("attrs mismatch?"),
			Tag::Boolean => crate::types::Boolean::attrs(self).expect("attrs mismatch?"),
			Tag::RustFn => crate::types::RustFn::attrs(self).expect("attrs mismatch?"),
			Tag::NumberI32 | Tag::NumberF32 => crate::types::Number::attrs(self).expect("attrs mismatch?"),
			Tag::ZeroSizedType => unimplemented!()
		}
	}

	pub fn has_lit(&self, attr: &str) -> crate::Result<bool> {
		self.attrs().has_lit(attr)
	}

	pub fn get_lit(&self, attr: &str) -> crate::Result<Option<crate::obj::Value>> {
		self.attrs().get_lit(attr)
	}

	pub fn set_lit(&self, attr: crate::literal::Literal, value: crate::obj::Value) {
		self.attrs().set_lit(attr, value);
	}

	pub fn del_lit(&self, attr: &str) -> Option<crate::obj::Value> {
		self.attrs().del_lit(attr)
	}

	pub fn has(&self, attr: &crate::object::Object) -> crate::Result<bool> {
		// self.attrs().has(attr)
		unreachable!()
	}

	pub fn get(&self, attr: &crate::object::Object) -> crate::Result<Option<crate::obj::Value>> {
		// self.attrs().get(attr)
		unreachable!()
	}

	pub fn set(&self, attr: crate::object::Object, value: crate::obj::Value) {
		// self.attrs().set(attr, value);
		unreachable!()
	}

	pub fn del(&self, attr: &str) -> Option<crate::obj::Value> {
		// self.attrs().del(attr)
		unreachable!()
	}
}

pub enum TaggedResult<'a, T> {
	Copy(T),
	Heap(&'a HeapInner)
}

pub unsafe trait Tagged : Sized {
	#[inline]
	fn into_data(self) -> Data where Self: ObjectType {
		Data::new(self)
	}

	#[inline]
	fn is_a(data: &Data) -> bool where Self: ObjectType {
		Self::from_data(data).is_some()
	}

	fn from_data<'a>(data: &'a Data) -> Option<TaggedResult<'a, Self>> where Self: ObjectType {
		data.from_heap()
	}

	fn attrs<'a>(data: &'a Data) -> Option<&'a crate::obj::attributes::Attributes> where Self: ObjectType {
		match Self::from_data(data)? {
			TaggedResult::Copy(_) => None,
			TaggedResult::Heap(inner) => Some(inner.attrs())
		}
	}
}

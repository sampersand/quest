#![allow(unused)]

#[cfg(target_endian="big")]
compile_error!("big endian not supported currently");

mod tag;
// mod inner;
// use inner::{Inner, Data};
use tag::Tag;

use super::HeapData;
use crate::types::{ObjectType, Number};
use std::mem::transmute;
use std::sync::atomic::{AtomicU8, AtomicU64, Ordering};
use std::cell::UnsafeCell;
use std::fmt::{self, Debug, Formatter};
use std::ops::{Deref, DerefMut};

#[repr(C, align(8))]
pub struct NaNBox(AtomicU64);

#[repr(C, align(8))]
struct Internal {
	lock: parking_lot::Mutex<()>,
	tag: UnsafeCell<Tag>,
	data: UnsafeCell<[u8; 6]>
}

impl NaNBox {
	fn new(tag: Tag, data: [u8; 6]) -> Self {
		Self::new_internal(Internal {
			lock: Default::default(),
			tag: UnsafeCell::new(tag),
			data: UnsafeCell::new(data)
		})
	}

	fn new_heap_internal(ptr: *const super::heapdata::Internal) -> Self {
		let [data @ .., _a, _b] = (ptr as u64).to_le_bytes();
		assert_eq!(_a | _b, 0, "upper bits of pointer ({:p}) weren't zero", ptr as *const ());
		Self::new(Tag::Heap, data)
	}

	fn new_internal(internal: Internal) -> Self {
		Self(AtomicU64::new(unsafe { transmute::<Internal, u64>(internal) }))
	}

	pub fn new_heap<T: ObjectType>(data: T) -> Self {
		Self::new_heap_internal(HeapData::new(data).into_raw())
	}

	pub fn downcast_heap<T: std::any::Any>(&self) -> Option<&T> {
		self.read_internal();//.downcast_heap(): Any
		unimplemented!()
	}

	fn read_internal(&self) -> Internal {
		let guard = self.read();
		unsafe { transmute::<u64, Internal>(self.0.load(Ordering::Relaxed)) }
	}

	// right now, both read and write are simply mutexes, but in the future we'll differentiate.
	fn read<'a>(&'a self) -> impl Drop + 'a {
		unsafe {
			(*(self as *const Self as *const Internal)).lock.lock()
		}
	}

	fn write<'a>(&'a self) -> impl Drop + 'a {
		unsafe {
			(*(self as *const Self as *const Internal)).lock.lock()
		}
	}
}

impl Internal {
	fn as_ptr(&self) -> *const super::heapdata::Internal {
		unsafe { *(self as *const _ as *const u64) as *const _ }
	}

	fn tag(&self) -> Tag {
		unsafe { *self.tag.get() }
	}

	fn data(&self) -> [u8; 6] {
		unsafe { *self.data.get() }
	}

	fn le_bytes(&self) -> [u8; 4] {
		let [bits @ .., _, _] = self.data();
		bits
	}

	unsafe fn duplicate(&self) -> Self {
		transmute::<u64, Self>(*(self as *const _ as *const u64))
	}

	// fn downcast_heap<'a, T: ObjectType>(&'a self) -> Option<impl std::ops::Deref<Target=T> + 'a> {
		// use std::any::TypeId;

		// match self.tag() {
		// 	Tag::Heap => unsafe { *self.as_ptr() }.downcast(),
		// 	Tag::NumberI32 if TypeId::of::<T>() == TypeId::of::<Number>()
		// 		=> Some(Number::from(i32::from_le_bytes(self.le_bytes()))),
		// 	Tag::NumberF32 if TypeId::of::<T>() == TypeId::of::<Number>()
		// 		=> Some(Number::from(f32::from_le_bytes(self.le_bytes()))),
		// 	_ => None
		// }
	// }
}

impl Debug for NaNBox {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let internal = self.read_internal();

		match internal.tag() {
			Tag::Heap => Debug::fmt(unsafe { &*internal.as_ptr() }, f),
			Tag::NumberI32 => Debug::fmt(&i32::from_le_bytes(internal.le_bytes()), f),
			Tag::NumberF32 => Debug::fmt(&f32::from_le_bytes(internal.le_bytes()), f)
		}
	}
}

impl Clone for NaNBox {
	fn clone(&self) -> Self {
		let internal = self.read_internal();

		match internal.tag() {
			Tag::Heap => unsafe {
				let data = HeapData::from_raw(internal.as_ptr());
				let dup = data.clone().into_raw();
				std::mem::forget(data); // we don't want the Arc to be dropped.

				Self::new_heap_internal(dup)
			},
			Tag::NumberI32 | Tag::NumberF32 => Self::new_internal(unsafe { internal.duplicate() })
		}
	}
}

impl Drop for NaNBox {
	fn drop(&mut self) {
		let internal = self.read_internal();

		match internal.tag() {
			Tag::Heap => drop(unsafe { HeapData::from_raw(internal.as_ptr()) }),
			Tag::NumberI32 | Tag::NumberF32 => { /* do nothing */ }
		}
	}
}

impl NaNBox {
	pub fn is_a<'a, T: ObjectType + 'a>(&self) -> bool {
		use std::any::TypeId;
		let internal = self.read_internal();

		match internal.tag() {
			Tag::NumberI32 | Tag::NumberF32 if TypeId::of::<T>() == TypeId::of::<Number>() => true,
			Tag::Heap => unsafe { &*internal.as_ptr() }.is_a::<T>(),
			_ => false
		}
	}


	pub fn is_identical(&self, rhs: &Self) -> bool {
		let lhs = self.read_internal();
		let rhs = rhs.read_internal();

		if lhs.tag() != rhs.tag() {
			return false;
		}

		match lhs.tag() {
			Tag::Heap => lhs.as_ptr() == rhs.as_ptr(),
			Tag::NumberI32 | Tag::NumberF32 => lhs.le_bytes() == rhs.le_bytes()
		}
	}

	pub fn deep_clone(&self) -> Self {
		let internal = self.read_internal();

		match internal.tag() {
			Tag::Heap => unsafe {
				let data = HeapData::from_raw(internal.as_ptr());
				let dup = data.deep_clone();
				std::mem::forget(data); // we could probably reorganize it so we don't need to forget.
				Self::new_heap_internal(dup.into_raw())
			},
			Tag::NumberF32 | Tag::NumberI32 => Self::new_internal(unsafe { internal.duplicate() })
		}
	}
}

pub unsafe trait NanBoxable : 'static {
	fn into_nanbox(self) -> NaNBox;
	fn from_nanbox(nanbox: &NaNBox) -> Option<&Self>;
}

impl NaNBox {
	#[inline]
	pub fn downcast<'a, T: NanBoxable>(&'a self) -> Option<impl Deref<Target=T> + 'a> {
		T::from_nanbox(self)
	}

	#[inline]
	pub fn downcast_mut<'a, T: ObjectType>(&'a self) -> Option<impl DerefMut<Target=T> + 'a> {
		self.data.downcast_mut()
	}

	// #[inline]
	// pub fn has_lit(&self, attr: &str) -> crate::Result<bool> {
	// 	self.attrs.has_lit(attr)
	// }

	// #[inline]
	// pub fn get_lit(&self, attr: &str) -> crate::Result<Option<Value>> {
	// 	self.attrs.get_lit(attr)
	// }

	// #[inline]
	// pub fn set_lit(&self, attr: crate::Literal, value: Value) -> crate::Result<()> {
	// 	self.attrs.set_lit(attr, value);
	// 	Ok(())
	// }

	// #[inline]
	// pub fn del_lit(&self, attr: &str) -> crate::Result<Option<Value>> {
	// 	Ok(self.attrs.del_lit(attr))
	// }

	// #[inline]
	// pub fn has(&self, attr: &Object) -> crate::Result<bool> {
	// 	self.attrs.has(attr)
	// }

	// #[inline]
	// pub fn get(&self, attr: &Object) -> crate::Result<Option<Value>> {
	// 	self.attrs.get(attr)
	// }

	// #[inline]
	// pub fn set(&self, attr: Object, value: Value) -> crate::Result<()> {
	// 	self.attrs.set(attr, value)
	// }

	// #[inline]
	// pub fn del(&self, attr: &Object) -> crate::Result<Option<Value>> {
	// 	self.attrs.del(attr)
	// }

	// #[inline]
	// pub fn add_parent(&self, val: Object) -> crate::Result<()> {
	// 	self.attrs.add_parent(val)
	// }

	// #[inline]
	// pub fn keys(&self, include_parents: bool) -> crate::Result<Vec<Object>> {
	// 	self.attrs.keys(include_parents)
	// }

}

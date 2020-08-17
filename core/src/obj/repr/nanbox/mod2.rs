#![allow(unused)]
mod tag;
mod inner;
pub use tag::Tag;
use inner::{Inner, Data};

#[cfg(target_endian="big")]
compile_error!("big endian not supported");

use std::fmt::{self, Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU64, Ordering};
use crate::types::{ObjectType, Number};
use std::cell::UnsafeCell;
use std::mem;

#[repr(C, align(8))]
pub struct NaNBox {
	lock: parking_lot::Mutex<()>,
	inner: UnsafeCell<Inner>
}

unsafe impl Send for NaNBox {}
unsafe impl Sync for NaNBox {}

impl Clone for NaNBox {
	fn clone(&self) -> Self {
		let guard = self.read();

		unsafe {
			match guard.tag() {
				Tag::Heap => {
					let data = super::HeapData::from_raw(guard.data().as_ptr());
					let dup = Data::from_ptr(data.clone().into_raw());
					mem::forget(data); // we don't want the Arc to be dropped.

					Self::new(Tag::Heap, dup)
				},
				tag @ Tag::NumberI32
					| tag @ Tag::NumberF32 => Self::new(tag, guard.data().duplicate()),
			}
		}
	}
}

impl Debug for NaNBox {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let guard = self.read();

		unsafe {
			match guard.tag() {
				Tag::Heap => Debug::fmt(&*guard.data().as_ptr(), f),
				Tag::NumberI32 => Debug::fmt(&mem::transmute::<u32, i32>(guard.data().lower_u32()), f),
				Tag::NumberF32 => Debug::fmt(&f32::from_bits(guard.data().lower_u32()), f),
			}
		}
	}
}

impl NaNBox {
	pub fn new_heap<T: ObjectType>(data: T) -> Self {
		Self::new(Tag::Heap, Data::from_ptr(super::HeapData::new(data).into_raw()))
	}

	fn new(tag: Tag, data: Data) -> Self {
		const _ASSERT_SIZES_SAME: [(); 0] = [(); 
			!(std::mem::size_of::<NaNBox>() == std::mem::size_of::<u64>()) as usize];

		Self {
			lock: Default::default(),
			inner: UnsafeCell::new(Inner::new(tag, data))
		}
	}

	fn read<'a>(&'a self) -> impl Deref<Target=Inner> + 'a {
		struct Reader<'a>(&'a Inner, parking_lot::MutexGuard<'a, ()>);

		impl Deref for Reader<'_> {
			type Target = Inner;

			#[inline]
			fn deref(&self) -> &Self::Target { self.0 }
		}

		let lock = self.lock.lock();
		Reader(unsafe { &*self.inner.get() }, lock)
	}

	fn write<'a>(&'a self) -> impl DerefMut<Target=Inner> + 'a  {
		struct Writer<'a>(&'a mut Inner, parking_lot::MutexGuard<'a, ()>);

		impl Deref for Writer<'_> {
			type Target = Inner;

			#[inline]
			fn deref(&self) -> &Self::Target { self.0 }
		}

		impl DerefMut for Writer<'_> {
			#[inline]
			fn deref_mut(&mut self) -> &mut Self::Target { self.0 }
		}

		let lock = self.lock.lock();
		Writer(unsafe { &mut *self.inner.get() }, lock)
	}
}



impl NaNBox {

	pub fn is_a<'a, T: crate::types::ObjectType + 'a>(&self) -> bool {
		use std::any::TypeId;
		let guard = self.read();

		match guard.tag() {
			Tag::NumberI32 | Tag::NumberF32 if TypeId::of::<T>() == TypeId::of::<Number>() => true,
			Tag::Heap => unsafe { &*guard.data().as_ptr() }.is_a::<T>(),
			_ => false
		}
	}

	pub fn is_identical(&self, rhs: &Self) -> bool {
		let self_guard = self.read();
		let rhs_guard = rhs.read();

		if self_guard.tag() != rhs_guard.tag() {
			return false;
		}

		match self_guard.tag() {
			Tag::Heap => unsafe { self_guard.data().as_ptr() == rhs_guard.data().as_ptr() },
			Tag::NumberI32 | Tag::NumberF32
				=> self_guard.data().lower_u32() == rhs_guard.data().lower_u32(),
		}
	}

	pub fn deep_clone(&self) -> Self {
		let guard = self.read();
		if guard.tag() == Tag::Heap {
			let ptr = unsafe {
				super::HeapData::from_raw(guard.data().as_ptr()).deep_clone().into_raw()
			};

			Self::new(Tag::Heap, Data::from_ptr(ptr))
		} else {
			Self::new(guard.tag(), unsafe { guard.data().duplicate() })
		}
	}
}

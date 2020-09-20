#![allow(unused)]

use std::sync::Arc;
use std::ops::{Deref, DerefMut};
use std::cell::UnsafeCell;
use parking_lot::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tag(u8);

impl Tag {
	const HEAP: Self = Self(0);
	const LITERAL: Self = Self(0b1000_0000);
}

#[repr(C)]
pub struct NanBox {
	mutex: Mutex<()>,
	inner: UnsafeCell<TaggedData>
}

#[repr(C)]
pub struct TaggedData {
	tag: Tag,
	data: [u8; 6]
}

impl NanBox {
	pub fn new_heap<T>(data: T) -> Self {
		let [a, b, data @ ..] = (Box::into_raw(Box::new(data)) as u64).to_ne_bytes();
		assert_eq!(a | b, 0, "upper bytes aren't zero?");

		Self {
			mutex: Mutex::new(()),
			inner: UnsafeCell::new(TaggedData { tag: Tag::HEAP, data })
		}
	}
}

impl TaggedData {
	#[inline]
	pub const fn tag(&self) -> Tag {
		self.tag
	}

	#[inline]
	pub const fn data(&self) -> [u8; 6] {
		self.data
	}
}

pub unsafe trait NaNBoxable : Sized + 'static {
	type Target;
	fn deref_inner(tagged_data: &TaggedData) -> Option<&Self::Target>;
	fn deref_inner_mut(tagged_data: &mut TaggedData) -> Option<&mut Self::Target>;
	fn into_nanbox(self) -> NanBox;
}

struct DowncastRef<L, T>(L, T);

impl<L, T: Deref> Deref for DowncastRef<L, T> {
	type Target = <T as Deref>::Target;

	fn deref(&self) -> &Self::Target {
		&self.1
	}
}

impl<L, T: DerefMut> DerefMut for DowncastRef<L, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.1
	}
}


impl NanBox {
	fn downcast<'a, T: NaNBoxable>(&'a self) -> Option<impl Deref<Target=<T as NaNBoxable>::Target> + 'a> {
		let lock = self.mutex.lock();
		let data = T::deref_inner(unsafe { &*self.inner.get() })?;

		Some(DowncastRef(lock, data))
	}

	fn downcast_mut<'a, T: NaNBoxable>(&'a self) -> Option<impl DerefMut<Target=<T as NaNBoxable>::Target> + 'a> {
		let lock = self.mutex.lock();
		let data = T::deref_inner_mut(unsafe { &mut *self.inner.get() })?;

		Some(DowncastRef(lock, data))
	}
}

unsafe impl NaNBoxable for crate::Literal {
	type Target = Self;
	fn deref_inner(tagged_data: &TaggedData) -> Option<&Self::Target> {
		#[repr(C)]
		struct Xform(u8, u8, [u8; 6]);

		if (tagged_data.tag().0 & Tag::LITERAL.0) == 0 {
			return None;
		}

		unsafe {
			let ptr: *const u8 = std::mem::transmute(Xform(0, 0, tagged_data.data()));
			let len = (tagged_data.tag().0 - Tag::LITERAL.0) as usize;

			let literal = std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len));

			Some(std::mem::transmute::<&&'static str, &Self>(&literal))
		}
	}

	fn deref_inner_mut(inner: &mut TaggedData) -> Option<&mut Self::Target> {
		unimplemented!()
	}

	fn into_nanbox(self) -> NanBox {
		unimplemented!()
	}
}

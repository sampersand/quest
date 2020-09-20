#![allow(unused)]

use std::sync::Arc;
use std::ops::{Deref, DerefMut};
use std::cell::UnsafeCell;
use parking_lot::Mutex;

mod mini_rwlock;
mod tag;
use mini_rwlock::MiniRwLock;
use tag::Tag;

#[repr(C)]
pub struct NaNBox {
	lock: MiniRwLock,
	tag: Tag,
	_data: [u8; 6]
}

impl NaNBox {
	#[inline]
	pub fn new(tag: Tag, data: [u8; 6]) -> Self {
		Self { lock: MiniRwLock::default(), tag, _data: data }
	}

	pub fn new_heap<T>(data: T) -> Self {
		unimplemented!()
	}

	unsafe fn as_ptr<T>(&self) -> *const T {
		unsafe {
			(*(self as *const Self as *const u64) & 0xffff_ffff_ffff) as *const T
		}
	}
}


pub unsafe trait NaNBoxable : Into<NaNBox> {
	type Target;

	unsafe fn deref_inner(nanbox: &NaNBox) -> Option<Self::Target>;
}

fn convert_ptr<T>(ptr: *const T) -> [u8; 6] {
	let [a, b, data @ ..] = (ptr as u64).to_le_bytes();
	assert_eq!(a | b, 0, "upper two bytes aren't zero? (a={:x}, b={:x})", a, b);
	data
}

impl From<crate::Literal> for NaNBox {
	fn from(lit: crate::Literal) -> Self {
		let lit = lit.into_inner();

		if lit.len() > Tag::MAX_EMBEDDED_LITERAL_LEN {
			Self::new(Tag::LITERAL_HEAP_ALLOC, convert_ptr(Box::into_raw(Box::new(lit))))
		} else {
			Self::new(Tag(Tag::LITERAL_EMBEDDED.0 | lit.len() as u8), convert_ptr(lit.as_ptr()))
		}
	}
}

unsafe impl NaNBoxable for crate::Literal {
	type Target = Self;
	unsafe fn deref_inner(nanbox: &NaNBox) -> Option<Self::Target> {
		let lit =
			if nanbox.tag.is(Tag::LITERAL_HEAP_ALLOC) {
				*nanbox.as_ptr::<&'static str>()
			} else if nanbox.tag.is(Tag::LITERAL_EMBEDDED) {
				let len = (nanbox.tag.0 - Tag::LITERAL_EMBEDDED.0) as usize;
				let ptr = nanbox.as_ptr::<u8>();
				std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len))
			} else {
				return None;
			};

		Some(Self::new(lit))
	}

	unsafe fn deref_inner_mut(nanbox: &NaNBox) -> Option<Self::Target> {
		let lit = 
			if 
	}
}
// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct Tag(u8);

// impl Tag {
// 	const HEAP: Self = Self(0);
// 	const LITERAL: Self = Self(0b1000_0000);
// }

// #[repr(C)]
// pub struct NanBox {
// 	mutex: Mutex<()>,
// 	inner: UnsafeCell<TaggedData>
// }

// #[repr(C)]
// pub struct TaggedData {
// 	tag: Tag,
// 	data: [u8; 6]
// }

// impl NanBox {
// 	pub fn new_heap<T>(data: T) -> Self {
// 		let [a, b, data @ ..] = (Box::into_raw(Box::new(data)) as u64).to_ne_bytes();
// 		assert_eq!(a | b, 0, "upper bytes aren't zero?");

// 		Self {
// 			mutex: Mutex::new(()),
// 			inner: UnsafeCell::new(TaggedData { tag: Tag::HEAP, data })
// 		}
// 	}
// }

// impl TaggedData {
// 	#[inline]
// 	pub const fn tag(&self) -> Tag {
// 		self.tag
// 	}

// 	#[inline]
// 	pub const fn data(&self) -> [u8; 6] {
// 		self.data
// 	}
// }

// pub unsafe trait NaNBoxable : Sized + 'static {
// 	type Target;
// 	fn deref_inner(tagged_data: &TaggedData) -> Option<&Self::Target>;
// 	fn deref_inner_mut(tagged_data: &mut TaggedData) -> Option<&mut Self::Target>;
// 	fn into_nanbox(self) -> NanBox;
// }

// struct DowncastRef<L, T>(L, T);

// impl<L, T: Deref> Deref for DowncastRef<L, T> {
// 	type Target = <T as Deref>::Target;

// 	fn deref(&self) -> &Self::Target {
// 		&self.1
// 	}
// }

// impl<L, T: DerefMut> DerefMut for DowncastRef<L, T> {
// 	fn deref_mut(&mut self) -> &mut Self::Target {
// 		&mut self.1
// 	}
// }


// impl NanBox {
// 	fn downcast<'a, T: NaNBoxable>(&'a self) -> Option<impl Deref<Target=<T as NaNBoxable>::Target> + 'a> {
// 		let lock = self.mutex.lock();
// 		let data = T::deref_inner(unsafe { &*self.inner.get() })?;

// 		Some(DowncastRef(lock, data))
// 	}

// 	fn downcast_mut<'a, T: NaNBoxable>(&'a self) -> Option<impl DerefMut<Target=<T as NaNBoxable>::Target> + 'a> {
// 		let lock = self.mutex.lock();
// 		let data = T::deref_inner_mut(unsafe { &mut *self.inner.get() })?;

// 		Some(DowncastRef(lock, data))
// 	}
// }

// unsafe impl NaNBoxable for crate::Literal {
// 	type Target = Self;
// 	fn deref_inner(tagged_data: &TaggedData) -> Option<&Self::Target> {
// 		#[repr(C)]
// 		struct Xform(u8, u8, [u8; 6]);

// 		if (tagged_data.tag().0 & Tag::LITERAL.0) == 0 {
// 			return None;
// 		}

// 		unsafe {
// 			let ptr: *const u8 = std::mem::transmute(Xform(0, 0, tagged_data.data()));
// 			let len = (tagged_data.tag().0 - Tag::LITERAL.0) as usize;

// 			let literal = std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len));

// 			Some(std::mem::transmute::<&&'static str, &Self>(&literal))
// 		}
// 	}

// 	fn deref_inner_mut(inner: &mut TaggedData) -> Option<&mut Self::Target> {
// 		unimplemented!()
// 	}

// 	fn into_nanbox(self) -> NanBox {
// 		unimplemented!()
// 	}
// }

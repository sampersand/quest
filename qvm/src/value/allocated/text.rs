use crate::value::{Value, allocated::ALLOC_TYPE_SIZE};
use std::mem::size_of;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::UnsafeCell;
use std::fmt::{self, Debug, Display, Formatter};

/// The type that represents Text within Quest.
#[derive(Named)]
#[quest(crate_name="crate")]
#[repr(C)]
pub struct Text {
	/// The data for this [`Text`]. Since we need to be able to convert around internal representations,
	/// we need it to be an [`UnsafeCell`].
	data: UnsafeCell<TextData>
}

/// Because allocating new memory for every string is expensive, we _also_ have `embed`, which
/// allows us to embed small strings directly into the struct itself.
///
/// Note that each type starts with a `usize` len, and are `#[repr(C)]`. Since we only ever allow a maximum of
/// `isize::MAX` bytes, and you can't have negative size, the sign bit is actually used to indicate if the data is
/// embedded or not.
#[repr(C)]
union TextData {
	shared: TextDataShared,
	owned: TextDataOwned,
	embed: TextDataEmbed
}

/// The flag that indicates whether something is embedded or not.
///
/// This is set on `len`. If it is embedded.
const EMBEDDED_FLAG: usize = 1 << (8 * std::mem::size_of::<usize>() - 1);

/// Text that's shared between multiple [`Text`]s.
///
/// Data here cannot be modified and requires a copy-on-write to be changed.
#[derive(Clone, Copy)]
#[repr(C)]
struct TextDataShared {
	/// The length of the shared region.
	len: usize,

	/// A pointer to the shared region.
	ptr: *const u8,

	/// A pointer to a reference count for this piece of text.
	refcount: *const AtomicUsize,

	/// Whether this data is shared. Both this and [`TextDataShared`] have it defined in the same spot.
	is_shared: bool
}

/// Text that's been allocated on the heap and exclusively owned by us.
///
/// If the length ever goes below `MAX_EMBED_SIZE`, we still keep it allocated here.
#[derive(Clone, Copy)]
#[repr(C)]
struct TextDataOwned {
	/// The length of this owned region.
	len: usize,

	/// A pointer to this region.
	ptr: *mut u8,

	/// The capacity of the owned region.
	cap: usize,

	/// Whether this data is shared. Both this and [`TextDataOwned`] have it defined in the same spot.
	is_shared: bool
}

sa::assert_eq_size!(usize, *const AtomicUsize);

/// The maximum size an embedded piece of text can be.
const MAX_EMBED_SIZE: usize = ALLOC_TYPE_SIZE - size_of::<usize>();

#[derive(Clone, Copy)]
#[repr(C, align(8))]
struct TextDataEmbed {
	/// The length of the embedded data.
	///
	/// Even though a `usize` wastes quite a few bytes, it means our `len` fetching is consistent.
	len: usize,

	/// The embedded data itself.
	text: [u8; MAX_EMBED_SIZE]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TextType {
	Embedded,
	Shared,
	Owned
}

impl Text {
	/// Creates a new embedded string.
	fn new_embed() -> Self {
 		Self {
			data: UnsafeCell::new(TextData {
				embed: TextDataEmbed {
					len: EMBEDDED_FLAG,
					text: [0; MAX_EMBED_SIZE]
				}
			})
		}
	}

	/// Allocates `cap` bytes.
	fn new_owned(cap: usize) -> Self {
		Self {
			data: UnsafeCell::new(TextData {
				owned: TextDataOwned {
					len: 0,
					cap,
					ptr: crate::alloc::alloc(cap),
					is_shared: false
				}
			})
		}
	}

	/// Creates a new [`Text`] with a capacity of at least `cap` bytes.
	pub fn with_capacity(cap: usize) -> Self {
		if cap <= MAX_EMBED_SIZE {
			Self::new_embed()
		} else {
			Self::new_owned(cap)
		}
	}

	/// Creates a new [`Text`] from the given input.
	pub fn new<T: Into<Self>>(source: T) -> Self {
		source.into()
	}

	/// Gets the length of `self`.
	#[inline]
	pub fn len(&self) -> usize {
		// get rid of the embedded flag.
		self._len() & !EMBEDDED_FLAG
	}

	/// Sets the length of this type
	///
	/// # Safety
	/// You must ensure that the length is within the embedded/allocated region
	#[inline]
	pub unsafe fn set_len(&mut self, mut len: usize) {
		debug_assert_eq!(self.is_embeded(), len <= MAX_EMBED_SIZE);

		if len <= MAX_EMBED_SIZE {
			len |= EMBEDDED_FLAG;
		}

		// SAFETY: every type has length in the same position, so we can pick any one.
		unsafe {
			(*self.data.get()).shared.len = len;
		} 
	}

	/// checks to see if the `text` is empty.
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Gets the capacity of `self`, returning `None` if we're a shared type.
	pub fn capacity(&self) -> Option<usize> {
		if self.is_embeded() {
			Some(MAX_EMBED_SIZE)
		} else if self.is_shared() {
			None
		} else {
			Some(unsafe { self.assume_owned().cap })
		}
	}

	fn classify(&self) -> TextType {
		if self.is_embeded() {
			TextType::Embedded
		} else if self.is_shared() {
			TextType::Shared
		} else {
			TextType::Owned
		}
	}

	/// Returns a mutable pointer to start of `self`'s data.
	///
	/// # Safety
	/// In addition to normal safety rules for pointers, you must also ensure that you don't
	/// write more than `self.capacity()` bytes of data.
	pub fn as_ptr_mut(&mut self) -> *mut u8 {
		// SAFETY: (todo, it's just UnsafeCell)
		unsafe {
			match self.classify() {
				TextType::Embedded => &mut self.assume_embed_mut().text as *mut _ as *mut u8,
				TextType::Owned => self.assume_owned().ptr,
				TextType::Shared => {
					*self = self.clone();
					self.assume_owned().ptr
				},
			}
		}
	}

	/// Returns a pointer to start of `self`'s data.
	pub fn as_ptr(&self) -> *const u8 {
		// SAFETY: (todo, it's just UnsafeCell)
		unsafe {
			match self.classify() {
				TextType::Embedded => &self.assume_embed().text as *const _ as *const u8,	
				TextType::Owned => self.assume_owned().ptr as *const u8,
				TextType::Shared => self.assume_shared().ptr
			}
		}
	}

	/// Assumes `self` is a [`TextDataEmbed`], and returns a copy of it.
	unsafe fn assume_embed(&self) -> &TextDataEmbed {
		debug_assert!(self.is_embeded());
		&(*self.data.get()).embed
	}

	/// Assumes `self` is a [`TextDataEmbed`], and returns a copy of it.
	unsafe fn assume_embed_mut(&mut self) -> &mut TextDataEmbed {
		debug_assert!(self.is_embeded());
		&mut (*self.data.get()).embed
	}

	/// Assumes `self` is a [`TextDataShared`], and returns a copy of it.
	unsafe fn assume_shared(&self) -> &TextDataShared {
		debug_assert!(self.is_shared());
		&(*self.data.get()).shared
	}

	/// Assumes `self` is a [`TextDataOwned`], and returns a copy of it.
	unsafe fn assume_owned(&self) -> &TextDataOwned {
		debug_assert!(self.is_owned());
		&(*self.data.get()).owned
	}

	// Gets the actual number associated with length, which also includes the embedded bit.
	fn _len(&self) -> usize {
		// SAFETY: every type has length in the same position, so we can pick any one.
		unsafe { (*self.data.get()).shared.len } 
	}

	#[inline]
	fn is_embeded(&self) -> bool {
		// We use the sign bit to indicate "embeddedness", so if the value's negative, it has the sign bit set.
		(self._len() as isize) < 0
	}

	#[inline]
	fn is_shared(&self) -> bool {
		// SAFETY: both shared and owned have the `is_shared` field defined in the same spot, so we can pick either.
		!self.is_embeded() && unsafe { (*self.data.get()).shared.is_shared }
	}

	#[inline]
	fn is_owned(&self) -> bool {
		!self.is_embeded() && !self.is_shared()
	}
}

impl Debug for Text {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let mut dbg = f.debug_tuple("Text");

		if let Ok(string) = std::str::from_utf8(self.as_ref()) {
			dbg.field(&string)
		} else {
			dbg.field(&self.as_ref())
		};

		dbg.finish()
	}
}

impl From<&'_ [u8]> for Text {
	#[inline]
	fn from(data: &[u8]) -> Self {
		let mut this = Self::with_capacity(data.len());

		// SAFETY:
		// - `this` has at least `data.len()` bytes as we used `with_capacity`.
		// - `data` has length `data.len()` tautologically.
		// - `u8` has an alignment of 1.
		// - They do not overlap, as we just allocated memory.
		unsafe {
			this.as_ptr_mut().copy_from_nonoverlapping(data.as_ptr(), data.len());
			this.set_len(data.len());
		}

		this
	}
}

impl From<Vec<u8>> for Text {
	#[inline]
	fn from(data: Vec<u8>) -> Self {
		data.as_slice().into()
	}
}

impl From<String> for Text {
	#[inline]
	fn from(data: String) -> Self {
		data.as_bytes().into()
	}
}

impl From<&'_ str> for Text {
	#[inline]
	fn from(data: &str) -> Self {
		data.as_bytes().into()
	}
}

impl std::iter::Extend<u8> for Text {
	fn extend<I: IntoIterator<Item=u8>>(&mut self, iter: I) {
		todo!();
	}
}

impl std::iter::FromIterator<u8> for Text {
	fn from_iter<I: IntoIterator<Item=u8>>(iter: I) -> Self {
		let iter = iter.into_iter();
		let mut this =
			match iter.size_hint().1 {
				Some(num) if num <= MAX_EMBED_SIZE => Self::new_embed(),
				Some(len) => Self::new_owned(len),
				None => Self::new_owned(MAX_EMBED_SIZE + 1)
			};
		this.extend(iter);
		this
	}
}

impl Clone for Text {
	fn clone(&self) -> Self {
		match self.classify() {
			// SAFETY: Since `&self` is an immutable reference, and we're only 
			// accessing data, not changing it, the `.get()` here is sound.
			TextType::Embedded => unsafe {
				Self {
					data: UnsafeCell::new(TextData { embed: *self.assume_embed() })
				}
			},

			// SAFETY: Like `Embedded`, we only use immutable references.
			TextType::Shared => unsafe {
				// NOTE: .fetch_add is thread-safe, and takes an `&self`.
				(*self.assume_shared().refcount).fetch_add(1, Ordering::Relaxed);

				Self {
					data: UnsafeCell::new(TextData { shared: *self.assume_shared() })
				}
			},

			// SAFETY: Since `Owned` is only applicable when the object is uniquely owned by us,
			// and since we're not casting to mutable references (but instead are using `*mut` directly),
			// operations here are sound.
			// FUTURE: we _could_ shrink-to-fit here, but it might be more harm than it's worth. Benchmark it?
			TextType::Owned => unsafe {
				let refcount = crate::alloc::alloc(size_of::<AtomicUsize>()) as *mut AtomicUsize;
				std::ptr::write(refcount, AtomicUsize::new(2));
				let refcount = refcount as *const AtomicUsize;

				(*self.data.get()).shared =
					TextDataShared {
						len: self.assume_owned().len,
						ptr: self.assume_owned().ptr,
						refcount,
						is_shared: true
					};

				Self {
					data: UnsafeCell::new(TextData { shared: *self.assume_shared() })
				}
			}
		}
	}
}

impl Drop for Text {
	fn drop(&mut self) {
		// SAFETY: In each section, we only use values which are valid if the classification is right.
		match self.classify() {
			TextType::Embedded => { /* we're embedded, so nothing to drop. */ },
			TextType::Shared => unsafe {
				let is_last = 1 == (*self.assume_shared().refcount).fetch_sub(1, Ordering::Relaxed);
				// We're the last one if, before decrementing, there was one left.
				if is_last {
					crate::alloc::dealloc(self.assume_shared().refcount as *mut u8);
					crate::alloc::dealloc(self.assume_shared().ptr as *mut u8);
				}
			},
			// SAFETY: Since we own the pointer, we're able to just flat-out deallocate it.
			TextType::Owned => unsafe {
				crate::alloc::dealloc(self.assume_owned().ptr)
			}
		}
	}
}

impl AsRef<[u8]> for Text {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		// SAFETY: since we constructed it ourselves, we know it's valid.
		unsafe {
			std::slice::from_raw_parts(self.as_ptr(), self.len())
		}
	}
}

impl AsMut<[u8]> for Text {
	#[inline]
	fn as_mut(&mut self) -> &mut [u8] {
		// SAFETY: since we constructed it ourselves, we know it's valid.
		unsafe {
			std::slice::from_raw_parts_mut(self.as_ptr_mut(), self.len())
		}
	}
}

impl Eq for Text {}
impl PartialEq for Text {
	fn eq(&self, rhs: &Self) -> bool {
		self.as_ref() == rhs.as_ref()
	}
}

impl crate::ShallowClone for Text {
	fn shallow_clone(&self) -> crate::Result<Self> {
		Ok(self.clone())
	}
}

impl crate::DeepClone for Text {
	fn deep_clone(&self) -> crate::Result<Self> {
		Ok(self.clone())
	}
}

impl_allocated_type!(for Text);
impl_allocated_value_type_ref!(for Text);

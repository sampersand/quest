use crate::value::{Value, Basic, ALLOC_SIZE};
use std::mem::size_of;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::UnsafeCell;

/// The type that represents Array within Quest.
#[repr(C)]
pub struct Array {
	/// In case we need to swap over to a `Shared`, we need `Basic` to be an `UnsafeCell` too.
	basic: UnsafeCell<Basic>,

	/// The data for this [`Array`]. Since we need to be able to convert around internal representations,
	/// we need it to be an [`UnsafeCell`].
	data: UnsafeCell<ArrayData>
}

/// The text is shared.
const FLAG_SHARED: u8  = 0b00000001;

/// The text is embeded.
const FLAG_EMBEDED: u8 = 0b00000010;

/// Because allocating new memory for every string is so expensive, we _also_ have `embed`, which
/// allows us to embed small strings directly into the struct itself.
#[repr(C)]
union ArrayData {
	shared: ArrayDataShared,
	owned: ArrayDataOwned,
	embed: ArrayDataEmbed
}

/// Array that's shared between multiple [`Array`]s.
///
/// Data here cannot be modified and requires a copy-on-write to be changed.
#[repr(C)]
#[derive(Clone, Copy)]
struct ArrayDataShared {
	/// The length of the shared region.
	len: usize,

	/// A pointer to a reference count for this piece of text.
	refcount: *const AtomicUsize,

	/// A pointer to the shared region.
	ptr: *const Value
}

/// Array that's been allocated on the heap and exclusively owned by us.
///
/// If the length ever goes below `MAX_EMBED_SIZE`, we still keep it allocated here.
#[repr(C)]
#[derive(Clone, Copy)]
struct ArrayDataOwned {
	/// The length of this owned region.
	len: usize,

	/// The capacity of the owned region.
	cap: usize,

	/// A pointer to this region.
	ptr: *mut Value
}

/// The maximum size an embedded piece of text can be.
const MAX_EMBED_SIZE: usize = (ALLOC_SIZE - size_of::<Basic>() - size_of::<u8>()) / size_of::<Value>();

// more of a sanity check than anything, as it would require `ALLOC_SIZE` to be massive.
const_assert!(MAX_EMBED_SIZE <= u8::MAX as usize);

#[repr(C)]
#[derive(Clone, Copy)]
struct ArrayDataEmbed {
	/// The length of the embedded data.
	///
	/// It's a [`u8`] as our [`MAX_EMBED_SIZE`] is less than [`u8::MAX`].
	len: u8,

	/// The embedded data itself.
	eles: [Value; MAX_EMBED_SIZE],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ArrayType {
	Embedded,
	Shared,
	Owned
}

static PARENT: Value = Value::NULL;

impl Array {
	/// Creates a new embedded string.
	fn new_embed() -> Self {
 		Self {
			basic: UnsafeCell::new(Basic::new(PARENT, FLAG_EMBEDED)),
			data: UnsafeCell::new(ArrayData {
				embed: ArrayDataEmbed {
					len: 0,
					eles: [Value::NULL; MAX_EMBED_SIZE],
				}
			})
		}
	}

	/// Allocates at least `cap` amount of [`Value`]s.
	fn new_owned(cap: usize) -> Self {
		let cap = cap * size_of::<Value>();

		Self {
			basic: UnsafeCell::new(Basic::new(PARENT, 0)),
			data: UnsafeCell::new(ArrayData {
				owned: ArrayDataOwned {
					len: 0,
					cap,
					ptr: crate::alloc::alloc(cap) as *mut Value
				}
			})
		}
	}

	/// Creates a new [`Array`] with a capacity of at least `cap` bytes.
	pub fn new(cap: usize) -> Self {
		if cap <= MAX_EMBED_SIZE {
			Self::new_embed()
		} else {
			Self::new_owned(cap)
		}
	}

	fn classify(&self) -> ArrayType {
		if self.is_embeded() {
			ArrayType::Embedded
		} else if self.is_shared() {
			ArrayType::Shared
		} else {
			ArrayType::Owned
		}
	}

	/// Fetches the [`Basic`] value associated with self as a reference.
	///
	/// # Safety
	/// Since `basic`'s wrapped in an [`UnsafeCell`], its `.get` safety guarantees must be met.
	unsafe fn fetch_basic(&self) -> Basic {
		*self.basic.get()
	}

	/// Assumes `self` is a [`ArrayDataEmbed`], and returns a copy of it.
	unsafe fn assume_embed(&self) -> ArrayDataEmbed {
		debug_assert!(self.is_embeded());
		(*self.data.get()).embed
	}

	/// Assumes `self` is a [`ArrayDataShared`], and returns a copy of it.
	unsafe fn assume_shared(&self) -> ArrayDataShared {
		debug_assert!(self.is_shared());
		(*self.data.get()).shared
	}

	/// Assumes `self` is a [`ArrayDataOwned`], and returns a copy of it.
	unsafe fn assume_owned(&self) -> ArrayDataOwned {
		debug_assert!(self.is_owned());
		(*self.data.get()).owned
	}

	#[inline]
	fn is_embeded(&self) -> bool {
		// SAFETY: We don't modify it, and all mutable accesses are controlled, so this is ok
		unsafe {
			self.fetch_basic().flags() & FLAG_EMBEDED != 0
		}
	}

	#[inline]
	fn is_shared(&self) -> bool {
		// SAFETY: We don't modify it, and all mutable accesses are controlled, so this is ok
		unsafe {
			self.fetch_basic().flags() & FLAG_SHARED != 0
		}
	}

	#[inline]
	fn is_owned(&self) -> bool {
		!self.is_embeded() && !self.is_shared()
	}
}

impl Clone for Array {
	/// Shallowly clones the array, _not_ a deep clone.
	fn clone(&self) -> Self {
		match self.classify() {
			// SAFETY: Since `&self` is an immutable reference, and we're only 
			// accessing data, not changing it, the `.get()` here is sound.
			ArrayType::Embedded => unsafe {
				Self {
					basic: UnsafeCell::new(self.fetch_basic().clone()),
					data: UnsafeCell::new(ArrayData {
						embed: {
							// unlike in text, we have to clone them all in Array.
							let embed = self.assume_embed();
							let mut eles = [Value::NULL; MAX_EMBED_SIZE];
							for i in 0..MAX_EMBED_SIZE {
								eles[i] = embed.eles[i].clone();
							}

							ArrayDataEmbed { len: embed.len, eles }
						}
					})
				}
			},

			// SAFETY: Like `Embedded`, we only use immutable references.
			ArrayType::Shared => unsafe {
				// NOTE: .fetch_add is thread-safe, and takes an `&self`.
				(*self.assume_shared().refcount).fetch_add(1, Ordering::Relaxed);

				Self {
					basic: UnsafeCell::new(self.fetch_basic().clone()),
					data: UnsafeCell::new(ArrayData { shared: self.assume_shared() })
				}
			},

			// SAFETY: Since `Owned` is only applicable when the object is uniquely owned by us,
			// and since we're not casting to mutable references (but instead are using `*mut` directly),
			// operations here are sound.
			// FUTURE: we _could_ shrink-to-fit here, but it might be more harm than it's worth
			// benchmark it?
			ArrayType::Owned => unsafe {
				*(*self.basic.get()).flags_mut() |= FLAG_SHARED;
				// *self.basic.flags_mut() |= FLAG_SHARED;

				let refcount = crate::alloc::alloc(size_of::<AtomicUsize>()) as *mut AtomicUsize;
				*refcount = AtomicUsize::new(2);
				let refcount = refcount as *const AtomicUsize;

				*self.data.get() = ArrayData {
					shared: ArrayDataShared {
						refcount,
						len: self.assume_owned().len,
						ptr: self.assume_owned().ptr,
					}
				};

				Self {
					basic: UnsafeCell::new(self.fetch_basic().clone()),
					data: UnsafeCell::new(ArrayData { shared: self.assume_shared() })
				}
			}
		}
	}
}

impl Drop for Array {
	fn drop(&mut self) {
		unsafe fn drop_ary(ptr: *mut Value, len: usize) {
			debug_assert!(len < isize::MAX as usize);

			for i in 0..len {
				std::ptr::drop_in_place(ptr.offset(i as isize));
			}
		}

		// SAFETY: In each section, we only use values which are valid if the classification is right.
		match self.classify() {
			ArrayType::Embedded => { /* these will be dropped automatically for us */ }
			ArrayType::Shared => unsafe {
				let shared = self.assume_shared();
				let is_last = 1 == (*shared.refcount).fetch_sub(1, Ordering::Relaxed);

				// We're the last one if, before decrementing, there was one left.
				if is_last {
					crate::alloc::dealloc(shared.refcount as *mut u8);

					drop_ary(shared.ptr as *mut Value, shared.len);
					crate::alloc::dealloc(shared.ptr as *mut u8);
				}
			},
			// SAFETY: Since we own the pointer, we're able to just flat-out deallocate it.
			ArrayType::Owned => unsafe {
				let owned = self.assume_owned();
				drop_ary(owned.ptr as *mut Value, owned.len);
				crate::alloc::dealloc(owned.ptr as *mut u8);
			}
		}
	}
}

impl Array {
	/// Gets the length of `self`.
	pub fn len(&self) -> usize {
		unsafe {
			match self.classify() {
				ArrayType::Embedded => self.assume_embed().len as usize,
				ArrayType::Shared => self.assume_shared().len,
				ArrayType::Owned => self.assume_owned().len,
			}
		}
	}

	/// checks to see if the `text` is empty.
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}
}

use crate::value::{Value, Basic, ALLOC_SIZE};
use std::mem::size_of;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::UnsafeCell;

/// The type that represents Text within Quest.
#[repr(C)]
pub struct Text {
	/// In case we need to swap over to a `Shared`, we need `Basic` to be an `UnsafeCell` too.
	basic: UnsafeCell<Basic>,
	/// The data for this [`Text`]. Since we need to be able to convert around internal representations,
	/// we need it to be an [`UnsafeCell`].
	data: UnsafeCell<TextData>
}

bitflags! {
	struct Flags : u8 {
		/// The text is shared.
		const SHARED = 1;
		/// The text is embeded.
		const EMBEDED = 2;
	}
}


/// Because allocating new memory for every string is so expensive, we _also_ have `embed`, which
/// allows us to embed small strings directly into the struct itself.
#[repr(C)]
union TextData {
	shared: TextDataShared,
	owned: TextDataOwned,
	embed: TextDataEmbed
}

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
	refcount: *const AtomicUsize
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
	cap: usize
}

/// The maximum size an embedded piece of text can be.
const MAX_EMBED_SIZE: usize = ALLOC_SIZE - size_of::<Basic>() - size_of::<usize>();

#[derive(Clone, Copy)]
#[repr(C)]
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

static PARENT: Value = Value::NULL;

impl Text {
	/// Creates a new embedded string.
	fn new_embed() -> Self {
 		Self {
			basic: UnsafeCell::new(Basic::new(PARENT.clone(), Flags::EMBEDED.bits())),
			data: UnsafeCell::new(TextData {
				embed: TextDataEmbed {
					len: 0,
					text: [0; MAX_EMBED_SIZE]
				}
			})
		}
	}

	/// Allocates `cap` bytes.
	fn new_owned(cap: usize) -> Self {
		Self {
			basic: UnsafeCell::new(Basic::new(PARENT.clone(), 0)),
			data: UnsafeCell::new(TextData {
				owned: TextDataOwned {
					len: 0,
					cap,
					ptr: crate::alloc::alloc(cap)
				}
			})
		}
	}

	/// Creates a new [`Text`] with a capacity of at least `cap` bytes.
	pub fn new(cap: usize) -> Self {
		if cap <= MAX_EMBED_SIZE {
			Self::new_embed()
		} else {
			Self::new_owned(cap)
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

	/// Fetches the [`Basic`] value associated with self as a reference.
	///
	/// # Safety
	/// Since `basic`'s wrapped in an [`UnsafeCell`], its `.get` safety guarantees must be met.
	unsafe fn fetch_basic(&self) -> &Basic {
		&*self.basic.get()
	}

	/// Fetches the [`Basic`] value associated with self as a reference.
	///
	/// # Safety
	/// Since `basic`'s wrapped in an [`UnsafeCell`], its `.get` safety guarantees must be met.
	unsafe fn flags(&self) -> Flags {
		let bits = self.fetch_basic().flags();
		debug_assert!(Flags::from_bits(bits).is_some());

		// SAFETY: Since this class controls the bits, they should never be invalid
		unsafe {
			Flags::from_bits_unchecked(bits)
		}
	}

	/// Assumes `self` is a [`TextDataEmbed`], and returns a copy of it.
	unsafe fn assume_embed(&self) -> TextDataEmbed {
		debug_assert!(self.is_embeded());
		(*self.data.get()).embed
	}

	/// Assumes `self` is a [`TextDataShared`], and returns a copy of it.
	unsafe fn assume_shared(&self) -> TextDataShared {
		debug_assert!(self.is_shared());
		(*self.data.get()).shared
	}

	/// Assumes `self` is a [`TextDataOwned`], and returns a copy of it.
	unsafe fn assume_owned(&self) -> TextDataOwned {
		debug_assert!(self.is_owned());
		(*self.data.get()).owned
	}

	#[inline]
	fn is_embeded(&self) -> bool {
		// SAFETY: We don't modify it, and all mutable accesses are controlled, so this is ok
		unsafe { self.flags() }.contains(Flags::EMBEDED)
	}

	#[inline]
	fn is_shared(&self) -> bool {
		// SAFETY: We don't modify it, and all mutable accesses are controlled, so this is ok
		unsafe { self.flags() }.contains(Flags::SHARED)
	}

	#[inline]
	fn is_owned(&self) -> bool {
		!self.is_embeded() && !self.is_shared()
	}
}

impl Clone for Text {
	fn clone(&self) -> Self {
		match self.classify() {
			// SAFETY: Since `&self` is an immutable reference, and we're only 
			// accessing data, not changing it, the `.get()` here is sound.
			TextType::Embedded => unsafe {
				Self {
					basic: UnsafeCell::new(self.fetch_basic().clone()),
					data: UnsafeCell::new(TextData { embed: self.assume_embed() })
				}
			},

			// SAFETY: Like `Embedded`, we only use immutable references.
			TextType::Shared => unsafe {
				// NOTE: .fetch_add is thread-safe, and takes an `&self`.
				(*self.assume_shared().refcount).fetch_add(1, Ordering::Relaxed);

				Self {
					basic: UnsafeCell::new(self.fetch_basic().clone()),
					data: UnsafeCell::new(TextData { shared: self.assume_shared() })
				}
			},

			// SAFETY: Since `Owned` is only applicable when the object is uniquely owned by us,
			// and since we're not casting to mutable references (but instead are using `*mut` directly),
			// operations here are sound.
			// FUTURE: we _could_ shrink-to-fit here, but it might be more harm than it's worth
			// benchmark it?
			TextType::Owned => unsafe {
				*(*self.basic.get()).flags_mut() |= Flags::SHARED.bits();
				// *self.basic.flags_mut() |= FLAG_SHARED;

				let refcount = crate::alloc::alloc(size_of::<AtomicUsize>()) as *mut AtomicUsize;
				*refcount = AtomicUsize::new(2);
				let refcount = refcount as *const AtomicUsize;

				*self.data.get() = TextData {
					shared: TextDataShared {
						refcount,
						len: self.assume_owned().len,
						ptr: self.assume_owned().ptr,
					}
				};

				Self {
					basic: UnsafeCell::new(self.fetch_basic().clone()),
					data: UnsafeCell::new(TextData { shared: self.assume_shared() })
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
	fn as_ref(&self) -> &[u8] {
		match self.classify() {
			// SAFETY: since we constructed it ourselves, we know it's valid.
			TextType::Embedded => unsafe {
				let TextDataEmbed { len, ref text } = self.assume_embed();
				std::slice::from_raw_parts(text as *const u8, len as usize)
			},
			TextType::Owned => unsafe {
				let TextDataOwned { len, ptr, .. } = self.assume_owned();
				std::slice::from_raw_parts(ptr, len)
			},
			TextType::Shared => unsafe {
				let TextDataShared { len, ptr, .. } = self.assume_shared();
				std::slice::from_raw_parts(ptr, len)
			},
		}
	}
}

impl AsMut<[u8]> for Text {
	fn as_mut(&mut self) -> &mut [u8] {
		match self.classify() {
			// SAFETY: since we constructed it ourselves, we know it's valid.
			TextType::Embedded => unsafe {
				let TextDataEmbed { len, ref mut text } = self.assume_embed();
				std::slice::from_raw_parts_mut(text as *mut [u8] as *mut u8, len as usize)
			},
			TextType::Owned => unsafe {
				let TextDataOwned { len, ptr, .. } = self.assume_owned();
				std::slice::from_raw_parts_mut(ptr, len)
			},
			TextType::Shared => unsafe {
				*self = self.clone();
				self.as_mut()
			},
		}
	}
}


impl Text {
	/// Gets the length of `self`.
	pub fn len(&self) -> usize {
		unsafe {
			match self.classify() {
				TextType::Embedded => self.assume_embed().len as usize,
				TextType::Shared => self.assume_shared().len,
				TextType::Owned => self.assume_owned().len,
			}
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
}

// type Error = u8;

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct TextClass;

// impl TextClass {
// 	pub fn initialize() -> Self {

// 	}
// }

// impl Value {
// 	/// Converts `self` to a Text. This simply clones `self`.
// 	pub fn qs_at_text(&self, _: &[&Value]) -> Result<Value, Error> {
// 		Ok(self.clone().into())
// 	}
// }

	// "@text" => method Self::qs_at_text,
	// "@regex" => method Self::qs_at_regex,
	// "inspect"  => method Self::qs_inspect,
	// "@num"    => method Self::qs_at_num,
	// "@list"   => method Self::qs_at_list,
	// "@bool"   => method Self::qs_at_bool,
	// "@iter"   => method Self::qs_at_iter,
	// "()"      => method Self::qs_call,

	// "~"       => method Self::qs_bitnot,
	// "="       => method Self::qs_assign,
	// "->"      => method Self::qs_arrow,
	// "<=>"     => method Self::qs_cmp,
	// "=="      => method Self::qs_eql,
	// "+"       => method Self::qs_add,
	// "+="      => method Self::qs_add_assign,

	// "len"     => method Self::qs_len,
	// "get"     => method Self::qs_get,
	// "[]"      => method Self::qs_get,
	// "[]="  => method |this, args| {
	// 	let mut arg = args.try_arg(0)?.downcast_mut::<crate::types::List>().expect("`[]=` called without List.");
	// 	arg.push(args.try_arg(1)?.clone());

	// 	Self::qs_set(this, arg.as_ref().iter().collect())
	// },
	// "set"     => method Self::qs_set,
	// "push"    => method Self::qs_push,
	// "pop"     => method Self::qs_pop,
	// "unshift" => method Self::qs_unshift,
	// "shift"   => method Self::qs_shift,
	// "clear"   => method Self::qs_clear,
	// "split"   => method Self::qs_split,
	// "reverse" => method Self::qs_reverse, 
	// "strip"   => method Self::qs_strip,
	// "replace" => method Self::qs_replace,
	// "sub" => method Self::qs_sub,
	// "gsub" => method Self::qs_gsub,

	// "count" => method Self::qs_count,
	// "empty?" => method Self::qs_empty_q,

	// "includes?" => method |this, args| {
	// 	let this = this.try_downcast::<Self>()?;
	// 	let rhs = args.try_arg(0)?.call_downcast::<Self>()?;

	// 	Ok(this.as_ref().contains(rhs.as_ref()).into())
	// }

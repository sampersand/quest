use std::fmt::{self, Debug, Display, Formatter};
use crate::{RustFn};
use std::marker::PhantomData;

mod float;
mod integer;
mod text;
mod null;
mod boolean;
mod object;
mod literal;

pub use literal::Literal;
pub use object::Object;
pub use boolean::Boolean;
pub use null::Null;
pub use text::Text;

pub type Float = f64;
pub type List = Vec<Value>;
pub type Map = std::collections::HashMap<Value, Value>;

/*
                  |-------------64 bit pointer's used bits------------|
±|-exponent-||------------------------fraction------------------------|
01111111 1111X000 00000000 00000000 00000000 00000000 00000000 00000000
^63      ^55      ^47      ^39      ^31      ^23      ^15      ^7
*/

// NB: The PhantomData's used to make us `!Sync` and `!Send`.
pub struct Value(ValueInner, PhantomData<*mut ()>);

#[repr(C, align(8))]
union ValueInner {
	float: f64,
	bits: u64,
	bytes: [u8; 8],
	tagged: TaggedData
}

#[repr(C, align(8))]
#[derive(Clone, Copy)]
struct TaggedData {
	tag: Tag,
	data: [u8; 6]
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tag(u16);

impl Tag {
	const QNAN: u16 = 0b01111111_11111000;
	const CLONE_MASK: u16 = 0b11111111_11111000;
	const COPY_MASK:  u16 = 0b01111111_11111000;

	const FLOAT64: Self  = Self(0); // Technically, its tag is anything other than these.
	const OBJECT: Self   = Self(Self::CLONE_MASK | 0b000);
	const TEXT: Self     = Self(Self::CLONE_MASK | 0b001);
	const REGEX: Self    = Self(Self::CLONE_MASK | 0b010);
	const BLOCK: Self    = Self(Self::CLONE_MASK | 0b011);
	const LIST: Self     = Self(Self::CLONE_MASK | 0b100);
	const MAP: Self      = Self(Self::CLONE_MASK | 0b101);
	const SCOPE: Self    = Self(Self::CLONE_MASK | 0b110);
	// _                 = Self(Self::CLONE_MASK | 0b111);
	const CONSTS: Self   = Self(Self::COPY_MASK  | 0b000); // see the `impl` block for definitions
	const INT48: Self    = Self(Self::COPY_MASK  | 0b001);
	const RUSTFN: Self   = Self(Self::COPY_MASK  | 0b010);
	const ZST: Self      = Self(Self::COPY_MASK  | 0b011); // eg `Iterable`, `Kernel`, `Pristine`, etc.
	const LITERAL: Self  = Self(Self::COPY_MASK  | 0b100);
	// _                 = Self(Self::COPY_MASK  | 0b101); */
	// _                 = Self(Self::COPY_MASK  | 0b110); */
	// _                 = Self(Self::COPY_MASK  | 0b111); */

	const fn is_clone(&self) -> bool {
		(self.0 & Self::CLONE_MASK) == Self::CLONE_MASK
	}

	const fn is_data_pointer(&self) -> bool {
		self.is_clone() || matches!(*self, Self::RUSTFN)
	}

	#[inline]
	const fn is_float(&self) -> bool {
		(self.0 & Self::QNAN) != Self::QNAN
	}
}


impl Display for Tag {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Self::OBJECT => write!(f, "Object"),
			Self::TEXT => write!(f, "Text"),
			Self::REGEX => write!(f, "Regex"),
			Self::BLOCK => write!(f, "Block"),
			Self::LIST => write!(f, "List"),
			Self::MAP => write!(f, "Map"),
			Self::SCOPE => write!(f, "Scope"),
			Self::CONSTS => write!(f, "Consts"),
			Self::INT48 => write!(f, "Int48"),
			Self::RUSTFN => write!(f, "RustFn"),
			Self::ZST => write!(f, "Zst"),
			Self::LITERAL => write!(f, "Literal"),
			_ => write!(f, "Float64"),
		}
	}
}

impl PartialEq for Value {
	#[inline]
	fn eq(&self, rhs: &Self) -> bool {
		self.bits() == rhs.bits()
	}
}

impl Default for Value {
	#[inline]
	fn default() -> Self {
		Self::NULL
	}
}

impl Value {
	const NAN:   Self = unsafe { Self::new_tagged(Tag::CONSTS, [0, 0, 0, 0, 0, 0b00]) };
	const NULL:  Self = unsafe { Self::new_tagged(Tag::CONSTS, [0, 0, 0, 0, 0, 0b01]) };
	const TRUE:  Self = unsafe { Self::new_tagged(Tag::CONSTS, [0, 0, 0, 0, 0, 0b10]) };
	const FALSE: Self = unsafe { Self::new_tagged(Tag::CONSTS, [0, 0, 0, 0, 0, 0b11]) };

	/// Creates a new [`Value`] with the given `tag` and `data`.
	///
	/// # Safety
	/// The caller must guarantee that `data` is a valid value for the given tag.
	const unsafe fn new_tagged(tag: Tag, data: [u8; 6]) -> Self {
		Self(ValueInner { tagged: TaggedData { tag, data } }, std::marker::PhantomData)
	}

	/// # Safety
	/// The caller must guarantee that `ptr` is a valid pointer of the type that `tag` expect.
	/// It must also live for the correct lifetime. [TODO: which is?]
	///
	/// TODO: Elsewhere, we need to ensure that the upper two bytes will always be zero.
	unsafe fn from_ptr(tag: Tag, ptr: *const ()) -> Self {
		debug_assert!(tag.is_data_pointer(), "Attempt to create a {:?} tag from a pointer", tag);

		let [u1, u2, data @ ..] = (ptr as usize as u64).to_le_bytes();
		debug_assert_eq!(u1, 0, "pointer didn't return 0 for upper byte");
		debug_assert_eq!(u2, 0, "pointer didn't return 0 for second-upper byte");

		unsafe {
			Self::new_tagged(tag, data)
		}
	}

	/// Retrieves the pointer used to create `self`.
	///
	/// # Safety Concerns
	/// While this function isn't _itself_ unsafe, the returned pointer may not be valid. It's up to the caller to ensure
	/// that `self` is a pointer type.
	#[inline]
	fn as_ptr(&self) -> *const () {
		debug_assert!(self.tag().is_data_pointer(), "attempted to get a pointer from a non-pointer tag: {}", self.tag());

		self.masked_data() as *const ()
	}

	/// Retrieves a mutable reference to the pointer used to create `self`.
	///
	/// # Safety Concerns
	/// The same concerns as [`as_ptr`] apply here, with the additional stipulation that `self` should not have been
	// created with a `Copy`-type.
	#[inline]
	fn as_mut_ptr(&mut self) -> *mut () {
		debug_assert!(self.tag().is_clone(), "attempted to get a mutable pointer from a non-mutable tag: {}", self.tag());

		self.as_ptr() as *mut ()	
	}

	/// Gets the tag of `self`
	fn tag(&self) -> Tag {
		// SAFETY: All bit patterns are valid tags, so accessing it is OK.
		unsafe {
			self.0.tagged.tag
		}
	}

	/// Gets the bits associated with self
	fn bits(&self) -> u64 {
		// SAFETY: All bit patterns are valid u64s.
		unsafe {
			self.0.bits
		}
	}

	/// Get the data for `self`, with the tag removed.
	fn masked_data(&self) -> u64 {
		self.bits() & 0xffff_ffff_ffff
	}
}

impl Debug for Value {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self.tag() {
			Tag::CONSTS => {
				// can't use `match` b/c we don't have `Eq` 
				if *self == Self::NAN {
					write!(f, "Nan")
				} else if *self == Self::NULL {
					write!(f, "Null")
				} else if *self == Self::TRUE {
					write!(f, "True")
				} else if *self == Self::FALSE {
					write!(f, "False")
				} else {
					unreachable!("unknown constant encountered: {:x}", self.masked_data())
				}
			},
			Tag::INT48 => write!(f, "Int48({})", self.masked_data()),
			Tag::ZST => write!(f, "ZST({:x})", self.masked_data()),
			Tag::LITERAL => write!(f, "Literal({:x})", self.masked_data()),
			tag if tag.is_data_pointer() => write!(f, "{}({:p})", tag, self.as_ptr()),
			other => write!(f, "Float({})", self.as_float().unwrap())
		}
	}
}

impl Clone for Value {
	fn clone(&self) -> Self {
		let tag = self.tag();

		// If we're a clone type, simply copy the bits.
		if !tag.is_clone() {
			return Self(ValueInner { bits: self.bits() }, Default::default());
		}

		// SAFETY: We do a lot of unchecked conversions here because the `match` checks the tags for us.
		unsafe {
			match tag {
				Tag::OBJECT => Self::new_object(self.as_object_unchecked().clone()),
				Tag::TEXT => Self::new_text(self.as_text_unchecked().clone()),
				Tag::REGEX => todo!(),
				Tag::BLOCK => todo!(),
				Tag::LIST => todo!(),
				Tag::MAP => todo!(),
				Tag::SCOPE => todo!(),
				_ => unreachable!("unknown clone tag encountered: {:?}", tag)
			}
		}
	}
}

impl Drop for Value {
	fn drop(&mut self) {
		use std::ptr::drop_in_place;

		let tag = self.tag();

		// If we're a copy type, then dropping's a no-op
		if !tag.is_clone() {
			return;
		}

		// SAFETY: We do a lot of unchecked conversions here because the `match` checks the tags for us.
		unsafe {
			match tag {
				Tag::OBJECT => drop_in_place(self.as_object_mut_unchecked() as *mut _),
				Tag::TEXT => drop_in_place(self.as_text_mut_unchecked() as *mut _),
				Tag::REGEX => todo!(),
				Tag::BLOCK => todo!(),
				Tag::LIST => todo!(),
				Tag::MAP => todo!(),
				Tag::SCOPE => todo!(),
				_ => unreachable!("unknown clone tag encountered: {:?}", tag)
			}
		}
	}
}

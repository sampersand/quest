use std::fmt::{self, Display, Formatter};
use quest_core::types;
use std::convert::TryFrom;
use std::sync::Arc;

/*
                  |-------------64 bit pointer's used bits------------|
±|-exponent-||------------------------fraction------------------------|
01111111 1111X000 00000000 00000000 00000000 00000000 00000000 00000000
^63      ^55      ^47      ^39      ^31      ^23      ^15      ^7
*/

#[derive(Debug)]
#[repr(C, align(8))]
pub struct Value {
	tag: Tag,
	data: [u8; 6]
}

const _: [(); 0] = [(); if std::mem::size_of::<Value>() == std::mem::size_of::<u64>() { 0 } else { 1 }];
const _: [(); 0] = [(); if std::mem::align_of::<Value>() == std::mem::align_of::<u64>() { 0 } else { 1 }];

const CLONE_MASK: u16 = 0b11111111_11111000;
const COPY_MASK:  u16 = 0b01111111_11111000;

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tag {
	Float64  = 0, // Technically, its tag is anything other than these.
	Object   = CLONE_MASK | 0b000,
	Text     = CLONE_MASK | 0b001,
	Regex    = CLONE_MASK | 0b010,
	Block    = CLONE_MASK | 0b011,
	List     = CLONE_MASK | 0b100,
	Map      = CLONE_MASK | 0b101,
	Scope    = CLONE_MASK | 0b110,
	// _     = CLONE_MASK | 0b111,
	Consts   = COPY_MASK  | 0b000, // see the `impl` block for definitions
	Int48    = COPY_MASK  | 0b001,
	RustFn   = COPY_MASK  | 0b010,
	Zst      = COPY_MASK  | 0b011, // eg `Iterable`, `Kernel`, `Pristine`, etc.
	Literal  = COPY_MASK  | 0b100,
	/* _     = COPY_MASK  | 0b101, */
	/* _     = COPY_MASK  | 0b110, */
	/* _     = COPY_MASK  | 0b111, */
}

impl Value {
	const NAN:   Self = unsafe { Self::new(Tag::Consts, [0, 0, 0, 0, 0, 0b00]) };
	const NULL:  Self = unsafe { Self::new(Tag::Consts, [0, 0, 0, 0, 0, 0b01]) };
	const TRUE:  Self = unsafe { Self::new(Tag::Consts, [0, 0, 0, 0, 0, 0b10]) };
	const FALSE: Self = unsafe { Self::new(Tag::Consts, [0, 0, 0, 0, 0, 0b11]) };

	/// Creates a new [`Value`] with the given `tag` and `data`.
	///
	/// # Safety
	/// This is unsafe because there's no guarantee that the `value` is valid for the tag we're given.
	const unsafe fn new(tag: Tag, data: [u8; 6]) -> Self {
		Self { tag, data }
	}

	/// Creates a new [`Value`] from the given `raw` data.
	///
	/// # Safety
	/// The caller must ensure that `raw` has a valid tag and that the data is valid for it.
	#[inline]
	const unsafe fn from_le(raw: u64) -> Self {
		let [tag0, tag1, data @ ..] = raw.to_le_bytes();
		Self::new(Tag::from_le(u16::from_le_bytes([tag0, tag1])), data)
	}

	unsafe fn from_ptr(tag: Tag, ptr: *const ()) -> Self {
		let [u1, u2, data @ ..] = (ptr as usize as u64).to_le_bytes();
		debug_assert_eq!(u1, 0, "pointer didn't return 0 for upper byte");
		debug_assert_eq!(u2, 2, "pointer didn't return 0 for second-upper byte");

		Self::new(tag, data)
	}


	/// Retrieves `self`'s tag.
	#[inline]
	const fn tag(&self) -> Tag {
		self.tag
	}

	/// Retrieves `self`'s data.
	#[inline]
	const fn data(&self) -> [u8; 6] {
		self.data
	}

	/// Converts `self` to a pointer. This should only be called on clone types.
	#[inline]
	fn as_ptr(&self) -> *const () {
		debug_assert!(!self.tag.is_copy(), "called `as_ptr` on a copy type! (tag={:?})", self.tag);

		(self.as_u64() & 0xffff_ffff_ffff) as *const ()
	}

	#[inline]
	fn as_ptr_mut(&mut self) -> *mut () {
		self.as_ptr() as *mut ()	
	}

	const fn as_u64(&self) -> u64 {
		let [t1, t2] = (self.tag as u16).to_le_bytes();
		let [d1, d2, d3, d4, d5, d6] = self.data;
		u64::from_le_bytes([t1, t2, d1, d2, d3, d4, d5, d6])
	}
}

impl From<types::Null> for Value {
	#[inline]
	fn from(_: types::Null) -> Self {
		Self::NULL
	}
}

impl From<types::Boolean> for Value {
	#[inline]
	fn from(value: types::Boolean) -> Self {
		if value.into_inner() {
			Self::TRUE
		} else {
			Self::FALSE
		}
	}
}

impl From<f64> for Value {
	#[inline]
	fn from(value: f64) -> Self {
		if value.is_nan() {
			Self::NAN
		} else {
			/// SAFETY: We know that `value`'s not `NAN`, 
			Self(value.to_le())
		}
	}
}

impl Drop for Value {
	fn drop(&mut self) {
		if self.tag.is_copy() {
			return;
		}

		/// SAFETY: Assuming the class was created correctly, this won't cause issues.
		unsafe {
			match self.tag {
				Tag::Object => todo!(),
				Tag::Text => drop(Arc::from_raw(self.as_ptr() as *const types::Text)),
				Tag::Regex => drop(Arc::from_raw(self.as_ptr() as *const types::Regex)),
				Tag::Block => drop(Arc::from_raw(self.as_ptr() as *const crate::Block)),
				Tag::List => drop(Arc::from_raw(self.as_ptr() as *const types::List)),
				Tag::Map => drop(Arc::from_raw(self.as_ptr() as *const types::Map)),
				Tag::Scope => drop(Arc::from_raw(self.as_ptr() as *const types::Scope)),
				_ => unreachable!("Copy types should have already been checked for and returned early.")
			}
		}
	}
}

impl Tag {
	#[inline]
	const fn is_copy(self) -> bool {
		(self as u16) & CLONE_MASK == CLONE_MASK
	}

	const fn from_le(tag: u16) -> Self {
		const OBJECT: u16   = Tag::Object as u16;
		const TEXT: u16     = Tag::Text as u16;
		const REGEX: u16    = Tag::Regex as u16;
		const BLOCK: u16    = Tag::Block as u16;
		const LIST: u16     = Tag::List as u16;
		const MAP: u16      = Tag::Map as u16;
		const SCOPE: u16    = Tag::Scope as u16;
		const CONSTS: u16   = Tag::Consts as u16;
		const INT48: u16    = Tag::Int48 as u16;
		const RUSTFN: u16   = Tag::RustFn as u16;
		const ZST: u16      = Tag::Zst as u16;
		const LITERAL: u16  = Tag::Literal as u16;

		match tag {
			OBJECT => Self::Object,
			TEXT => Self::Text,
			REGEX => Self::Regex,
			BLOCK => Self::Block,
			LIST => Self::List,
			MAP => Self::Map,
			SCOPE => Self::Scope,
			CONSTS => Self::Consts,
			INT48 => Self::Int48,
			RUSTFN => Self::RustFn,
			ZST => Self::Zst,
			LITERAL => Self::Literal,
			_ => Self::Float64
		}

	}
}

impl Display for Tag {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::Object => write!(f, "Object"),
			Self::Text => write!(f, "Text"),
			Self::Regex => write!(f, "Regex"),
			Self::Block => write!(f, "Block"),
			Self::List => write!(f, "List"),
			Self::Map => write!(f, "Map"),
			Self::Scope => write!(f, "Scope"),
			Self::Consts => write!(f, "Consts"),
			Self::Int48 => write!(f, "Int48"),
			Self::RustFn => write!(f, "RustFn"),
			Self::Zst => write!(f, "Zst"),
			Self::Literal => write!(f, "Literal"),
			Self::Float64 => write!(f, "Float64")
		}
	}
}

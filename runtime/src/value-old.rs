use std::fmt::{self, Display, Formatter};
use quest_core::types;
use std::convert::TryFrom;

/*
                  |------------------64 bit pointer-------------------|
±|-exponent-||------------------------fraction------------------------|
01111111 1111X000 00000000 00000000 00000000 00000000 00000000 00000000
^63      ^55      ^47      ^39      ^31      ^23      ^15      ^7
*/

#[repr(transparent)]
pub struct Value(ValueInner);

const _: [(); 0] = [(); if std::mem::size_of::<Value>() == std::mem::size_of::<u64>() { 0 } else { 1 }];
const _: [(); 0] = [(); if std::mem::align_of::<Value>() == std::mem::align_of::<u64>() { 0 } else { 1 }];

#[repr(C, align(8))]
union ValueInner {
	tagged: TaggedData,
	raw: u64
}

#[derive(Debug, Clone, Copy)]
#[repr(C, align(8))]
struct TaggedData { tag: Tag, data: [u8; 6] }

const CLONE_MASK: u16 = 0b01111111_11111000;
const COPY_MASK:  u16 = 0b11111111_11111000;

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tag {
	Float64  = 0, // Technically, its tag is anything other than these.
	Object   = CLONE_MASK | 0b000,
	Text     = CLONE_MASK | 0b001,
	Regex    = CLONE_MASK | 0b010,
	Block    = CLONE_MASK | 0b011,
	List     = CLONE_MASK | 0b100,
	Map      = CLONE_MASK | 0b101,
	Scope    = CLONE_MASK | 0b110,
	External = CLONE_MASK | 0b111,  // types defined by outside libraries.
	NullBool = COPY_MASK  | 0b000,
	Int48    = COPY_MASK  | 0b001,
	RustFn   = COPY_MASK  | 0b010,
	Zst      = COPY_MASK  | 0b011,
	Literal  = COPY_MASK  | 0b100,
	/* _     = COPY_MASK  | 0b101, */
	/* _     = COPY_MASK  | 0b110, */
	/* _     = COPY_MASK  | 0b111, */
}


impl Value {
	/// Creates a new [`Value`] with the given `tag` and `data`.
	///
	/// # Safety
	/// This is unsafe because there's no guarantee that the `value` is valid for the tag we're given.
	pub const unsafe fn new(tag: Tag, data: [u8; 6]) -> Self {
		Self { tag, data }
	}

	/// Creates a new [`Value`] from the given `raw` data.
	///
	/// # Safety
	/// The caller must ensure that `raw` has a valid tag and that the data is valid for it.
	#[inline]
	pub const unsafe fn from_raw(raw: u64) -> Self {
		let [tag0, tag1, data @ ..] = raw.to_le_bytes();
		Self::new(Tag::from(u16::from_le_bytes([tag0, tag1])), data)
	}

	/// Retrieves `self`'s tag.
	#[inline]
	pub const fn tag(&self) -> Tag {
		self.tag
	}

	/// Retrieves `self`'s data.
	#[inline]
	pub const fn data(&self) -> [u8; 6] {
		self.data
	}

	/// Converts `self` to a pointer. This should only be called on clone types, ie with the CLONE_MASK
	#[inline]
	const fn as_ptr(&self) -> *const () {
		debug_assert!(!self.tag.is_copy(), "called `as_ptr` on a copy type! (tag={:?})", self.tag);

		(self.as_u64() & 0xffff_ffff_ffff) as *const ()
	}

	#[inline]
	const fn as_ptr_mut(&mut self) -> *mut () {
		self.as_ptr() as *mut ()	
	}

	const fn as_u64(&self) -> u64 {
		use std::mem::{size_of, align_of, transmute};
		const _: [(); 0] = [(); if size_of::<Value>() == size_of::<u64>() { 0 } else { 1 }];
		const _: [(); 0] = [(); if align_of::<Value>() == align_of::<u64>() { 0 } else { 1 }];

		/// SAFETY: All bit representations of `u64` are valid, and `self`'s size and alignment are correct, so this
		/// is ok

		u64::from_le_bytes(unsafe { transmute(self) })
	}
}

impl Drop for Value {
	fn drop(&mut self) {
		if self.tag.is_copy() {
			return;
		}

		todo!();
		// self.tag.drop_data(self.data);
	}
}

impl Tag {
	const fn is_copy(self) -> bool {
		(self as u16) & COPY_MASK == COPY_MASK
	}
}

impl From<u16> for Tag {
	fn from(tag: u16) -> Self {
		const OBJECT: u16   = Tag::Object as u16;
		const TEXT: u16     = Tag::Text as u16;
		const REGEX: u16    = Tag::Regex as u16;
		const BLOCK: u16    = Tag::Block as u16;
		const LIST: u16     = Tag::List as u16;
		const MAP: u16      = Tag::Map as u16;
		const SCOPE: u16    = Tag::Scope as u16;
		const EXTERNAL: u16 = Tag::External as u16;
		const NULLBOOL: u16 = Tag::NullBool as u16;
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
			EXTERNAL => Self::External,
			NULLBOOL => Self::NullBool,
			INT48 => Self::Int48,
			RUSTFN => Self::RustFn,
			ZST => Self::Zst,
			LITERAL => Self::Literal,
			_ => Self::Float64
		}
	}
}

impl From<Tag> for u16 {
	fn from(tag: Tag) -> Self {
		tag as _
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
			Self::External => write!(f, "External"),
			Self::NullBool => write!(f, "NullBool"),
			Self::Int48 => write!(f, "Int48"),
			Self::RustFn => write!(f, "RustFn"),
			Self::Zst => write!(f, "Zst"),
			Self::Literal => write!(f, "Literal"),
			Self::Float64 => write!(f, "Float64")
		}
	}
}

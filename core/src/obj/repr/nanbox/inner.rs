use super::Tag;

#[repr(C)]
pub struct Inner {
	tag: Tag,
	data: Data
}

#[repr(transparent)]
pub struct Data([u8; 6]);

impl Inner {
	#[inline]
	pub const fn new(tag: Tag, data: Data) -> Self {
		Self { tag, data }
	}

	#[inline]
	pub const fn tag(&self) -> Tag {
		self.tag
	}

	#[inline]
	pub const fn data(&self) -> &Data {
		&self.data
	}
}

impl Data {
	#[inline]
	pub const fn from_bytes(data: [u8; 6]) -> Self {
		Self(data)
	}

	#[inline]
	pub const unsafe fn duplicate(&self) -> Self {
		Self::from_bytes(self.0)
	}

	pub fn from_u64(data: u64) -> Self {
		assert_eq!(data & 0xffff_0000_0000_0000, 0, "data isn't only the lower 16 bits");
		let [data @ .., _, _] = unsafe { std::mem::transmute::<u64, [u8; 8]>(data) };
		Self::from_bytes(data)
	}

	#[inline]
	pub fn from_ptr(ptr: *const super::super::heap_only::Internal) -> Self {
		Self::from_u64(ptr as u64)
	}

	pub fn lower_u32(&self) -> u32 {
		let [lower @ .., _a, _b] = self.0;
		debug_assert_eq!(_a | _b, 0, "lower_u32 still had upper bytes set? (_a={:b}, _b={:b})", _a, _b);
		unsafe { std::mem::transmute::<[u8; 4], u32>(lower) }
	}


	pub unsafe fn as_ptr(&self) -> *const super::super::heap_only::Internal {
		let ptr = *((self as *const Self as *const u8).offset(-2) as *const u64);
		(ptr & 0x0000_ffff_ffff_ffff) as *const _
	}
}

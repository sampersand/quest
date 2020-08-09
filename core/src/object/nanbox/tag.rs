#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tag {
	Heap = 0x0000,
	Null = 0x0001,
	Boolean = 0x0002,
	RustFn = 0x0003,
	NumberI32 = 0x0004,
	NumberF32 = 0x0005,
	ZeroSizedType,
}

#[test]
fn is_u16() {
	assert_eq!(std::mem::size_of::<Tag>(), std::mem::size_of::<u16>());
}

impl Tag {
	#[cfg(debug_assertions)]
	pub fn is_copy(self) -> bool {
		!matches!(self, Self::Heap)
	}

	#[cfg(debug_assertions)]
	pub fn is_tag(tag: u16) -> bool {
		tag <= Self::NumberF32 as u16
	}
}

impl From<u64> for Tag {
	fn from(num: u64) -> Self {
		let tag = (num >> 48) as u16;

		debug_assert!(Tag::is_tag(tag), "invalid tag found: {:?}", tag);

		unsafe {
			std::mem::transmute::<u16, Self>(tag)
		}
	}
}

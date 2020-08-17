#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tag {
	Heap      = 0,
	NumberI32 = 1,
	NumberF32 = 2,
}


impl Tag {
	pub const MAX_TAG: u8 = 2;
}

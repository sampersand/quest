#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tag(pub u8);

impl Tag {
	pub const HEAP: Self = Self(0);

	pub const LITERAL: Self            = Self(0b1000_0000);
	pub const LITERAL_EMBEDDED: Self   = Self(0b1000_0000);
	pub const LITERAL_HEAP_ALLOC: Self = Self(0b1100_0000);
	pub const MAX_EMBEDDED_LITERAL_LEN: usize = 0b11_1111;

	pub const fn is(self, t: Self) -> bool {
		(self.0 & t.0) == t.0
	}
}

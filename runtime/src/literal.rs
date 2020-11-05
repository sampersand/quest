// use std::hash::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Literal(u32);

impl Literal {
	pub fn new(literal: &'static str) -> Self {
		todo!()
	}

	pub(crate) const fn into_raw(self) -> [u8; 6] {
		let [_, _, bytes @ ..] = (self.0 as u64).to_be_bytes();
		bytes
	} 

	pub(crate) const unsafe fn from_raw(raw: [u8; 6]) -> Self {
		Self(0)
		// Self(raw)
	}
}

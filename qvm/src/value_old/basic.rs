use crate::{Value, LMap};

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct Basic {
	flags: Flags,
	parent: Value,
	attrs: Box<LMap>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C, align(4))]
struct Flags {
	compiler: [u8; 3],
	custom_flags: u8,
}

impl Basic {
	pub fn new(parent: Value, custom_flags: u8) -> Self {
		Self {
			parent,
			flags: Flags { compiler: [0; 3], custom_flags },
			attrs: Box::new(LMap::new()),
		}
	}

	pub fn flags(&self) -> u8 {
		self.flags.custom_flags
	}

	pub fn flags_mut(&mut self) -> &mut u8 {
		&mut self.flags.custom_flags
	}
}

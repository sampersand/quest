bitflags::bitflags! {
	#[derive(Default)]
	pub struct Flags: u8 {
		const CLEAR    = 0b00000000;
		const POS      = 0b00000001;
		const NEG      = 0b00000010;
		const ZERO     = 0b00000100;
		const CMP_MASK = 0b11111000;
	}
}


impl Flags {
	pub const fn new() -> Self {
		Self::CLEAR
	}
}

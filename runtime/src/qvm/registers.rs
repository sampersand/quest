use super::Register;
use std::ops::{Index, IndexMut};
use crate::{Integer, Value};

pub type RegisterIndex = u8;

#[derive(Debug)]
pub struct Registers([Register; RegisterIndex::MAX as usize]);

const RSP_INDEX: RegisterIndex = RegisterIndex::MAX - 1;
const RIP_INDEX: RegisterIndex = RegisterIndex::MAX - 2;
const RET_INDEX: RegisterIndex = RegisterIndex::MAX - 3;

impl Registers {
	/// Creates a new list of registers.
	pub const fn new()  -> Self {
		Self([Register::ZERO; RegisterIndex::MAX as usize])
	}

	pub fn sp(&self) -> &Register {
		&self[RSP_INDEX]
	}

	pub fn sp_mut(&mut self) -> &mut Register {
		&mut self[RSP_INDEX]
	}

	pub fn ip(&self) -> &Register {
		&self[RIP_INDEX]
	}

	pub fn ip_mut(&mut self) -> &mut Register {
		&mut self[RIP_INDEX]
	}

	pub fn ret(&self) -> &Register {
		&self[RET_INDEX]
	}

	pub fn ret_mut(&mut self) -> &mut Register {
		&mut self[RET_INDEX]
	}
}

impl Index<RegisterIndex> for Registers {
	type Output = Register;
	fn index(&self, index: RegisterIndex) -> &Self::Output {
		// SAFETY: We have exactly `RegisterIndex::MAX` registers, so every `index` is a valid register.
		unsafe {
			&*(&self.0[0] as *const Register).offset(index as isize)
		}
	}
}

impl IndexMut<RegisterIndex> for Registers {
	fn index_mut(&mut self, index: RegisterIndex) -> &mut Self::Output {
		// SAFETY: We have exactly `RegisterIndex::MAX` registers, so every `index` is a valid register.
		unsafe {
			&mut *(&mut self.0[0] as *mut Register).offset(index as isize)
		}
	}
}

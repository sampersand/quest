use crate::Value;

mod bytecode;
mod register;
mod registers;
mod stack;
mod flags;

use stack::Stack;
use bytecode::ByteCode;
use register::Register;
use registers::{Registers, RegisterIndex};
use flags::Flags;

#[derive(Debug)]
pub struct QuestVm {
	program: Vec<ByteCode>,
	registers: Registers,
	stack: Stack,
	flags: Flags
}

impl QuestVm {
	pub fn new(bytecode: impl IntoIterator<Item=ByteCode>) -> Self {
		Self {
			registers: Registers::new(),
			stack: Stack::new(),
			flags: Flags::new(),
			program: bytecode.into_iter().collect(),
		}
	}
}

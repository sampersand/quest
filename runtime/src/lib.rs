extern crate static_assertions as sa;

mod register;
mod bytecode;

pub use register::{Registers, Register, RegisterIndex};
pub use bytecode::ByteCode;

#[allow(unused)]
#[derive(Debug, Default)]
pub struct QuestVm {
	regs: Registers,
	stack: Vec<quest_core::Object>,
	code: Vec<ByteCode>,
	flags: Flags
}

const DEFAULT_STACK_CAPACITY: usize = 100000;

impl QuestVm {
	pub fn new(code: impl IntoIterator<Item=ByteCode>) -> Self {
		Self {
			regs: Default::default(),
			stack: Vec::with_capacity(DEFAULT_STACK_CAPACITY),
			code: code.into_iter().collect(),
			flags: Flags::default()
		}
	}

	pub fn run(mut self) -> quest_core::Object {
		while self.is_running() {
			let ip = self.regs[RegisterIndex::InstructionPointer].as_integer();
			self.regs[RegisterIndex::InstructionPointer].store((ip + 1).into());
			self.code[ip as usize].run(&mut self);
		}

		self.regs[RegisterIndex::Return].take()
	}

	fn is_running(&self) -> bool {
		self.code.len() > self.regs[RegisterIndex::InstructionPointer].as_integer() as usize
	}

	pub fn print_program(&self) {
		for line in &self.code {
			println!("{}", line);
		}
	}
}

bitflags::bitflags! {
	#[derive(Default)]
		struct Flags : u8 {
		const POS = 1;
		const ZERO = 2;
		const NEG = 4;
		const CMP = 7;
	}
}

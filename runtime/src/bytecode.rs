use quest_core::Literal;
use std::fmt::{self, Display, Formatter};
use crate::{QuestVm, RegisterIndex, Flags};

type Offset = isize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ByteCode {
	// General
	Nop,
	LoadL(RegisterIndex, Literal),
	StoreI(RegisterIndex, i64),
	StoreL(RegisterIndex, Literal),
	Mov(RegisterIndex, RegisterIndex),
	Push(RegisterIndex),
	Pop(RegisterIndex),

	// call/cc and jumping
	Call(Offset),
	Ret(RegisterIndex),
	Raise(RegisterIndex),
	Cmp(RegisterIndex),
	Jeq(Offset),
	Jne(Offset),
	Jgt(Offset),
	Jge(Offset),
	Jlt(Offset),
	Jle(Offset),
	Jmp(Offset),

	// Math
	Neg(RegisterIndex),
	Add(RegisterIndex, RegisterIndex),
	Sub(RegisterIndex, RegisterIndex),
	Mul(RegisterIndex, RegisterIndex),
	Div(RegisterIndex, RegisterIndex),
	Mod(RegisterIndex, RegisterIndex),
	Pow(RegisterIndex, RegisterIndex),

	// Bitwise
	Not(RegisterIndex),
	And(RegisterIndex, RegisterIndex),
	 Or(RegisterIndex, RegisterIndex),
	Xor(RegisterIndex, RegisterIndex),
	Shl(RegisterIndex, RegisterIndex),
	Shr(RegisterIndex, RegisterIndex),

	// Quest-Specific
	// in the future, i may want to add "GetAttrImmediate" or something, where you can pass a value in as the attr name.
	// These all go <src>, <attr>, ...
	GetAttrL(RegisterIndex, Literal),
	SetAttrL(RegisterIndex, Literal, RegisterIndex),
	DelAttrL(RegisterIndex, Literal),
	CallAttrL(RegisterIndex, Literal, u8),

	GetAttr(RegisterIndex, RegisterIndex),
	SetAttr(RegisterIndex, RegisterIndex, RegisterIndex),
	DelAttr(RegisterIndex, RegisterIndex),
	CallAttr(RegisterIndex, RegisterIndex, u8)
}


impl ByteCode {
	pub fn run(self, vm: &mut QuestVm) {
		macro_rules! callbinary {
			($vm:expr, $lhs:expr, $rhs:expr, $literal:expr) => {{
				Self::Push($rhs).run($vm);
				Self::CallAttrL($lhs, $literal, 1).run($vm) 
			}};
		}
		match self {
			// General
			Self::Nop => { /* do nothing */ },
			Self::LoadL(reg, literal) => {
				use quest_core::types::Text;
				let obj = Text::const_new(literal.into_inner()).evaluate().expect("couldn't eval?");
				vm.regs[reg].store(obj)
			},
			Self::StoreI(reg, num) => vm.regs[reg].store(num.into()),
			Self::StoreL(reg, lit) => {
				vm.regs[reg].store(lit.into())
			},
			Self::Mov(dst, src) => {
				let dup = vm.regs[src].load().clone();
				vm.regs[dst].store(dup);
			},
			Self::Push(reg) => {
				let data = vm.regs[reg].load().clone();
				vm.stack.push(data);
			},
			Self::Pop(reg) => {
				let val = vm.stack.pop().expect("popped from an empty stack!");
				vm.regs[reg].store(val);
			},

			// call/cc and jumping
			Self::Call(offset) => {
				Self::Push(RegisterIndex::InstructionPointer).run(vm);
				Self::Jmp(offset).run(vm);
			}
			Self::Ret(ret) => {
				let val = vm.regs[ret].take();
				vm.regs[RegisterIndex::Return].store(val);
				Self::Pop(RegisterIndex::InstructionPointer).run(vm);
			},
			Self::Raise(_) => todo!("raise"),
			Self::Cmp(reg) => {
				use std::cmp::Ordering;

				let val = vm.regs[reg].as_integer();
				match val.cmp(&0) {
					Ordering::Less    => vm.flags = (vm.flags & !Flags::CMP) | Flags::NEG,
					Ordering::Equal   => vm.flags = (vm.flags & !Flags::CMP) | Flags::ZERO,
					Ordering::Greater => vm.flags = (vm.flags & !Flags::CMP) | Flags::POS,
				}
			}
			Self::Jeq(offset) => if  dbg!(vm.flags).contains(Flags::ZERO) { Self::Jmp(offset).run(vm) },
			Self::Jne(offset) => if !vm.flags.contains(Flags::ZERO) { Self::Jmp(offset).run(vm) },
			Self::Jgt(offset) => if  vm.flags.contains( Flags::POS) { Self::Jmp(offset).run(vm) },
			Self::Jge(offset) => if !vm.flags.contains( Flags::NEG) { Self::Jmp(offset).run(vm) },
			Self::Jlt(offset) => if  vm.flags.contains( Flags::NEG) { Self::Jmp(offset).run(vm) },
			Self::Jle(offset) => if !vm.flags.contains( Flags::POS) { Self::Jmp(offset).run(vm) },
			Self::Jmp(offset) => {
				let ip = vm.regs[RegisterIndex::InstructionPointer].as_integer();
				vm.regs[RegisterIndex::InstructionPointer].store((ip as isize + offset).into())
			},

			// Math
			Self::Neg(reg) => Self::CallAttrL(reg, Literal::NEG, 0).run(vm),
			Self::Add(lhs, rhs) => callbinary!(vm, lhs, rhs, Literal::ADD),
			Self::Sub(lhs, rhs) => callbinary!(vm, lhs, rhs, Literal::SUB),
			Self::Mul(lhs, rhs) => callbinary!(vm, lhs, rhs, Literal::MUL),
			Self::Div(lhs, rhs) => callbinary!(vm, lhs, rhs, Literal::DIV),
			Self::Mod(lhs, rhs) => callbinary!(vm, lhs, rhs, Literal::MOD),
			Self::Pow(lhs, rhs) => callbinary!(vm, lhs, rhs, Literal::POW),

			// Bitwise
			Self::Not(reg) => Self::CallAttrL(reg, Literal::NOT, 0).run(vm),
			Self::And(lhs, rhs) => callbinary!(vm, lhs, rhs, Literal::BAND),
			Self::Or(lhs, rhs) => callbinary!(vm, lhs, rhs, Literal::BOR),
			Self::Xor(lhs, rhs) => callbinary!(vm, lhs, rhs, Literal::BXOR),
			Self::Shl(lhs, rhs) => callbinary!(vm, lhs, rhs, Literal::SHL),
			Self::Shr(lhs, rhs) => callbinary!(vm, lhs, rhs, Literal::SHR),

			// Quest-Specific
			Self::GetAttrL(obj, literal) => {
				let value = vm.regs[obj].load().get_attr_lit(&literal).expect("todo: err handling");
				vm.regs[obj].store(value);
			},
			Self::SetAttrL(obj, literal, val) => {
				let val = vm.regs[val].load().clone();
				vm.regs[obj].load().set_attr_lit(literal, val).expect("todo: err handling");
			},
			Self::DelAttrL(obj, literal) => {
				let deleted = vm.regs[obj].load().del_attr_lit(&literal).expect("todo: err handling");
				vm.regs[obj].store(deleted);
			},
			Self::CallAttrL(obj, literal, argcount) => {
				let mut args = Vec::with_capacity(argcount as usize);
				for _ in 0..argcount {
					args.push(vm.stack.pop().expect("attempted to pop from an empty stack"));
				}

				let result = vm.regs[obj].load().call_attr_lit(&literal,
					args.iter().collect::<quest_core::Args>()).expect("todo: err handling");
				vm.regs[RegisterIndex::Return].store(result);
			}

			Self::GetAttr(obj, attr) => {
				let value = vm.regs[obj].load().get_attr(vm.regs[attr].load()).expect("todo: err handling");
				vm.regs[obj].store(value);
			},
			Self::SetAttr(obj, attr, val) => {
				let attr = vm.regs[attr].load().clone();
				let val = vm.regs[val].load().clone();
				vm.regs[obj].load().set_attr(attr, val).expect("todo: err handling");
			},
			Self::DelAttr(obj, attr) => {
				let deleted = vm.regs[obj].load().del_attr(vm.regs[attr].load()).expect("todo: err handling");
				vm.regs[obj].store(deleted);
			},
			Self::CallAttr(obj, attr, argcount) => {
				let mut args = Vec::with_capacity(argcount as usize);
				for _ in 0..argcount {
					args.push(vm.stack.pop().expect("attempted to pop from an empty stack"));
				}

				let result = vm.regs[obj].load().call_attr(
					vm.regs[attr].load(),
					args.iter().collect::<quest_core::Args>()).expect("todo: err handling");
				vm.regs[RegisterIndex::Return].store(result);
			}
		};
	}
}

impl Display for ByteCode{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			// General
			Self::Nop => write!(f, "nop"),
			Self::StoreI(reg, int) => write!(f, "loadi {}, ${}", reg, int),
			Self::LoadL(reg, literal) => write!(f, "loadl {}, \"{}\"", reg, literal),
			Self::StoreL(reg, literal) => write!(f, "storel {}, \"{}\"", reg, literal),
			Self::Mov(dst, src) => write!(f, "mov {}, {}", dst, src),
			Self::Push(reg) => write!(f, "push {}", reg),
			Self::Pop(reg) => write!(f, "pop {}", reg),

			// call/cc and jumping
			Self::Call(offset) => write!(f, "call {}", offset),
			Self::Ret(reg) => write!(f, "ret {}", reg),
			Self::Raise(_) => todo!(),
			Self::Cmp(reg) => write!(f, "cmp {}", reg),
			Self::Jeq(offset) => write!(f, "jeq ${}", offset),
			Self::Jne(offset) => write!(f, "jne ${}", offset),
			Self::Jgt(offset) => write!(f, "jgt ${}", offset),
			Self::Jge(offset) => write!(f, "jge ${}", offset),
			Self::Jlt(offset) => write!(f, "jlt ${}", offset),
			Self::Jle(offset) => write!(f, "jle ${}", offset),
			Self::Jmp(offset) => write!(f, "jmp ${}", offset),

			// Math
			Self::Neg(reg) => write!(f, "neg {}", reg),
			Self::Add(lhs, rhs) => write!(f, "add {}, {}", lhs, rhs),
			Self::Sub(lhs, rhs) => write!(f, "sub {}, {}", lhs, rhs),
			Self::Mul(lhs, rhs) => write!(f, "mul {}, {}", lhs, rhs),
			Self::Div(lhs, rhs) => write!(f, "div {}, {}", lhs, rhs),
			Self::Mod(lhs, rhs) => write!(f, "mod {}, {}", lhs, rhs),
			Self::Pow(lhs, rhs) => write!(f, "pow {}, {}", lhs, rhs),

			// Bitwise
			Self::Not(reg) => write!(f, "not {}", reg),
			Self::And(lhs, rhs) => write!(f, "and {}, {}", lhs, rhs),
			Self::Or(lhs, rhs) => write!(f, "or {}, {}", lhs, rhs),
			Self::Xor(lhs, rhs) => write!(f, "xor {}, {}", lhs, rhs),
			Self::Shl(lhs, rhs) => write!(f, "shl {}, {}", lhs, rhs),
			Self::Shr(lhs, rhs) => write!(f, "shr {}, {}", lhs, rhs),

			// Quest-Specific
			Self::GetAttrL(obj, literal) => write!(f, "getattrl {}, \"{}\"", obj, literal),
			Self::SetAttrL(obj, literal, value) => write!(f, "setattrl {}, \"{}\", {}", obj, literal, value),
			Self::DelAttrL(obj, literal) => write!(f, "delattrl {}, \"{}\"", obj, literal),
			Self::CallAttrL(obj, literal, count) => write!(f, "callattrl {}, \"{}\", {}", obj, literal, count),

			Self::GetAttr(obj, attr) => write!(f, "getattr {}, {}", obj, attr),
			Self::SetAttr(obj, attr, value) => write!(f, "setattr {}, {}, {}", obj, attr, value),
			Self::DelAttr(obj, attr) => write!(f, "delattr {}, {}", obj, attr),
			Self::CallAttr(obj, attr, count) => write!(f, "callattr {}, {}, {}", obj, attr, count)
		}
	}
}

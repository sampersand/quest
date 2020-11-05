use crate::{Integer, Value, value::Literal};
use super::{QuestVm, RegisterIndex, Flags};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ByteCode {
	// General
	Nop,
	StoreI(RegisterIndex, Integer),
	Dup(RegisterIndex, RegisterIndex),
	Push(RegisterIndex),
	Pop(RegisterIndex),

	// call/cc
	Call(Integer),
	Ret(RegisterIndex),
	Raise(RegisterIndex),

	// Jumping
	Cmp(RegisterIndex),
	Jeq(Integer),
	Jne(Integer),
	Jgt(Integer),
	Jge(Integer),
	Jlt(Integer),
	Jle(Integer),
	Jmp(Integer),

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
	// These all go <attr>, <src>, ...
	GetAttrL(RegisterIndex, Literal),
	SetAttrL(RegisterIndex, Literal, RegisterIndex),
	DelAttrL(RegisterIndex, Literal),
	CallAttrL(RegisterIndex, Literal, u8),

	GetAttr(RegisterIndex, RegisterIndex),
	SetAttr(RegisterIndex, RegisterIndex, RegisterIndex),
	DelAttr(RegisterIndex, RegisterIndex),
	CallAttr(RegisterIndex, RegisterIndex, u8)
}

mod ops {
	use super::*;

	#[inline]
	pub(super) fn nop() {
		// do nothing. it's a no-op lol
	}

	pub(super) fn store_i(vm: &mut QuestVm, dst: RegisterIndex, value: Integer) {
		vm.registers[dst].store(Value::new_int(value));
	}

	pub(super) fn dup(vm: &mut QuestVm, src: RegisterIndex, dst: RegisterIndex) {
		let dup = vm.registers[src].load().clone();
		vm.registers[dst].store(dup);
	}

	pub(super) fn push(vm: &mut QuestVm, reg: RegisterIndex) {
		vm.stack.push(vm.registers[reg].take());
	}

	pub(super) fn pop(vm: &mut QuestVm, reg: RegisterIndex) {
		let value = vm.stack.pop().expect("popped from an empty stack!");
		vm.registers[reg].store(value);
	}

	pub(super) fn call(vm: &mut QuestVm, offset: Integer) {
		vm.stack.push(vm.registers.ip_mut().take());

		jmp(vm, offset)
	}

	pub(super) fn ret(vm: &mut QuestVm, reg: RegisterIndex) {
		let ip = vm.stack.pop().expect("popped from an empty stack!");
		vm.registers.ip_mut().store(ip);

		let ret = vm.registers[reg].take();
		vm.registers.ret_mut().store(ret);
	}

	pub(super) fn raise(vm: &mut QuestVm, what: RegisterIndex) {
		let _ = what;
		todo!("raise");
	}

	// Jumping
	pub(super) fn cmp(vm: &mut QuestVm, reg: RegisterIndex) {
		vm.flags &= Flags::CMP_MASK;
		use std::cmp::Ordering;

		match vm.registers[reg].cmp() {
			Ordering::Less => vm.flags |= Flags::NEG,
			Ordering::Equal => vm.flags |= Flags::ZERO,
			Ordering::Greater => vm.flags |= Flags::POS,
		}
	}

	// Jumping
	macro_rules! jmp_instrs {
		($($name:ident $(($not:tt))? $flag:ident),*) => {
			$(
				pub(super) fn $name(vm: &mut QuestVm, offset: Integer) {
					if $($not)? vm.flags.contains(Flags::$flag) {
						jmp(vm, offset);
					}
				}
			)*
		};
	}

	jmp_instrs!(jeq ZERO, jne (!) ZERO, jlt NEG, jle (!) POS, jgt POS, jge (!) NEG);

	pub(super) fn jmp(vm: &mut QuestVm, offset: Integer) {
		let sp = vm.registers.sp().load().as_int().expect("stack pointer wasn't an int?");
		let new_sp = Integer::new(sp.into_i64() + offset.into_i64()).expect("stack pointer overflowed!");
		vm.registers.sp_mut().store(crate::Value::new_int(new_sp))
	}

	pub(super) fn callattr_l(vm: &mut QuestVm, object: RegisterIndex, attr: Literal, args: &[RegisterIndex]) {
		let _ = (vm, attr, args);
		todo!();
	}

	pub(super) fn neg(vm: &mut QuestVm, reg: RegisterIndex) {
		if let Some(int) = vm.registers[reg].load().as_int() {
			let new = Integer::new_unchecked(-int.into_i64());
			vm.registers[reg].store(Value::new_int(new));
		} else {
			callattr_l(vm, reg, Literal::NOT, &[])
		}
	}

	pub(super) fn add(vm: &mut QuestVm, lhs: RegisterIndex, rhs: RegisterIndex) 
		if let Some(lhs) = vm.registers[lhs].load().as_int() {
			if let Some(rhs) = vm.registers[rhs].load().as_int() {
				vm.registers[reg].store(Value::new_integer(lhs.into_i64() + rhs.into_i64()));
			} else {
				let rhs = vm.registers[rhs].load
			}
		} else {
			callattr_l(vm, reg, Literal::NOT, &[])
		}
	}
}

impl ByteCode {
	pub fn run(&self, vm: &mut QuestVm) {
		match *self {
			// General
			Self::Nop => ops::nop(),
			Self::StoreI(reg, value) => ops::store_i(vm, reg, value),
			Self::Dup(src, dst) => ops::dup(vm, src, dst),
			Self::Push(reg) => ops::push(vm, reg),
			Self::Pop(reg) => ops::pop(vm, reg),

			// call/cc
			Self::Call(offset) => ops::call(vm, offset),
			Self::Ret(reg) => ops::ret(vm, reg),
			Self::Raise(err) => ops::raise(vm, err),

			// Jumping
			Self::Cmp(reg) => ops::cmp(vm, reg),
			Self::Jeq(offset) => ops::jeq(vm, offset),
			Self::Jne(offset) => ops::jne(vm, offset),
			Self::Jgt(offset) => ops::jgt(vm, offset),
			Self::Jge(offset) => ops::jge(vm, offset),
			Self::Jlt(offset) => ops::jlt(vm, offset),
			Self::Jle(offset) => ops::jle(vm, offset),
			Self::Jmp(offset) => ops::jmp(vm, offset),

			// Math ops
			Self::Neg(reg) => ops::neg(vm, reg),
			Self::Add(lhs, rhs) => ops::add(vm, lhs, rhs),
			_ => todo!(),
		};
		// 	Self::PushI(RegisterIndex, Value),
		// 	Self::PopI(RegisterIndex, Value),

		// 	// call/cc
		// 	Call(Offset, Vec<RegisterIndex>),
		// 	Ret(RegisterIndex),
		// 	Raise(RegisterIndex),

		// 	// Jumping
		// 	Cmp(RegisterIndex),
		// 	Jeq(Offset),
		// 	Jne(Offset),
		// 	Jgt(Offset),
		// 	Jge(Offset),
		// 	Jlt(Offset),
		// 	Jle(Offset),
		// 	Jmp(Offset),

		// 	// Math
		// 	Neg(RegisterIndex),
		// 	Add(RegisterIndex, RegisterIndex),
		// 	Sub(RegisterIndex, RegisterIndex),
		// 	Mul(RegisterIndex, RegisterIndex),
		// 	Div(RegisterIndex, RegisterIndex),
		// 	Mod(RegisterIndex, RegisterIndex),
		// 	Pow(RegisterIndex, RegisterIndex),

		// 	// Bitwise
		// 	Not(RegisterIndex),
		// 	And(RegisterIndex, RegisterIndex),
		// 	 Or(RegisterIndex, RegisterIndex),
		// 	Xor(RegisterIndex, RegisterIndex),
		// 	Shl(RegisterIndex, RegisterIndex),
		// 	Shr(RegisterIndex, RegisterIndex),

		// 	// Quest-Specific
		// 	// in the future, i may want to add "GetAttrImmediate" or something, where you can pass a value in as the attr name.
		// 	// These all go <attr>, <src>, ...
		// 	GetAttr(RegisterIndex, RegisterIndex),
		// 	SetAttr(RegisterIndex, RegisterIndex, RegisterIndex),
		// 	DelAttr(RegisterIndex, RegisterIndex),
		// 	CallAttr(RegisterIndex, RegisterIndex, Vec<RegisterIndex>)
		// }
	}
}

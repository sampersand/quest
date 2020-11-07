use super::Register;
use std::ops::{Index, IndexMut};
use std::fmt::{self, Display, Formatter};

/// The amount of [scratch](Registers::scratch) registers that are available.
// TODO: If this is changed, make sure `RegisterIndex` is changed as well.
const SCRATCH_REGISTERS: usize = 64;

/// The amount of [preserved](Registers::preserved) that are available.
// TODO: If this is changed, make sure `RegisterIndex` is changed as well.
const PRESERVED_REGISTERS: usize = 64;

/// A struct that holds all the registers for this vm.
#[derive(Debug, Clone)]
pub struct Registers {
	/// Scratch registers: registers that are able to be clobbered (and thus must be 
	/// pushed to the stack if a function is called.)
	scratch: [Register; SCRATCH_REGISTERS],

	/// Preserved registers: registers that must be restored to their previous value
	/// whenever a function returns.
	preserved: [Register; PRESERVED_REGISTERS],

	/// The current instruction pointer. 
	ip: Register,

	/// The current stack pointer.
	sp: Register,

	/// The register used to store a return value.
	ret: Register
}

impl Default for Registers {
	fn default() -> Self {
		use std::convert::TryFrom;
		// this can be optimized with `unsafe` and `MaybeUninit`.

		let scratch = vec![Register::default(); SCRATCH_REGISTERS];
		let preserved = vec![Register::default(); PRESERVED_REGISTERS];

		Self {
			scratch: <&[Register; SCRATCH_REGISTERS]>::try_from(scratch.as_slice()).unwrap().clone(),
			preserved: <&[Register; PRESERVED_REGISTERS]>::try_from(preserved.as_slice()).unwrap().clone(),
			ip: Register::new(0.into()),
			sp: Register::new(0.into()),
			ret: Register::default()
		}
	}
}

/// The ways you can index into [`Registers`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegisterIndex(u8);

#[allow(non_upper_case_globals, non_snake_case)]
impl RegisterIndex {
	/// The stack pointer.
	pub const StackPointer: Self = Self(0b10000000);

	/// The instruction pointer.
	pub const InstructionPointer: Self = Self(0b10000001);

	/// The instruction which return values are put in.
	pub const Return: Self = Self(0b10000010);

	/// A scratch register---it can be clobbered by callers.
	pub const fn Scratch(index: u8) -> Self {
		Self(index & 0b00111111)
	}

	/// A preserved register---it must be restored when a function returns.
	pub const fn Preserved(index: u8) -> Self {
		Self((index & 0b00111111) | 0b01000000)
	}
}


impl Display for RegisterIndex {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Self::StackPointer => write!(f, "%sp"),
			Self::InstructionPointer => write!(f, "%ip"),
			Self::Return => write!(f, "%ret"),
			Self(scratch) if scratch < 0b01000000 => write!(f, "%s{}", scratch),
			Self(preserved) => write!(f, "%p{}", preserved),
		}
	}
}
impl Index<RegisterIndex> for Registers {
	type Output = Register;

	fn index(&self, idx: RegisterIndex) -> &Self::Output {
		if idx.0 & 0b10000000 != 0 {
			match idx {
				RegisterIndex::StackPointer => &self.sp,
				RegisterIndex::InstructionPointer => &self.ip,
				RegisterIndex::Return => &self.ret,
				_ => unreachable!("unknown register encountered: {:?}", idx)
			}
		} else if idx.0 & 0b01000000 != 0 {
			&self.preserved[(idx.0 & 0b00111111) as usize]
		} else {
			&self.scratch[idx.0 as usize]
		}
	}
}

impl IndexMut<RegisterIndex> for Registers {
	fn index_mut(&mut self, idx: RegisterIndex) -> &mut Self::Output {
		if idx.0 & 0b10000000 != 0 {
			match idx {
				RegisterIndex::StackPointer => &mut self.sp,
				RegisterIndex::InstructionPointer => &mut self.ip,
				RegisterIndex::Return => &mut self.ret,
				_ => unreachable!("unknown register encountered: {:?}", idx)
			}
		} else if idx.0 & 0b01000000 != 0 {
			&mut self.preserved[(idx.0 & 0b00111111) as usize]
		} else {
			&mut self.scratch[idx.0 as usize]
		}
	}
}


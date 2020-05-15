use std::cmp::Ordering;
use crate::obj::{Object, types};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
	Pos, Neg,
	Add, Sub, Mul, Div, Mod, Pow,
	Not, Eql, Neq, Lth, Leq, Gth, Geq, Cmp, And, Or,
	Lsh, Rsh, BNot, BAnd, BOr, Xor,
	Assign, Dot, DotAssign,
	AddAssign, SubAssign, MulAssign, DivAssign, ModAssign, PowAssign, LshAssign, RshAssign, BAndAssign, BOrAssign, XorAssign,
}

impl From<Operator> for Object {
	fn from(op: Operator) -> Self {
		Object::from(types::Text::from(op))
	}
}

impl From<Operator> for types::Text {
	fn from(op: Operator) -> Self {
		use Operator::*;
		Self::from(match op {
			Pos => "+@", Neg => "-@",
			Add => "+", Sub => "-", Mul => "*", Div => "/", Mod => "%",
			Pow => "**",
			Not => "!", Eql => "==", Neq => "!=",
			Lth => "<", Leq => "<=", Gth => ">", Geq => ">=",
			Cmp => "<=>",
			And => "&&", Or => "||",
			BNot => "~", BAnd => "&",
			Lsh => "<<", Rsh => ">>", BOr => "|", Xor => "^",
			Dot => ".",
			Assign => "=", DotAssign => ".=",
			AddAssign => "+=", SubAssign => "-=", MulAssign => "*=", DivAssign => "/=", ModAssign => "%=",
			PowAssign => "**=", LshAssign => "<<=", RshAssign => ">>=", BAndAssign => "&=", BOrAssign => "|=",
			XorAssign => "^=",
		})
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Associativity {
	LeftToRight,
	RightToLeft,
	UnaryOperOnLeft,
	// UnaryOperOnRight,
}

impl Operator {
	pub fn into_unary_left(self) -> Self {
		match self {
			Operator::Add => Operator::Pos,
			Operator::Sub => Operator::Neg,
			_ => self
		}
	}

	pub fn is_symbol_unary_left(&self) -> bool {
		use Operator::*;

		match self {
			Pos | Neg | Add | Sub | Not | BNot => true,
			_ => false
		}
	}

	pub fn assoc(&self) -> Associativity {
		use Operator::*;
		match self {
			Pos | Neg | Not | BNot => Associativity::UnaryOperOnLeft,
			Assign | DotAssign | AddAssign | SubAssign | MulAssign | DivAssign
				| ModAssign | PowAssign | LshAssign | RshAssign | BAndAssign | BOrAssign | XorAssign
				=> Associativity::RightToLeft,
			_ => Associativity::LeftToRight
		}
	}

	pub fn arity(&self) -> usize {
		match self.assoc() {
			Associativity::LeftToRight | Associativity::RightToLeft => 2,
			Associativity::UnaryOperOnLeft /*| Associativity::UnaryOperOnRight*/ => 1
		}
	}

	fn precedence(&self) -> usize {
		use Operator::*;
		// using ruby's precedence as a template.
		match self {
			Dot => 0,
			Not | BNot | Pos => 1,
			Pow => 2,
			Neg => 3,
			Mul | Div | Mod => 4,
			Add | Sub => 5,
			Lsh | Rsh => 6,
			BAnd => 7,
			BOr | Xor => 8,
			Lth | Leq | Gth | Geq => 9,
			Cmp | Eql | Neq => 10,
			And => 11,
			Or => 12,
			Assign | DotAssign | AddAssign | SubAssign | MulAssign | DivAssign | ModAssign
				| PowAssign | LshAssign | RshAssign | BAndAssign | BOrAssign | XorAssign => 13
		}
	}
}

impl Ord for Operator {
	fn cmp(&self, rhs: &Operator) -> Ordering {
		self.precedence().cmp(&rhs.precedence())
	}
}

impl PartialOrd for Operator {
	fn partial_cmp(&self, rhs: &Operator) -> Option<Ordering> {
		Some(self.cmp(rhs))
	}
}




use std::cmp::Ordering;
use quest::{Object, Key, types, EqResult};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
	Pos, Neg,
	Add, Sub, Mul, Div, Mod, Pow,
	Not, Eql, Neq, Lth, Leq, Gth, Geq, Cmp, And, Or,
	Lsh, Rsh, BNot, BAnd, BOr, Xor,
	Assign, Dot, DotAssign, ColonColon,
	Call, Index,
	AddAssign, SubAssign, MulAssign, DivAssign, ModAssign, PowAssign, LshAssign, RshAssign, BAndAssign, BOrAssign, XorAssign,
}


impl From<Operator> for Object {
	fn from(op: Operator) -> Self {
		Object::from(types::Text::from(op))
	}
}

impl EqResult<Key> for Operator {
	fn equals(&self, rhs: &Key) -> quest::Result<bool> {
		self.to_string().as_str().equals(rhs)
	}
}

impl Display for Operator {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		use Operator::*;
		write!(f, "{}", match self {
			Pos => "+@", Neg => "-@",
			Add => "+", Sub => "-", Mul => "*", Div => "/", Mod => "%",
			Pow => "**",
			Not => "!", Eql => "==", Neq => "!=",
			Lth => "<", Leq => "<=", Gth => ">", Geq => ">=",
			Cmp => "<=>",
			And => "&&", Or => "||",
			BNot => "~", BAnd => "&",
			Lsh => "<<", Rsh => ">>", BOr => "|", Xor => "^",
			Dot => ".", ColonColon => "::",
			Assign => "=", DotAssign => ".=",
			Call => "()", Index => "[]",
			AddAssign => "+=", SubAssign => "-=", MulAssign => "*=", DivAssign => "/=", ModAssign => "%=",
			PowAssign => "**=", LshAssign => "<<=", RshAssign => ">>=", BAndAssign => "&=", BOrAssign => "|=",
			XorAssign => "^=",
		})
	}
}
impl From<Operator> for types::Text {
	fn from(op: Operator) -> Self {
		types::Text::from(op.to_string())
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
			Dot | ColonColon => 0,
			Call | Index => 1,
			Not | BNot | Pos => 2,
			Pow => 3,
			Neg => 4,
			Mul | Div | Mod => 5,
			Add | Sub => 6,
			Lsh | Rsh => 7,
			BAnd => 8,
			BOr | Xor => 9,
			Lth | Leq | Gth | Geq => 10,
			Cmp | Eql | Neq => 11,
			And => 12,
			Or => 13,
			Assign | DotAssign | AddAssign | SubAssign | MulAssign | DivAssign | ModAssign
				| PowAssign | LshAssign | RshAssign | BAndAssign | BOrAssign | XorAssign => 14
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




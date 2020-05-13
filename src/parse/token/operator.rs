use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
	Pos, Neg,
	Add, Sub, Mul, Div, Mod, Pow,
	Not, Eql, Neq, Lth, Leq, Gth, Geq, Cmp, And, Or,
	Lsh, Rsh, BNot, BAnd, BOr, Xor,
	Dot, Comma, Endline,

	Assign, DotAsn, DotDel,
	AddAsn, SubAsn, MulAsn, DivAsn, ModAsn, PowAsn, LshAsn, RshAsn, BAndAsn, BOrAsn, XorAsn,
	Index, Call, IndexAsn, IndexDel
}

impl Display for Operator {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		use self::Operator::*;
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
			Dot => ".", Comma => ",", Endline => ";",
			Assign => "=", DotAsn => ".=", DotDel => ".~",
			AddAsn => "+=", SubAsn => "-=", MulAsn => "*=", DivAsn => "/=", ModAsn => "%=",
			PowAsn => "**=", LshAsn => "<<=", RshAsn => ">>=", BAndAsn => "&=", BOrAsn => "|=",
			XorAsn => "^=",
			Call => "()",
			Index => "[]",
			IndexAsn => "[]=",
			IndexDel => "[]~",
		})
	}
}

#[derive(Debug)]
pub enum Associativity {
	LeftToRight,
	RightToLeft,
	UnaryOperOnLeft,
	UnaryOperOnRight,
}

impl Operator {
	pub fn associativity(&self) -> Associativity {
		use Operator::*;
		match self {
			Pos | Neg | Not | BNot => Associativity::UnaryOperOnLeft,
			Assign | DotAsn | DotDel | AddAsn | SubAsn | MulAsn | DivAsn
				| ModAsn | PowAsn | LshAsn | RshAsn | BAndAsn | BOrAsn | XorAsn
				=> Associativity::RightToLeft,
			Index | IndexAsn | IndexDel => unreachable!(),
			_ => Associativity::LeftToRight
		}
	}

	pub fn arity(&self) -> usize {
		match self.associativity() {
			Associativity::LeftToRight | Associativity::RightToLeft => 2,
			Associativity::UnaryOperOnLeft | Associativity::UnaryOperOnRight => 1
		}
	}

	fn precedence(&self) -> usize {
		use Operator::*;
		// using ruby's precedence as a template.
		match self {
			Dot => 0,
			Call => 1,
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
			DotAsn | DotDel => 14, // do i want them here?
			Assign | AddAsn | SubAsn | MulAsn | DivAsn | ModAsn
				| PowAsn | LshAsn | RshAsn | BAndAsn | BOrAsn | XorAsn => 14,
			Comma => 15,
			Endline => 16,
			Index | IndexAsn | IndexDel => todo!()
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




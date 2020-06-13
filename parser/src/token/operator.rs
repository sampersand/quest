#![allow(unused)]
use crate::Result;
use crate::stream::{BufStream, Stream};
use crate::token::{Token, Tokenizable, TokenizeResult};
use quest::{Object, Key, types, EqResult};
use std::cmp::Ordering;
use std::io::BufRead;
use std::fmt::{self, Display, Formatter};

macro_rules! operator_enum {
	(; TRY_PARSE $_repr:literal ()) => { None };
	(; TRY_PARSE $repr:literal) => { Some($repr) };
	(; TRY_PARSE $_repr:literal ($ident:literal)) => { Some($ident) };
	($(
		$variant:ident($repr:literal $(($($ident:literal)?))? $ord:literal $($assoc:ident)? $($arity:literal)?)
	)*) => {
		#[derive(Debug, Clone, Copy, PartialEq, Eq)]
		pub enum Operator {
			$($variant),*
		}

		impl Tokenizable for Operator {
			type Item = Self;

			fn try_tokenize<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Self::Item>> {
				$({
					let o = operator_enum!(; TRY_PARSE $repr $(($($ident)?))?);
					if o.map(|x| stream.starts_with(x)).transpose()?.unwrap_or(false) {
						try_seek!(stream, o.unwrap().len() as i64);
						return Ok(TokenizeResult::Some(Operator::$variant))
					}
				})*
				{
					Ok(TokenizeResult::None)
				}
			}
		}

		impl Display for Operator {
			fn fmt(&self, f: &mut Formatter) -> fmt::Result {
				match self {
					$(
						Operator::$variant => Display::fmt($repr, f),
					)*
				}
			}
		}

		impl Operator {
			fn precedence(&self) -> usize {
				match self {
					$(Operator::$variant => $ord,)*
				}
			}
		}
	};
}

// "Longer" operators need to be at the top so shorter ones don't overshadow them
operator_enum!{
	// 3 characters
	PowAssign("**=" 16) LshAssign("<<=" 16) RshAssign(">>=" 16) Cmp("<=>" 13)

	// 2 characters
	AddAssign("+=" 16) SubAssign("-=" 16) MulAssign("*=" 16) DivAssign("/=" 16) ModAssign("%=" 16) 
	BAndAssign("&=" 16) BOrAssign("|=" 16) BXorAssign("^=" 16) Or("||" 15) And("&&" 14) Eql("==" 12)
	Neq("!=" 12) Leq("<=" 11) Geq(">=" 11) Lsh("<<" 7) Rsh(">>" 7) Pow("**" 3) ColonColon("::" 0)

	// 1 Character
	Assign("=" 16) Lth("<" 11) Gth(">" 11) BXor("^" 10) BOr("|" 9) BAnd("&" 8) Add("+" 6) Sub("-" 6)
	Mul("*" 5) Div("/" 5) Mod("%" 5) Not("!" 2 UnaryOperOnLeft 1) BNot("~" 2 UnaryOperOnLeft 1)
	Dot("." 0)

	// Unrepresentable
	Neg("-@" () 4 UnaryOperOnLeft 1)
	Pos("+@" () 2 UnaryOperOnLeft 1) 
	DotAssign(".=" () 16)
	Call("()" () 1)
	Index("[]" () 1)
}

impl From<Operator> for Token {
	fn from(op: Operator) -> Token {
		Token::Operator(op)
	}
}

impl PartialOrd for Operator {
	fn partial_cmp(&self, rhs: &Operator) -> Option<std::cmp::Ordering> {
		Some(self.cmp(&rhs))
	}
}

impl Ord for Operator {
	fn cmp(&self, rhs: &Operator) -> std::cmp::Ordering {
		self.precedence().cmp(&rhs.precedence())
	}
}

// #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
// pub enum Associativity {
// 	LeftToRight,
// 	RightToLeft,
// 	UnaryOperOnLeft,
// 	// UnaryOperOnRight,
// }

// impl Operator {
// 	pub fn into_unary_left(self) -> Self {
// 		match self {
// 			Operator::Add => Operator::Pos,
// 			Operator::Sub => Operator::Neg,
// 			_ => self
// 		}
// 	}

// 	pub fn is_symbol_unary_left(&self) -> bool {
// 		use Operator::*;

// 		match self {
// 			Pos | Neg | Add | Sub | Not | BNot => true,
// 			_ => false
// 		}
// 	}

// 	pub fn assoc(&self) -> Associativity {
// 		use Operator::*;
// 		match self {
// 			Pos | Neg | Not | BNot => Associativity::UnaryOperOnLeft,
// 			Assign | DotAssign | AddAssign | SubAssign | MulAssign | DivAssign
// 				| ModAssign | PowAssign | LshAssign | RshAssign | BAndAssign | BOrAssign | XorAssign
// 				=> Associativity::RightToLeft,
// 			_ => Associativity::LeftToRight
// 		}
// 	}

// 	pub fn arity(&self) -> usize {
// 		match self.assoc() {
// 			Associativity::LeftToRight | Associativity::RightToLeft => 2,
// 			Associativity::UnaryOperOnLeft /*| Associativity::UnaryOperOnRight*/ => 1
// 		}
// 	}

// 	fn precedence(&self) -> usize {
// 		use Operator::*;
// 		// using ruby's precedence as a template.
// 		match self {
// 			Dot | ColonColon => 0,
// 			Call | Index => 1,
// 			Not | BNot | Pos => 2,
// 			Pow => 3,
// 			Neg => 4,
// 			Mul | Div | Mod => 5,
// 			Add | Sub => 6,
// 			Lsh | Rsh => 7,
// 			BAnd => 8,
// 			BOr | Xor => 9,
// 			Lth | Leq | Gth | Geq => 10,
// 			Cmp | Eql | Neq => 11,
// 			And => 12,
// 			Or => 13,
// 			Assign | DotAssign | AddAssign | SubAssign | MulAssign | DivAssign | ModAssign
// 				| PowAssign | LshAssign | RshAssign | BAndAssign | BOrAssign | XorAssign => 14
// 		}
// 	}
// }

// impl Ord for Operator {
// 	fn cmp(&self, rhs: &Operator) -> Ordering {
// 		self.precedence().cmp(&rhs.precedence())
// 	}
// }

// impl PartialOrd for Operator {
// 	fn partial_cmp(&self, rhs: &Operator) -> Option<Ordering> {
// 		Some(self.cmp(rhs))
// 	}
// }
#![allow(unused)]
use crate::Result;
use crate::expression::{PutBack, Constructable, Expression};
use crate::stream::{Stream, Contexted};
use crate::token::{Token, Tokenizable, TokenizeResult};
use quest_core::{Object, types};
use std::cmp::Ordering;
use std::io::BufRead;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Associativity {
	LeftToRight,
	RightToLeft,
	UnaryOperOnLeft,
	// UnaryOperOnRight,
}

macro_rules! operator_enum {
	(; MAX_PRECEDENCE $ord:literal) => { $ord };
	(; MAX_PRECEDENCE $lhs:literal $($rest:literal)*) => {{
		let rhs = operator_enum!(; MAX_PRECEDENCE $($rest)*);
		// a hack to get the maximum value of integers
		[$lhs, rhs][($lhs < rhs) as usize]
	}};

	(; ASSOC ) => { operator_enum!(; ASSOC LeftToRight ); };
	(; ASSOC $which:ident ) => { Associativity::$which };
	(; TRY_PARSE $_repr:literal ()) => { None };
	(; TRY_PARSE $repr:literal) => { Some($repr) };
	(; TRY_PARSE $_repr:literal ($ident:literal)) => { Some($ident) };
	($(
		$variant:ident($repr:literal $(($($ident:literal)?))? $ord:literal $($assoc:ident)? $($arity:literal)?)
	)+) => {
		#[derive(Debug, Clone, Copy, PartialEq, Eq)]
		pub enum Operator {
			$($variant),*
		}

		impl Tokenizable for Operator {
			type Item = Self;

			fn try_tokenize<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Self::Item>> {
				$({
					let o = operator_enum!(; TRY_PARSE $repr $(($($ident)?))?);
					if o.map(|x| stream.next_if_starts_with(x)).transpose()?.unwrap_or(false) {
						return Ok(TokenizeResult::Some(Operator::$variant))
					}
				})+
				{
					Ok(TokenizeResult::None)
				}
			}
		}

		impl Operator {
			pub const MAX_PRECEDENCE: usize = operator_enum!(; MAX_PRECEDENCE $($ord)+) as usize;

			pub fn repr(&self) -> &'static str {
				match self {
					$(Operator::$variant => $repr),+
				}
			}
			pub fn precedence(&self) -> usize {
				match self {
					$(Operator::$variant => $ord,)+
				}
			}


			pub fn assoc(&self) -> Associativity {
				match self {
					$(Operator::$variant => operator_enum!(; ASSOC $($assoc)?),)+
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
	Call("()" () 0)
	Index("[]" () 1)
	WithBlock("{}" () 1)
}

impl Display for Operator {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.repr(), f)
	}
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

impl quest_core::ToObject for Operator {
	fn to_object(&self) -> Object {
		Object::from(self.to_string())
	}
}

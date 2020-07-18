//! All the operators that can be used within Quest.
#![allow(unused)]
use crate::Result;
use crate::expression::{PutBack, Constructable, Expression};
use crate::stream::{Stream, Contexted};
use crate::token::{Token, Tokenizable};
use quest_core::{Object, types};
use std::cmp::Ordering;
use std::io::BufRead;
use std::fmt::{self, Display, Formatter};

// TODO: actually implement this.
/// The associativity of an operator.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Associativity {
	/// Indicates the operator is a left-to-right associative binary operator.
	///
	/// This means if you have `x OP y OP z`, it'll be parsed as `(x OP y) OP z`. Most operators are
	/// Left-to-right associative.
	LeftToRight,

	/// Indicates the operator is a left-to-right associative binary operator.
	///
	/// This means if you have `x OP y OP z`, it'll be parsed as `x OP (y OP z)`. Few operators are
	/// Right-to-left associative: Some noteable exceptions are `=` and `**`.
	RightToLeft,

	/// Indicates the operator is a unary operator on the left-hand side.
	///
	/// This means it's represented as `OP x`
	UnaryOperOnLeft,

	// UnaryOperOnRight,
}

macro_rules! docify {
	($($doc:expr)*; $item:item) => { $(#[doc=$doc])* $item };
}

macro_rules! operator_enum {
	(; ASSOC ) => { operator_enum!(; ASSOC LeftToRight ); };
	(; ASSOC $which:ident ) => { Associativity::$which };
	(; TRY_PARSE $_repr:literal ()) => { None };
	(; TRY_PARSE $repr:literal) => { Some($repr) };
	(; TRY_PARSE $_repr:literal ($ident:literal)) => { Some($ident) };
	($(
		$variant:ident($repr:literal $(($($ident:literal)?))? $ord:literal $($assoc:ident)? $($arity:literal)?)
	)+) => {
		/// An enum represenitng all possible operators
		#[derive(Debug, Clone, Copy, Eq)]
		pub enum Operator {
			$(#[doc="The `"]
			#[doc=$repr]
			#[doc="` operator."]
			$variant),*
		}

		impl Tokenizable for Operator {
			fn try_tokenize<S: Stream>(stream: &mut S) -> Result<Option<Self>> {
				$({
					let o = operator_enum!(; TRY_PARSE $repr $(($($ident)?))?);
					if o.map(|x| stream.next_if_starts_with(x)).transpose()?.unwrap_or(false) {
						return Ok(Some(Operator::$variant))
					}
				})+
				{
					Ok(None)
				}
			}
		}

		impl Operator {
			/// Gets the string representation of the operator.
			pub fn repr(&self) -> &'static str {
				match self {
					$(Operator::$variant => $repr),+
				}
			}

			/// gets the precedence of the operator.
			fn precedence(&self) -> usize {
				match self {
					$(Operator::$variant => $ord,)+
				}
			}

			/// Gets the associativity of the oeprator.
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
	Neq("!=" 12) Leq("<=" 11) Geq(">=" 11) Lsh("<<" 7) Rsh(">>" 7) Pow("**" 3) Scoped("::" 0)

	// 1 Character
	Assign("=" 16) Lth("<" 11) Gth(">" 11) BXor("^" 10) BOr("|" 9) BAnd("&" 8) Add("+" 6) Sub("-" 6)
	Mul("*" 5) Div("/" 5) Mod("%" 5) Not("!" 2 UnaryOperOnLeft 1) BNot("~" 2 UnaryOperOnLeft 1)
	Dot("." 0)

	// Unrepresentable
	Neg("-@" () 4 UnaryOperOnLeft 1)
	Pos("+@" () 2 UnaryOperOnLeft 1) 
	RootScope("::@" () 0 UnaryOperOnLeft 1) 
	DotAssign(".=" () 16)
	Call("()" () 0)
	Index("[]" () 1)
	WithBlock("{}" () 1)
}

impl Display for Operator {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.repr(), f)
	}
}

impl From<Operator> for &'static str {
	#[inline]
	fn from(op: Operator) -> Self {
		op.repr()
	}
}

impl std::hash::Hash for Operator {
	#[inline]
	fn hash<H: std::hash::Hasher>(&self, h: &mut H) {
		self.repr().hash(h)
	}
}

impl PartialEq for Operator {
	#[inline]
	fn eq(&self, rhs: &Operator) -> bool {
		self.repr() == rhs.repr()
	}
}

impl From<Operator> for Token {
	#[inline]
	fn from(op: Operator) -> Token {
		Token::Operator(op)
	}
}

impl PartialOrd for Operator {
	#[inline]
	fn partial_cmp(&self, rhs: &Operator) -> Option<std::cmp::Ordering> {
		Some(self.cmp(&rhs))
	}
}

impl Ord for Operator {
	#[inline]
	fn cmp(&self, rhs: &Operator) -> std::cmp::Ordering {
		self.precedence().cmp(&rhs.precedence())
	}
}

impl quest_core::ToObject for Operator {
	#[inline]
	fn to_object(&self) -> Object {
		Object::from(self.to_string())
	}
}

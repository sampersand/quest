//! The list of literal attributes used within quest.

use std::borrow::Borrow;
use std::fmt::{self, Display, Formatter};

/// A literal attribute, used internally to speed up field access.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Literal(&'static str);

impl Literal {
	#[inline]
	pub const fn new(lit: &'static str) -> Self {
		Self(lit)
	}

	#[inline]
	pub const fn into_inner(self) -> &'static str {
		self.0
	}
}

impl Display for Literal {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl From<&'static str> for Literal {
	#[inline]
	fn from(lit: &'static str) -> Self {
		Self::new(lit)
	}
}

impl AsRef<str> for Literal {
	#[inline]
	fn as_ref(&self) -> &str {
		&self.0
	}
}

impl Borrow<str> for Literal {
	#[inline]
	fn borrow(&self) -> &str {
		self.as_ref()
	}
}

impl PartialEq<str> for Literal {
	#[inline]
	fn eq(&self, rhs: &str) -> bool {
		self.0 == rhs
	}
}


impl PartialEq<&str> for Literal {
	#[inline]
	fn eq(&self, rhs: &&str) -> bool {
		self.0 == *rhs
	}
}

impl From<Literal> for crate::Object {
	#[inline]
	fn from(lit: Literal) -> Self {
		lit.0.into()
	}
}

impl Borrow<Literal> for &'static str {
	fn borrow(&self) -> &Literal {
		unsafe {
			&*(self as *const Self as *const Literal)
		}
	}
}

macro_rules! literals {
	($($name:ident $key:literal)*) => {
		impl Literal {
			$(
				#[doc = "The attribute `"]
				#[doc = $key]
				#[doc = "`."]
				pub const $name: Literal = Literal::new($key);
			)*
		}
	};
}

literals! {
	// stuff for mappings
	__PARENTS__ "__parents__" __ID__ "__id__" __ATTR_MISSING__ "__attr_missing__"

	__THIS__ "__this__" __KEYS__ "__keys__" __ARGS__ "__args__" __STACK__ "__stack__"

	// conversions
	AT_BOOL "@bool" AT_TEXT "@text" AT_NUM "@num" AT_LIST "@list"

	// common functions
	CLONE "clone" HASH "hash" INSPECT "inspect" NAME "name"

	// operators
	ADD  "+"   SUB  "-"    MUL "*"    DIV    "/"   MOD "%"    POW "**"   POS  "+@"   NEG "-@"
	NOT  "!"   EQL  "=="   NEQ "!="   LTH    "<"   GTH ">"    LEQ "<="   GEQ  ">="   CMP "<=>"
	BNOT "~"   BAND "&"    BOR "|"    BXOR   "^"   SHL "<<"   SHR ">>"   CALL "()"
}

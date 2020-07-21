//! The list of literal attributes used within quest.

/// A literal attribute, used internally to speed up field access.
pub type Literal = &'static str;

macro_rules! literals {
	($($name:ident $key:literal)*) => {
		$(
			#[doc = "The attribute `"]
			#[doc = $key]
			#[doc = "`."]
			pub const $name: Literal = $key;
		)*
	};
}

literals! {
	// stuff for mappings
	__PARENTS__ "__parents__" __ID__ "__id__" __ATTR_MISSING__ "__attr_missing__"

	__THIS__ "__this__" __KEYS__ "__keys__" __STACK__ "__stack__"

	// conversions
	AT_BOOL "@bool" AT_TEXT "@text" AT_NUM "@num" AT_LIST "@list"

	// common functions
	CLONE "clone" HASH "hash" INSPECT "inspect"

	// operators
	ADD  "+"   SUB  "-"    MUL "*"    DIV    "/"   MOD "%"    POW "**"   POS  "+@"   NEG "-@"
	NOT  "!"   EQL  "=="   NEQ "!="   LTH    "<"   GTH ">"    LEQ "<="   GEQ  ">="   CMP "<=>"
	BNOT "~"   BAND "&"    BOR "|"    BXOR   "^"   SHL "<<"   SHR ">>"   CALL "()"
}

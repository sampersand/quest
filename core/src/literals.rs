macro_rules! literals {
	($($name:ident $key:literal)*) => {
		$(
			pub const $name: crate::obj::Key = crate::obj::Key::Literal($key);
		)*
	};
}

literals! {
	// stuff for mappings
	__PARENTS__ "__parents__" __ID__ "__id__" __ATTR_MISSING__ "__attr_missing__"

	__THIS__ "__this__"

	// conversions
	AT_BOOL "@bool" AT_TEXT "@text" AT_NUM "@num" AT_LIST "@list" AT_MAP "@map"

	// common functions
	CLONE "clone" HASH "hash"

	// operators
	ADD  "+"   SUB  "-"    MUL "*"    DIV    "/"   MOD "%"    POW "**"   POS  "+@"   NEG "-@"
	NOT  "!"   EQL  "=="   NEQ "!="   LTH    "<"   GTH ">"    LEQ "<="   GEQ  ">="   CMP "<=>"
	BNOT "~"   BAND "&"    BOR "|"    BXOR   "^"   SHL "<<"   SHR ">>"   CALL "()"
}
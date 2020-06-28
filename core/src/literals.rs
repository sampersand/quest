macro_rules! literals {
	($($name:ident $key:literal)*) => {
		$(
			pub const $name: crate::obj::Key = crate::obj::Key::Literal($key);
		)*
	};
}

literals! {
	// stuff for mappings
	PARENTS "__parents__" ID "__id__" ATTR_MISSING "__attr_missing__"

	// conversions
	AT_BOOL "@bool" AT_TEXT "@text" AT_NUM "@num" AT_LIST "@list" AT_MAP "@map"

	// common functions
	CLONE "clone" HASH "hash"

	// operators
	ADD  "+"   SUB  "-"    MUL "*"    DIV    "/"   MOD "%"    POW "**"   POS  "+@"   NEG "-@"
	NOT  "!"   EQL  "=="   NEQ "!="   LTH    "<"   GTH ">"    LEQ "<="   GEQ  ">="   CMP "<=>"
	BNOT "~"   BAND "&"    BOR "|"    BXOR   "^"   SHL "<<"   SHR ">>"   CALL "()"
}
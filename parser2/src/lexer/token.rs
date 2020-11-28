#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
	// Misc Tokens
	Newline, Semicolon, Comma,

	// Parens
	LParen, RParen, LBracket, RBracket, LBrace, RBrace,

	// Assignment Operators
	AddAssign, SubAssign, MulAssign, DivAssign, ModAssign, PowAssign, 
	AndAssign, OrAssign, XorAssign, ShlAssign, ShrAssign,
	Assign,

	// Logical Operators
	Cmp, Eql, Neq, Lth, Gth, Leq, Geq,

	// Bitwise Operators
	And, Or, Xor, Shl, Shr,

	// Math Operators
	Add, Sub, Mul, Div, Mod, Pow,

	// Unary Operators
	Neg, Pos, BNot, LNot,

	// Scoping
	Dot, Colon2,

	// Literals
	Identifier(String), Number(String), String(String)
}

impl Token {
	pub fn try_parse(&str)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
	// Misc Tokens
	Newline, Semicolon, Comma,

	// Parens
	LParen, RParen, LBracket, RBracket, LBrace, RBrace,

	// Assignment Operators
	AddAssign, SubAssign, MulAssign, DivAssign, ModAssign, PowAssign, 
	AndAssign, OrAssign, XorAssign, ShlAssign, ShrAssign,
	Assign,

	// Logical Operators
	Cmp, Eql, Neq, Lth, Gth, Leq, Geq,

	// Bitwise Operators
	And, Or, Xor, Shl, Shr,

	// Math Operators
	Add, Sub, Mul, Div, Mod, Pow,

	// Unary Operators
	Neg, Pos, BNot, LNot,

	// Scoping
	Dot, Colon2,

	// Literals
	Identifier(String), Number(String), String(String)
}
}

mod operator;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParenType {
	Curly, Bracket, Paren
}

impl Display for ParenType {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			ParenType::Curly   => write!(f, "{{}}"),
			ParenType::Bracket => write!(f, "[]"),
			ParenType::Paren   => write!(f, "()"),
		}
	}
}

pub use self::operator::Operator;

#[derive(Debug)]
#[non_exhaustive] // we might want, eg, "time" literals or whatnot in the future?
pub enum Literal {
	Number(crate::obj::types::Number),
	Text(crate::obj::types::Text),
	Variable(crate::obj::types::Text)
}


#[derive(Debug)]
#[non_exhaustive]
pub enum Token {
	Literal(Literal),
	Operator(Operator),
	Left(ParenType),
	Right(ParenType),
}
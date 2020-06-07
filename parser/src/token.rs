pub mod operator;
pub use self::operator::Operator;
use std::fmt::{self, Debug, Formatter};
use quest::{Object, types};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParenType {
	Paren, Bracket, Brace
}

impl From<ParenType> for Object {
	fn from(parentype: ParenType) -> Self {
		Object::from(types::Text::from(parentype))
	}
}

impl From<ParenType> for types::Text {
	fn from(parentype: ParenType) -> Self {
		match parentype {
			ParenType::Paren => types::Text::new_static("()"),
			ParenType::Bracket => types::Text::new_static("[]"),
			ParenType::Brace => types::Text::new_static("{}"),
		}
	}
}


#[derive(PartialEq, Eq, Clone)]
#[non_exhaustive] // we might want, eg, "time" literals or whatnot in the future?
pub enum Literal {
	Number(types::Number),
	Text(types::Text),
	Variable(types::Text)
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum Token {
	Literal(Literal),
	Operator(Operator),
	Comma,
	Endline,
	Left(ParenType),
	Right(ParenType),
}

impl Debug for Literal {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Literal::Number(num) => write!(f, "Literal::Number({:?})", num),
			Literal::Text(txt) => write!(f, "Literal::Text({:?})", txt),
			Literal::Variable(var) => write!(f, "Literal::Variable({:?})", var)
		}
	}
}




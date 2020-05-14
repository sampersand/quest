pub mod operator;
pub use self::operator::Operator;
use crate::obj::types;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParenType {
	Paren, Bracket, Brace
}

impl From<ParenType> for types::Text {
	fn from(bracket: ParenType) -> Self {
		match bracket {
			ParenType::Paren => types::Text::new_static("()"),
			ParenType::Bracket => types::Text::new_static("[]"),
			ParenType::Brace => types::Text::new_static("{}"),
		}
	}
}


#[derive(Debug, PartialEq, Eq, Clone)]
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
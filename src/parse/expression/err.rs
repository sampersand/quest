use crate::parse::token::{ParenType, Operator, Token};

#[derive(Debug)]
pub enum Error {
	UnexpectedRightParen(ParenType),
	MissingRightParen(ParenType),
	MismatchedParen(ParenType, ParenType),
	UnexpectedOperator(Operator),
	UnexpectedToken(Token),
	NoTokens
}
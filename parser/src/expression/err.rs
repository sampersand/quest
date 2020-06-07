use crate::token::{ParenType, Operator, Token};

#[derive(Debug)]
// TODO: have this have a reference to the stream
pub enum Error {
	UnexpectedRightParen(ParenType),
	MissingRightParen(ParenType),
	MismatchedParen(ParenType, ParenType),
	UnexpectedOperator(Operator),
	UnexpectedToken(Token),
	NoTokens
}
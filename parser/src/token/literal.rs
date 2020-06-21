use crate::Result;
use crate::stream::Stream;
use crate::expression::{Constructable, Executable};
use crate::token::{Token, Tokenizable, TokenizeResult};
use std::fmt::{self, Display, Formatter};

mod text;
mod number;
mod variable;

use self::text::TextTokenizer;
use self::number::NumberTokenizer;
pub use self::variable::Variable;

pub type Text = <TextTokenizer as Tokenizable>::Item;
pub type Number = <NumberTokenizer as Tokenizable>::Item;
// pub type Variable = <VariableTokenizer as Tokenizable>::Item;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Literal {
	Text(Text),
	Number(Number),
	Variable(Variable)
}

impl Display for Literal {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Literal::Text(t) => Display::fmt(&t, f),
			Literal::Number(n) => Display::fmt(&n, f),
			Literal::Variable(v) => Display::fmt(&v, f),
		}
	}
}

impl Executable for Literal {
	fn execute(&self) -> quest::Result<quest::Object> {
		match self {
			Literal::Text(t) => t.execute(),
			Literal::Number(n) => n.execute(),
			Literal::Variable(v) => v.execute(),
		}
	}
}

impl From<Literal> for Token {
	fn from(lit: Literal) -> Token {
		Token::Literal(lit)
	}
}

impl Tokenizable for Literal {
	type Item = Self;
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Self>> {
		match NumberTokenizer::try_tokenize(stream)?.map(Literal::Number) {
			TokenizeResult::None => { /* do nothing, parse the next one */ },
			other => return Ok(other)
		}

		match TextTokenizer::try_tokenize(stream)?.map(Literal::Text) {
			TokenizeResult::None => { /* do nothing, parse the next one */ },
			other => return Ok(other)
		}

		Ok(Variable::try_tokenize(stream)?.map(Literal::Variable))
	}
}

impl Constructable for Literal {
	type Item = Self;
	fn try_construct_primary<C>(ctor: &mut C) -> Result<Option<Self>>
	where
		C: Iterator<Item=Result<Token>> + crate::expression::PutBack + crate::stream::Contexted
	{
		match ctor.next().transpose()? {
			Some(Token::Literal(lit)) => Ok(Some(lit)),
			Some(tkn) => { ctor.put_back(Ok(tkn)); Ok(None) }
			None => Ok(None),
		}
	}
}


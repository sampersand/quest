use crate::{Result, Stream};
use crate::token::{Token, Tokenizable, TokenizeResult};
use std::fmt::{self, Display, Formatter};

mod text;
mod number;
mod variable;

use self::text::Text;
use self::number::Number;
use self::variable::Variable;

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


impl From<Literal> for Token {
	fn from(lit: Literal) -> Token {
		Token::Literal(lit)
	}
}

impl Tokenizable for Literal {
	type Item = Self;
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Self>> {
		match Number::try_tokenize(stream)?.map(Literal::Number) {
			TokenizeResult::None => { /* do nothing, parse the next one */ },
			other => return Ok(other)
		}

		match Text::try_tokenize(stream)?.map(Literal::Text) {
			TokenizeResult::None => { /* do nothing, parse the next one */ },
			other => return Ok(other)
		}

		Ok(Variable::try_tokenize(stream)?.map(Literal::Variable))
	}
}




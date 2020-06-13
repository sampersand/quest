use std::io::BufRead;
use crate::Result;
use crate::token::{Token, Parsable, ParseResult};
use crate::stream::{BufStream, Stream};
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

impl Parsable for Literal {
	type Item = Self;
	fn try_parse_old<S: BufRead>(stream: &mut BufStream<S>) -> Result<ParseResult<Self::Item>> {
		match Number::try_parse_old(stream)?.map(Literal::Number) {
			ParseResult::None => { /* do nothing, parse the next one */ },
			other => return Ok(other)
		}

		match Text::try_parse_old(stream)?.map(Literal::Text) {
			ParseResult::None => { /* do nothing, parse the next one */ },
			other => return Ok(other)
		}

		Ok(Variable::try_parse_old(stream)?.map(Literal::Variable))
	}
}




use crate::Result;
use crate::stream::Stream;
use crate::expression::{Constructable, Executable};
use crate::token::{Token, Tokenizable, TokenizeResult};
use std::fmt::{self, Display, Formatter};

pub mod text;
pub mod number;
pub mod variable;

use self::text::TextTokenizer;
use self::number::NumberTokenizer;

/// A literal text.
pub type Text = text::Text;

/// A literal number.
pub type Number = number::Number;

/// A literal variable.
pub use self::variable::Variable;

/// Represents a literal value in Quest.
///
/// Due to the lack of keywords in quest, values such as `true` and `false` are not their own
/// distinct literal types: They're simply [`Variable`](#)s that will be evaluated at run time.
///
/// There are also no literal lists or maps: These are both considered [`Block`](#)s.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Literal {
	/// A literal piece of text.
	///
	/// See [`Text`](#) for more information on parsing.
	Text(Text),
	/// A literal number.
	///
	/// See [`Number`](#) for more information on parsing.
	Number(Number),
	/// A variable name.
	///
	/// See [`Variable`](#) for more information on parsing.
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


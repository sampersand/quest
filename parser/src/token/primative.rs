use crate::Result;
use crate::stream::Stream;
use crate::expression::{Constructable, Executable};
use crate::token::{Token, Tokenizable, TokenizeResult};
use std::fmt::{self, Display, Formatter};

pub mod text;
pub mod number;
pub mod variable;
pub mod stackpos;
pub mod regex;


/// A text literal.
pub use text::Text;

/// A number literal.
pub use number::Number;

/// A variable literal.
pub use variable::Variable;

/// A stackpos literal.
pub use stackpos::StackPos;

/// A regex literal.
pub use self::regex::Regex;


/// Represents a primative value in Quest.
///
/// Due to the lack of keywords in quest, values such as `true` and `false` are not their own
/// distinct literal types: They're simply [`Variable`](#)s that will be evaluated at run time.
///
/// There are also no literal lists or maps: These are both considered [`Block`](#)s.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Primative {
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
	Variable(Variable),

	Regex(Regex),
	StackPos(StackPos)
}

impl Display for Primative {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Primative::Text(t) => Display::fmt(&t, f),
			Primative::Number(n) => Display::fmt(&n, f),
			Primative::Variable(v) => Display::fmt(&v, f),
			Primative::Regex(r) => Display::fmt(&r, f),
			Primative::StackPos(s) => Display::fmt(&s, f),
		}
	}
}

impl Executable for Primative {
	fn execute(&self) -> quest_core::Result<quest_core::Object> {
		match self {
			Primative::Text(t) => t.execute(),
			Primative::Number(n) => n.execute(),
			Primative::Variable(v) => v.execute(),
			Primative::Regex(r) => r.execute(),
			Primative::StackPos(s) => s.execute(),
		}
	}
}

impl From<Primative> for Token {
	fn from(lit: Primative) -> Token {
		Token::Primative(lit)
	}
}

impl Tokenizable for Primative {
	type Item = Self;
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Self>> {
		match Variable::try_tokenize(stream)?.map(Primative::Variable) {
			TokenizeResult::None => { /* do nothing, parse the next one */ },
			other => return Ok(other)
		}

		match Number::try_tokenize(stream)?.map(Primative::Number) {
			TokenizeResult::None => { /* do nothing, parse the next one */ },
			other => return Ok(other)
		}

		match Text::try_tokenize(stream)?.map(Primative::Text) {
			TokenizeResult::None => { /* do nothing, parse the next one */ },
			other => return Ok(other)
		}

		match Regex::try_tokenize(stream)?.map(Primative::Regex) {
			TokenizeResult::None => { /* do nothing, parse the next one */ },
			other => return Ok(other)
		}

		Ok(StackPos::try_tokenize(stream)?.map(Primative::StackPos))
	}
}

impl Constructable for Primative {
	type Item = Self;
	fn try_construct_primary<C>(ctor: &mut C) -> Result<Option<Self>>
	where
		C: Iterator<Item=Result<Token>> + crate::expression::PutBack + crate::stream::Contexted
	{
		match ctor.next().transpose()? {
			Some(Token::Primative(lit)) => Ok(Some(lit)),
			Some(tkn) => { ctor.put_back(Ok(tkn)); Ok(None) }
			None => Ok(None),
		}
	}
}


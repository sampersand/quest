use crate::Result;
use crate::stream::Stream;
use crate::expression::{Constructable, Executable};
use crate::token::{Token, Tokenizable};
use std::fmt::{self, Display, Formatter};
use super::{text::Text, number::Number, variable::Variable, stackpos::StackPos, regex::Regex};


/// Represents a primative value in Quest.
///
/// Due to the lack of keywords in quest, values such as `true` and `false` are not their own
/// distinct literal types: They're simply [`Variable`](#)s that will be evaluated at run time.
///
/// There are also no literal lists or maps: These are both considered [`Block`](#)s.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Primative {
	/// A [`Text`] literal.
	Text(Text),

	/// A [`Number`] literal.
	Number(Number),

	/// A [`Variable`] literal.
	Variable(Variable),

	/// A [`Regex`] literal.
	Regex(Regex),

	/// A [`Stackpos`] literal.
	StackPos(StackPos)
}

impl Display for Primative {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::Text(t) => Display::fmt(&t, f),
			Self::Number(n) => Display::fmt(&n, f),
			Self::Variable(v) => Display::fmt(&v, f),
			Self::Regex(r) => Display::fmt(&r, f),
			Self::StackPos(s) => Display::fmt(&s, f),
		}
	}
}

impl Executable for Primative {
	fn execute(&self) -> quest_core::Result<quest_core::Object> {
		match self {
			Self::Text(t) => t.execute(),
			Self::Number(n) => n.execute(),
			Self::Variable(v) => v.execute(),
			Self::Regex(r) => r.execute(),
			Self::StackPos(s) => s.execute(),
		}
	}
}

impl From<Primative> for Token {
	fn from(lit: Primative) -> Token {
		Self::Primative(lit)
	}
}

impl Tokenizable for Primative {
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<Option<Self>> {
		// TODO: make this more idiomatic rust

		if let token @ Some(_) = Variable::try_tokenize(stream)?.map(Self::Variable) {
			Ok(token)
		} else if let token @ Some(_) = Number::try_tokenize(stream)?.map(Self::Number) {
			Ok(token)
		} else if let token @ Some(_) = Text::try_tokenize(stream)?.map(Self::Text) {
			Ok(token)
		} else if let token @ Some(_) = Regex::try_tokenize(stream)?.map(Self::Regex) {
			Ok(token)
		} else {
			Ok(StackPos::try_tokenize(stream)?.map(Self::StackPos))
		}
	}
}

impl Constructable for Primative {
	fn try_construct_primary<C>(ctor: &mut C) -> Result<Option<Self>>
	where
		C: Iterator<Item=Result<Token>> + crate::expression::PutBack + crate::stream::Contexted
	{
		match ctor.next().transpose()? {
			Some(Token::Primative(lit)) => Ok(Some(lit)),
			Some(tkn) => {
				ctor.put_back(Ok(tkn));
				Ok(None)
			},
			None => Ok(None)
		}
	}
}

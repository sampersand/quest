//! Parsing a literal text

use crate::{Result, Stream};
use crate::expression::Executable;
use crate::token::{Operator, Tokenizable, primitive::Variable};
use quest_core::Object;
use std::fmt::{self, Debug, Display, Formatter};

/// A literal text is actually just a `quest_core::Text`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Text(quest_core::types::Text, Quote);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Quote { Single, Double, Dollar }

impl Display for Quote {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&char::from(*self), f)
	}
}

impl From<Quote> for char {
	fn from(q: Quote) -> Self {
		match q {
			Quote::Single => '\'',
			Quote::Double => '"',
			Quote::Dollar => '$',
		}
	}
}


impl Executable for Text {
	fn execute(&self) -> quest_core::Result<Object> {
		Ok(self.0.clone().into())
	}
}

impl Display for Text {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if self.1 == Quote::Dollar {
			write!(f, "{0}{1}", self.1, self.0)
		} else {
			write!(f, "{0}{1}{0}", self.1, self.0)
		}
	}
}

fn try_tokenize_quoted<S: Stream>(stream: &mut S, quote: Quote) -> Result<Option<Text>> {
	let mut text = String::new();

	let starting_context = stream.context().clone();

	while let Some(chr) = stream.next().transpose()? {
		match chr {
			'\\' => match stream.next().transpose()? {
				Some(chr @ '\\')
					| Some(chr @ '\'')
					| Some(chr @ '\"') => text.push(chr),
				Some('n') => text.push('\n'),
				Some('\n') => { /* do nothing */ },
				Some('t') => text.push('\t'),
				Some('r') => text.push('\r'),
				Some('0') => text.push('\0'),
				Some('u') | Some('U')
					| Some('x') | Some('X') => todo!("additional string parsing"),
				Some(chr) => return Err(parse_error!(stream, BadEscapeChar(chr))),
				None      => return Err(parse_error!(context=starting_context, UnterminatedQuote)),
			},
			chr if chr == char::from(quote) => return Ok(Some(Text(text.into(), quote))),
			chr => text.push(chr)
		}
	}

	Err(parse_error!(context=starting_context, UnterminatedQuote))
}

// valid syntax is `$variable_name` or `$operator`.
fn try_tokenize_dollar_sign<S: Stream>(stream: &mut S) -> Result<Option<Text>> {
	const QUOTE: Quote = Quote::Dollar;
	macro_rules! from_other {
		($($p:ty),*) => {
			$(
				match <$p>::try_tokenize(stream)?
					.map(|val| Text(val.to_string().into(), QUOTE))
				{
					v @ Some(_) => return Ok(v),
					None => {},
				}
			)*
		};
	}

	if stream.next_if_starts_with("-@")? {
		return Ok(Some(Text("-@".into(), QUOTE)));
	} else if stream.next_if_starts_with("+@")? {
		return Ok(Some(Text("+@".into(), QUOTE)));
	}

	from_other!(Variable, Operator);

	if stream.next_if_starts_with("()")? {
		Ok(Some(Text("()".into(), QUOTE)))
	} else {
		Err(parse_error!(stream, UnterminatedQuote))
	}
}

impl Tokenizable for Text {
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<Option<Self>> {
		match stream.next().transpose()? {
			Some('$') => try_tokenize_dollar_sign(stream),
			Some('\'') => try_tokenize_quoted(stream, Quote::Single),
			Some('\"') => try_tokenize_quoted(stream, Quote::Double),
			Some(chr) => {
				unseek_char!(stream; chr);
				Ok(None)
			},
			None => Ok(None)
		}
	}
}

//! Parsing a literal text

use crate::{Result, Stream};
use crate::expression::Executable;
use crate::token::{Operator, Tokenizable, primitive::Variable};
use quest_core::Object;

/// A literal text is actually just a `quest_core::Text`.
pub use quest_core::types::Text;


impl Executable for Text {
	fn execute(&self) -> quest_core::Result<Object> {
		Ok(self.clone().into())
	}
}

fn try_tokenize_quoted<S: Stream>(stream: &mut S, quote: char) -> Result<Option<Text>> {
	let mut text = String::new();

	let starting_context = stream.context().clone();

	while let Some(chr) = stream.next().transpose()? {
		match chr {
			'\\' if quote == '"' => match stream.next().transpose()? {
				Some(chr @ '\\')
					| Some(chr @ '\'')
					| Some(chr @ '\"') => text.push(chr),
				Some('n') => text.push('\n'),
				Some('\n') => { /* do nothing */ },
				Some('\r') => { stream.next_if_starts_with("\n")?; },
				Some('t') => text.push('\t'),
				Some('r') => text.push('\r'),
				Some('0') => text.push('\0'),
				Some('u') | Some('U')
					| Some('x') | Some('X') => todo!("additional string parsing"),
				Some(chr) => return Err(parse_error!(stream, BadEscapeChar(chr))),
				None      => return Err(parse_error!(context=starting_context, UnterminatedQuote)),
			},
			'\\' => match stream.next().transpose()? {
				Some(chr @ '\\') | Some(chr @ '\'') => text.push(chr),
				Some(other) => { text.push('\\'); text.push(other); },
				None => return Err(parse_error!(context=starting_context, UnterminatedQuote))
			},
			chr if chr == quote => return Ok(Some(text.into())),
			chr => text.push(chr)
		}
	}

	Err(parse_error!(context=starting_context, UnterminatedQuote))
}

// valid syntax is `$variable_name` or `$operator`.
fn try_tokenize_dollar_sign<S: Stream>(stream: &mut S) -> Result<Option<Text>> {
	macro_rules! from_other {
		($($p:ty),*) => {
			$(
				match <$p>::try_tokenize(stream)?.map(|val| val.to_string().into()) {
					v @ Some(_) => return Ok(v),
					None => {},
				}
			)*
		};
	}

	if stream.next_if_starts_with("-@")? {
		return Ok(Some("-@".into()));
	} else if stream.next_if_starts_with("+@")? {
		return Ok(Some("+@".into()));
	}

	from_other!(Variable, Operator);

	if stream.next_if_starts_with("()")? {
		Ok(Some("()".into()))
	} else {
		Err(parse_error!(stream, UnterminatedQuote))
	}
}

impl Tokenizable for Text {
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<Option<Self>> {
		match stream.next().transpose()? {
			Some('$') => try_tokenize_dollar_sign(stream),
			Some(quote @ '\"') | Some(quote @ '\'') => try_tokenize_quoted(stream, quote),
			Some(chr) => {
				unseek_char!(stream; chr);
				Ok(None)
			},
			None => Ok(None)
		}
	}
}

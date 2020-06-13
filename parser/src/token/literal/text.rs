use crate::{Result, Stream};
use crate::stream::Contexted;
use crate::token::{Operator, Parenthesis, Tokenizable, TokenizeResult};
use crate::token::literal::{variable, Variable};
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text(quest::types::Text);


impl Display for Text {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(&self.0.as_ref(), f)
	}
}

impl Text {
	fn try_tokenize_quoted<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Self>> {
		let mut text = String::new();
		let quote = stream.next().expect("internal error: a string should have been passed")?;
		debug_assert!(quote == '"' || quote == '\'');

		let starting_context = stream.context().clone();

		while let Some(chr) = stream.next().transpose()? {
			match chr {
				'\\' => match stream.next().transpose()? {
					Some(chr @ '\\')
						| Some(chr @ '\'')
						| Some(chr @ '\"') => text.push(chr),
					Some('n') => text.push('\n'),
					Some('t') => text.push('\t'),
					Some('r') => text.push('\r'),
					Some('0') => text.push('\0'),
					Some('u') | Some('U')
						| Some('x') | Some('X') => todo!("additional string parsing"),
					Some(chr) => return Err(parse_error!(stream, BadEscapeChar(chr))),
					None      => return Err(parse_error!(context=starting_context, UnterminatedQuote)),
				},
				chr if chr == quote => return Ok(TokenizeResult::Some(Text(text.into()))),
				chr => text.push(chr)
			}
		}

		Err(parse_error!(context=starting_context, UnterminatedQuote))
	}

	// valid syntax is `$variable_name` or `$operator`.
	fn try_tokenize_dollar_sign<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Self>> {
		assert_eq!(stream.next().transpose()?, Some('$'));

		let first = stream.peek().unwrap_or_else(|| Err(parse_error!(stream, UnterminatedQuote)))?;

		macro_rules! from_other {
			($($p:ty),*) => {
				$(
					match <$p>::try_tokenize(stream)?.map(|val| Text(val.to_string().into())) {
						v @ TokenizeResult::Some(_) => return Ok(v),
						TokenizeResult::None => {},
						_ => return Err(parse_error!(stream, UnterminatedQuote))
					}
				)*
			};
		}

		from_other!(Variable, Operator, Parenthesis);

		Err(parse_error!(stream, UnterminatedQuote))
	}
}


impl Tokenizable for Text {
	type Item = Self;
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Self>> {
		match stream.peek().transpose()? {
			Some('$')               => Text::try_tokenize_dollar_sign(stream),
			Some('\"') | Some('\'') => Text::try_tokenize_quoted(stream),
			_ => Ok(TokenizeResult::None)
		}
	}
}





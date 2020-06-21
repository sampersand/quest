use crate::{Result, Stream};
use crate::expression::Executable;
use crate::token::{Operator, Tokenizable, TokenizeResult};
use crate::token::literal::Variable;
use quest::{Object, types::Text};

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct TextTokenizer;

impl Executable for Text {
	fn execute(&self) -> quest::Result<Object> {
		Ok(self.clone().into())
	}
}

impl TextTokenizer {
	fn try_tokenize_quoted<S: Stream>(stream: &mut S, quote: char) -> Result<TokenizeResult<Text>> {
		let mut text = String::new();

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
				chr if chr == quote => return Ok(TokenizeResult::Some(text.into())),
				chr => text.push(chr)
			}
		}

		Err(parse_error!(context=starting_context, UnterminatedQuote))
	}

	// valid syntax is `$variable_name` or `$operator`.
	fn try_tokenize_dollar_sign<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Text>> {
		assert_eq!(stream.next().transpose()?, Some('$'));

		macro_rules! from_other {
			($($p:ty),*) => {
				$(
					match <$p>::try_tokenize(stream)?.map(|val| val.to_string().into()) {
						v @ TokenizeResult::Some(_) => return Ok(v),
						TokenizeResult::None => {},
						_ => return Err(parse_error!(stream, UnterminatedQuote))
					}
				)*
			};
		}

		from_other!(Variable, Operator);

		if stream.next_if_starts_with("()")? {
			Ok(TokenizeResult::Some("()".into()))
		} else {
			Err(parse_error!(stream, UnterminatedQuote))
		}
	}
}

impl Tokenizable for TextTokenizer {
	type Item = Text;
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Text>> {
		match stream.next().transpose()? {
			Some('$') => TextTokenizer::try_tokenize_dollar_sign(stream),
			Some(quote @ '\"') | Some(quote @ '\'') =>
				TextTokenizer::try_tokenize_quoted(stream, quote),
			_ => {
				try_seek!(stream, -1);
				Ok(TokenizeResult::None)
			}
		}
	}
}

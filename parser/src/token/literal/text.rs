use crate::{Stream, Result};
use crate::token::{Operator, Parenthesis, Parsable, ParseResult};
use crate::token::literal::{variable, Variable};
use std::io::BufRead;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text(quest::types::Text);


impl Display for Text {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(&self.0.as_ref(), f)
	}
}

impl Text {
	fn try_parse_quoted<S: BufRead>(stream: &mut Stream<S>) -> Result<ParseResult<Self>> {
		let mut text = String::new();
		let quote = stream.next_char()?.expect("internal error: a string should have been passed");
		debug_assert!(quote == '"' || quote == '\'');
		let starting_context = stream.context().clone();

		while let Some(chr) = stream.next_char()? {
			match chr {
				'\\' => match stream.next_char()? {
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
				chr if chr == quote => return Ok(ParseResult::Some(Text(text.into()))),
				chr => text.push(chr)
			}
		}

		Err(parse_error!(context=starting_context, UnterminatedQuote))
	}

	// valid syntax is `$variable_name` or `$operator`.
	fn try_parse_dollar_sign<S: BufRead>(stream: &mut Stream<S>) -> Result<ParseResult<Self>> {
		assert_eq!(stream.next_char()?, Some('$'));

		let first = stream.peek_char()?.ok_or_else(|| parse_error!(stream, UnterminatedQuote))?;

		if variable::is_variable_start(first) {
			Ok(Variable::try_parse(stream)?.map(|var| Text(var.to_string().into())))
		} else {
			match Operator::try_parse(stream)?.map(|op| Text(op.to_string().into())) {
				ParseResult::Some(val) => return Ok(ParseResult::Some(val)),
				ParseResult::None =>
					if let ParseResult::Some(val) = Parenthesis::try_parse(stream)?
							.map(|p| Text(p.to_string().into())) {
						return Ok(ParseResult::Some(val));
					},
				_ => {}
			}

			Err(parse_error!(stream, UnterminatedQuote))
		}
	}
}


impl Parsable for Text {
	type Item = Self;
	fn try_parse<S: BufRead>(stream: &mut Stream<S>) -> Result<ParseResult<Self>> {
		match stream.peek_char()? {
			Some('$') => Text::try_parse_dollar_sign(stream),
			Some('"') | Some('\'') => Text::try_parse_quoted(stream),
			_ => Ok(ParseResult::None)
		}
	}
}


use std::io::BufRead;
use crate::token::{Result, Parsable, ParseResult};
use crate::Stream;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable(quest::types::Text);

impl Display for Variable {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0.as_ref(), f)
	}
}


#[inline]
pub fn is_variable_start(c: char) -> bool {
	!c.is_ascii() || c.is_ascii_alphabetic() || c == '_' || c == '@'
}

#[inline]
pub fn is_variable_body(c: char) -> bool {
	is_variable_start(c) || c.is_ascii_digit()
}

impl Parsable for Variable {
	type Item = Self;
	fn try_parse<S: BufRead>(stream: &mut Stream<S>) -> Result<ParseResult<Self>> {
		if !stream.peek_char()?.map(is_variable_start).unwrap_or(false) {
			return Ok(ParseResult::None)
		}

		let mut variable = String::with_capacity(1);
		variable.push(stream.next_char()?.expect("internal error: first char should be available"));
		debug_assert!(is_variable_start(variable.chars().next().unwrap()));

		while let Some(chr) = stream.next_char()? {
			if is_variable_body(chr) {
				variable.push(chr)
			} else {
				stream.unshift_char(chr);
				break;
			}
		}

		Ok(ParseResult::Some(Variable(variable.into())))
	}
}

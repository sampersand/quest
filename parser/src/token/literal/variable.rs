use crate::{Result, Stream};
use crate::token::{Parsable, ParseResult};
use std::io::{Seek, SeekFrom};
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
	fn try_parse<S: Stream>(stream: &mut S) -> Result<ParseResult<Self>> {
		if !stream.peek_char()?.map(is_variable_start).unwrap_or(false) {
			return Ok(ParseResult::None)
		}

		let mut variable = String::with_capacity(1);

		while let Some(chr) = stream.next().transpose()? { 
			if is_variable_body(chr) {
				variable.push(chr)
			} else if let Err(err) = stream.seek(SeekFrom::Current(-1)) {
				return Err(parse_error!(stream, CantReadStream(err)));
			} else {
				break;
			}
		}

		Ok(ParseResult::Some(Variable(variable.into())))
	}
}


use std::io::BufRead;
use crate::token::{Result, Parsable, ParseResult};
use crate::Stream;

// a dummy struct just so we can have a type to impl `Parsable`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Whitespace;

pub enum Never {}
impl From<Never> for super::Token {
	fn from(_: Never) -> Self {
		unreachable!()
	}
}

impl Parsable for Whitespace {
	type Item = Never;
	fn try_parse<S: BufRead>(stream: &mut Stream<S>) -> Result<ParseResult<Never>> {
		if stream.peek_char()?.map(char::is_whitespace).unwrap_or(false) {
			while let Some(chr) = stream.next_char()? {
				if !chr.is_whitespace() {
					stream.unshift_char(chr);
					return Ok(ParseResult::RestartParsing);
				}
			}
		}

		Ok(ParseResult::None)
	}
}

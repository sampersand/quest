use std::io::BufRead;
use crate::token::{Result, Parsable, ParseResult};
use crate::Stream;
pub use super::whitespace::Never;

// a dummy struct just so we can have a type to impl `Parsable`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Comment;

fn line_comment<S: BufRead>(stream: &mut Stream<S>) -> Result<()> {
	while let Some(chr) = stream.next_char()? {
		if chr == '\n' {
			break;
		}
	}
	Ok(())
}

fn block_comment<S: BufRead>(stream: &mut Stream<S>) -> Result<()> {
	let begin_context = stream.context().clone();

	while let Some(chr) = stream.next_char()? {
		match chr {
			'*' if stream.peek_char()? == Some('/') => {
				assert_eq!(stream.next_char().unwrap(), Some('/'));
				return Ok(()); // end of line
			},
			'/' if stream.peek_char()? == Some('*') => {
				assert_eq!(stream.next_char().unwrap(), Some('*'));
				block_comment(stream)?; // allow for nested block comments
			},
			_ => { /* do nothing, we ignore other characters */ }
		}
	}
	Err(parse_error!(context=begin_context, UnterminatedBlockComment))
}

impl Parsable for Comment {
	type Item = Never;
	fn try_parse<S: BufRead>(stream: &mut Stream<S>) -> Result<ParseResult<Never>> {
		match stream.next_char()? {
			Some('#') => { 
				line_comment(stream)?;
				Ok(ParseResult::RestartParsing)
			},

			Some('/') if stream.peek_char()? == Some('*') => {
				assert_eq!(stream.next_char().unwrap(), Some('*'));
				block_comment(stream)?;
				Ok(ParseResult::RestartParsing)
			},
			Some(chr) => {
				stream.unshift_char(chr);
				Ok(ParseResult::None)
			},
			None => Ok(ParseResult::None)
		}
	}
}



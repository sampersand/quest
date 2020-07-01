//! Parsing a literal text

use crate::{Result, Stream};
use crate::expression::Executable;
use crate::token::{Tokenizable, TokenizeResult};

use quest_core::Object;

/// A literal text is actually just a `quest_core::Text`.
pub use quest_core::types::Text;


impl Executable for Text {
	fn execute(&self) -> quest_core::Result<Object> {
		Ok(self.clone().into())
	}
}


#[inline]
fn is_unquoted_start(c: char) -> bool {
	!c.is_ascii() || c.is_ascii_alphabetic() || c == '_' || c == '@'
}

#[inline]
fn is_unquoted_body(c: char) -> bool {
	is_unquoted_start(c) || c.is_ascii_digit()
}

fn try_tokenize_unquoted<S: Stream>(stream: &mut S, first: char) -> Result<Text> {
	let mut text = first.to_string();

	while let Some(chr) = stream.next().transpose()? { 
		if is_unquoted_body(chr) {
			text.push(chr)
		} else {
			try_seek!(stream, -1);
			break;
		}
	}

	text.shrink_to_fit();

	Ok(text.into())
}

fn try_tokenize_quoted<S: Stream>(stream: &mut S, quote: char) -> Result<Text> {
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
			chr if chr == quote => return Ok(text.into()),
			chr => text.push(chr)
		}
	}

	Err(parse_error!(context=starting_context, UnterminatedQuote))
}

impl Tokenizable for Text {
	type Item = Self;
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Self>> {
		match stream.next().transpose()? {
			Some(chr) if is_unquoted_start(chr) =>
				try_tokenize_unquoted(stream, chr).map(TokenizeResult::Some),
			Some(quote @ '\"') | Some(quote @ '\'') =>
				try_tokenize_quoted(stream, quote).map(TokenizeResult::Some),
			Some(_) => {
				try_seek!(stream, -1);
				Ok(TokenizeResult::None)
			},
			None => Ok(TokenizeResult::None)
		}
	}
}
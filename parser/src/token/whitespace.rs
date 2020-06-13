use crate::token::{Tokenizable, TokenizeResult};
use crate::{Result, Stream};

// a dummy struct just so we can have a type to impl `Tokenizable`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Whitespace;

pub enum Never {}
impl From<Never> for super::Token {
	fn from(_: Never) -> Self {
		unreachable!()
	}
}

impl Tokenizable for Whitespace {
	type Item = Never;
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Never>> {
		if stream.peek().transpose()?.map(char::is_whitespace).unwrap_or(false) {
			while let Some(chr) = stream.next().transpose()? {
				if !chr.is_whitespace() {
					try_seek!(stream, -1);
					return Ok(TokenizeResult::RestartParsing);
				}
			}
		}

		Ok(TokenizeResult::None)
	}
}

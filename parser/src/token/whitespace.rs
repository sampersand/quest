use crate::token::{Tokenizable, TokenizeResult};
use crate::{Result, Stream};

// a dummy struct just so we can have a type to impl `Tokenizable`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Whitespace;

impl Tokenizable for Whitespace {
	type Item = !;
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<TokenizeResult<!>> {
		match stream.next().transpose()? {
			Some(chr) if chr.is_whitespace() =>
				while let Some(chr) = stream.next().transpose()? {
					if !chr.is_whitespace() {
						unseek_char!(stream; chr);
						return Ok(TokenizeResult::RestartParsing);
					}
				},
			Some(chr) => unseek_char!(stream; chr),
			None => {}
		}

		Ok(TokenizeResult::None)
	}
}

use crate::{Result, Stream};
use crate::token::Tokenizable;
use crate::expression::Executable;

use quest_core::types::regex::{Flag, Flags};
pub use quest_core::types::{Regex, regex::RegexError};


impl Executable for Regex {
	#[inline]
	fn execute(&self) -> quest_core::Result<quest_core::Object> {
		Ok(self.clone().into())
	}
}

impl Tokenizable for Regex {
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<Option<Self>> {
		let mut rxp =
			match stream.next().transpose()? {
				Some('/') => {
					match stream.next().transpose()? {
						Some(chr) if chr.is_ascii_whitespace() => {
							unseek_char!(stream; '/', chr);
							return Ok(None)
						},
						Some('/') => {
							unseek_char!(stream; '/');
							String::default()
						},
						Some(chr) => chr.to_string(),
						None => {
							unseek_char!(stream; '/');
							return Ok(None)
						}
					}
				},
				Some(chr) => {
					unseek_char!(stream; chr);
					return Ok(None)
				},

				None => return Ok(None)
			};

		while let Some(chr) = stream.next().transpose()? { 
			match chr {
				'\\' => 
					match stream.next().transpose()? {
						Some(chr @ '/') | Some(chr @ '\\') => rxp.push(chr),
						Some(other) => {
							rxp.push('\\');
							rxp.push(other);
						},
						None => return Err(parse_error!(stream, UnterminatedQuote))
					},
				'/' => break,
				chr => rxp.push(chr)
			}
		}

		let mut flags = Flags::new();

		while let Some(chr) = stream.next().transpose()? {
			match chr {
				'i' => flags.set(Flag::CaseInsensitive),
				'm' => flags.set(Flag::MultiLine),
				's' => flags.set(Flag::DotMatchNewLine),
				'U' => flags.set(Flag::SwapGreedy),
				'x' => flags.set(Flag::IgnoreWhitespace),
				chr => {
					unseek_char!(stream; chr);
					break
				}
			};
		}

		Regex::new_with_options(&rxp, flags)
			.map(Some)
			.map_err(|err| parse_error!(stream, BadRegex(err)))
	}
}


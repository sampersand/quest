use crate::{Result, Stream};
use crate::token::Tokenizable;
use crate::expression::Executable;

use quest_core::types::regex::Flags;
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

		let mut flags = Flags::empty();

		while let Some(chr) = stream.next().transpose()? {
			match chr {
				'i' => flags.insert(Flags::CASE_INSENSITIVE),
				'm' => flags.insert(Flags::MULTI_LINE),
				's' => flags.insert(Flags::DOT_MATCH_NEWLINE),
				'U' => flags.insert(Flags::SWAP_GREEDY),
				'x' => flags.insert(Flags::IGNORE_WHITESPACE),
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


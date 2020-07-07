use crate::{Result, Stream};
use crate::token::{Tokenizable, TokenizeResult};
use crate::expression::Executable;

pub use quest_core::types::Regex;
pub use quest_core::types::regex::RegexError;


impl Executable for Regex {
	#[inline]
	fn execute(&self) -> quest_core::Result<quest_core::Object> {
		Ok(self.clone().into())
	}
}

impl Tokenizable for Regex {
	type Item = Self;
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Self>> {
		let mut regex =
			match stream.next().transpose()? {
				Some('/') => {
					match stream.next().transpose()? {
						Some(chr) if chr.is_ascii_whitespace() => {
							try_seek!(stream, -2);
							return Ok(TokenizeResult::None)
						},
						Some('/') => { try_seek!(stream, -1); String::default() },
						Some(other) => other.to_string(),
						None => { try_seek!(stream, -1); return Ok(TokenizeResult::None) }
					}
				},
				Some(_) => { try_seek!(stream, -1); return Ok(TokenizeResult::None) },
				None => return Ok(TokenizeResult::None)
			};

		while let Some(chr) = stream.next().transpose()? { 
			match chr {
				'\\' => 
					match stream.next().transpose()? {
						Some(chr @ '/') | Some(chr @ '\\') => regex.push(chr),
						Some(other) => { regex.push('\\'); regex.push(other); },
						None => return Err(parse_error!(stream, UnterminatedQuote))
					},
				'/' => break,
				_ => regex.push(chr)
			}
		}

		let mut opts = String::default();

		let mut regex_builder = ::regex::RegexBuilder::new(&regex);
		while let Some(chr) = stream.next().transpose()? {
			if opts.contains(chr) {
				continue;
			}

			match chr {
				'i' => regex_builder.case_insensitive(true),
				'm' => regex_builder.multi_line(true),
				's' => regex_builder.dot_matches_new_line(true),
				'U' => regex_builder.swap_greed(true),
				'x' => regex_builder.ignore_whitespace(true),
				_ => { try_seek!(stream, -1); break }
			};

			opts.push(chr);
		}

		regex_builder.build()
			.map(|re| Regex::new_with_options(re, opts))
			.map(TokenizeResult::Some)
			.map_err(|err| parse_error!(stream, BadRegex(err)))
	}
}


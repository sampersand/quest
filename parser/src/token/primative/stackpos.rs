use crate::{Result, Stream};
use crate::token::{Tokenizable, TokenizeResult};
use quest_core::Binding;
use crate::expression::Executable;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StackPos(isize);

impl Display for StackPos {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, ":{}", self.0)
	}
}

impl Executable for StackPos {
	#[inline]
	fn execute(&self) -> quest_core::Result<quest_core::Object> {
		let stack = Binding::stack();
		let len = stack.len();
		match quest_core::utils::correct_index(self.0, len) {
			Some(idx) => Ok(stack[idx].clone().into()),
			None => Err(quest_core::error::KeyError::OutOfBounds{ idx: self.0, len }.into())
		}
	}
}

impl Tokenizable for StackPos {
	type Item = Self;
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Self>> {
		match stream.next().transpose()? {
			Some(':') => 
				match stream.next().transpose()? {
					// oops! this is the `::` operator
					Some(':') => {
						try_seek!(stream, -2);
						return Ok(TokenizeResult::None);
					},
					Some(_) => try_seek!(stream, -1),
					None => {}
				},
			Some(_) => {
				try_seek!(stream, -1);
				return Ok(TokenizeResult::None)
			},
			None => return Ok(TokenizeResult::None)
		};


		fn next_non_underscore<S: Stream>(stream: &mut S) -> Result<Option<char>> {
			match stream.next().transpose()? {
				Some('_') => next_non_underscore(stream),
				Some(chr) => Ok(Some(chr)),
				None => Ok(None)
			}
		}


		let mut pos = String::with_capacity(1);

		match next_non_underscore(stream)? {
			Some(chr @ '-') | Some(chr @ '+') => { 
				pos.push(chr);
				match next_non_underscore(stream)? {
					Some(chr) if chr.is_ascii_digit() => pos.push(chr),
					_ => return Err(parse_error!(stream, Message("unexpected end of stack pos literal")))
				}
			},
			Some(chr) if chr.is_ascii_digit() => pos.push(chr),
			_ => return Err(parse_error!(stream, Message("unexpected end of stack pos literal")))
		}

		while let Some(chr) = next_non_underscore(stream)? { 
			if chr.is_ascii_digit() {
				pos.push(chr)
			} else {
				try_seek!(stream, -1);
				break;
			}
		}

		use std::str::FromStr;

		match isize::from_str(&pos) {
			Ok(pos) => Ok(TokenizeResult::Some(StackPos(pos))),
			Err(err) => Err(parse_error!(stream,
				MessagedString(format!("invalid stack pos literal: {}", err))))
		}
	}
}

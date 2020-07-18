use crate::{Result, Stream};
use crate::token::Tokenizable;
use quest_core::Binding;
use crate::expression::Executable;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StackPos(isize);

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
	InvalidLiteral(std::num::ParseIntError),
}

impl Display for Error {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::InvalidLiteral(ref err) => Display::fmt(err, f)
		}
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Self::InvalidLiteral(err) => Some(err)
		}
	}
}

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
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<Option<Self>> {
		let mut pos =
			match stream.next().transpose()? {
				Some(':') => 
					match stream.next().transpose()? {
						Some(chr @ '-')
							| Some(chr @ '+') 
							| Some(chr @ '0'..='9') => chr.to_string(),
						Some(other) => {
							unseek_char!(stream; other, ':');
							return Ok(Option::None)
						},
						None => {
							unseek_char!(stream; ':');
							return Ok(Option::None)
						}
					},
				Some(chr) => {
					unseek_char!(stream; chr);
					return Ok(Option::None)
				},
				None => return Ok(Option::None)
			};

		while let Some(chr) = stream.next_non_underscore().transpose()? { 
			match chr {
				chr @ '0'..='9' => pos.push(chr),
				chr => {
					unseek_char!(stream; chr);
					break
				}
			}
		}

		use std::str::FromStr;

		match isize::from_str(&pos) {
			Ok(pos) => Ok(Some(Self(pos))),
			Err(err) => Err(parse_error!(stream, CantTokenize(Error::InvalidLiteral(err).into())))
		}
	}
}

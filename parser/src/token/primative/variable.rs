use crate::{Result, Stream};
use crate::token::Tokenizable;
use crate::expression::Executable;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Variable(quest_core::types::Text);

impl Display for Variable {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl Executable for Variable {
	#[inline]
	fn execute(&self) -> quest_core::Result<quest_core::Object> {
		self.0.evaluate()
	}
}

#[inline]
fn is_variable_start(c: char) -> bool {
	!c.is_ascii() || c.is_ascii_alphabetic() || c == '_' || c == '@'
}

#[inline]
fn is_variable_body(c: char) -> bool {
	is_variable_start(c) || c.is_ascii_digit()
}

impl Tokenizable for Variable {
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<Option<Self>> {
		let mut variable =
			match stream.next().transpose()? {
				Some(chr) if is_variable_start(chr) => chr.to_string(),
				Some(chr) => {
					unseek_char!(stream; chr);
					return Ok(None)
				},
				None => return Ok(None)
			};

		while let Some(chr) = stream.next().transpose()? { 
			if is_variable_body(chr) {
				variable.push(chr)
			} else {
				unseek_char!(stream; chr);
				break;
			}
		}

		variable.shrink_to_fit();

		Ok(Some(Self(variable.into())))
	}
}


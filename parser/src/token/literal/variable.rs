use crate::{Result, Stream};
use crate::token::{Tokenizable, TokenizeResult};
use crate::expression::Executable;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable(quest::types::Text);

impl Display for Variable {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0.as_ref(), f)
	}
}


impl Executable for Variable {
	fn execute(&self) -> quest::Result<quest::Object> {
		quest::Object::from(self.0.clone())
			.call_attr(&crate::token::Operator::Call, vec![])
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
	type Item = Self;
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Self>> {
		if !stream.peek().transpose()?.map(is_variable_start).unwrap_or(false) {
			return Ok(TokenizeResult::None)
		}

		let mut variable = String::with_capacity(1);

		while let Some(chr) = stream.next().transpose()? { 
			use std::io::{Seek, SeekFrom};

			if is_variable_body(chr) {
				variable.push(chr)
			} else {
				try_seek!(stream, -1);
				break;
			}
		}

		Ok(TokenizeResult::Some(Variable(variable.into())))
	}
}


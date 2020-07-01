use crate::{Result, Stream};
use crate::token::{Tokenizable, TokenizeResult};
use crate::expression::Executable;
use std::fmt::{self, Display, Formatter};
use super::Text;

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


impl Tokenizable for Variable {
	type Item = Self;
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Self>> {
		match stream.next().transpose()? {
			Some('$') =>
				if let TokenizeResult::Some(text) = Text::try_tokenize(stream)? {
					Ok(TokenizeResult::Some(Variable(text)))
				} else {
					Err(parse_error!(stream, UnterminatedQuote))
				},
			Some(_) => {
				try_seek!(stream, -1);
				Ok(TokenizeResult::None)
			},
			None => Ok(TokenizeResult::None)
		}
	}
}


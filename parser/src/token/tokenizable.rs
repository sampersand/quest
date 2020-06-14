use crate::Result;
use crate::stream::Stream;

// "TokenizeResult" is probably not the best name, because it implies an "Err", but we return
// a `Result<TokenizeResult>`...
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenizeResult<T> {
	Some(T),
	RestartParsing,
	StopParsing,
	None
}

impl<T> TokenizeResult<T> {
	pub fn map<F: FnOnce(T) -> Q, Q>(self, func: F) -> TokenizeResult<Q> {
		match self {
			TokenizeResult::Some(val) => TokenizeResult::Some(func(val)),
			TokenizeResult::RestartParsing => TokenizeResult::RestartParsing,
			TokenizeResult::StopParsing => TokenizeResult::StopParsing,
			TokenizeResult::None => TokenizeResult::None
		}
	}

	pub fn map_none<F: FnOnce() -> T>(self, func: F) -> TokenizeResult<T> {
		if matches!(self, TokenizeResult::None) {
			TokenizeResult::Some(func())
		} else {
			self
		}
	}
}

pub trait Tokenizable {
	type Item;
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Self::Item>>;
}

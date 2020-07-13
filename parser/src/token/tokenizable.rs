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

	#[allow(clippy::use_self)]
impl<T> TokenizeResult<T> {
	pub fn map<F: FnOnce(T) -> Q, Q>(self, func: F) -> TokenizeResult<Q> {
		match self {
			Self::Some(val) => TokenizeResult::Some(func(val)),
			Self::RestartParsing => TokenizeResult::RestartParsing,
			Self::StopParsing => TokenizeResult::StopParsing,
			Self::None => TokenizeResult::None
		}
	}

	pub fn map_none<F: FnOnce() -> T>(self, func: F) -> Self {
		if matches!(self, Self::None) {
			Self::Some(func())
		} else {
			self
		}
	}
}

pub trait Tokenizable {
	type Item;
	fn try_tokenize<S: Stream>(stream: &mut S) -> Result<TokenizeResult<Self::Item>>;
}

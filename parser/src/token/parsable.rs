use crate::Result;
use crate::stream::Stream;


// "ParseResult" is probably not the best name, because it implies an "Err", but we return
// a `Result<ParseResult>`...
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseResult<T> {
	Some(T),
	RestartParsing,
	#[allow(dead_code)] // this is a future improvement, eg `__END__`
	StopParsing,
	None
}

impl<T> ParseResult<T> {
	pub fn map<F: FnOnce(T) -> Q, Q>(self, func: F) -> ParseResult<Q> {
		match self {
			ParseResult::Some(val) => ParseResult::Some(func(val)),
			ParseResult::RestartParsing => ParseResult::RestartParsing,
			ParseResult::StopParsing => ParseResult::StopParsing,
			ParseResult::None => ParseResult::None
		}
	}
}

pub trait Parsable {
	type Item;
	fn try_parse_old<S: std::io::BufRead>(stream: &mut crate::stream::BufStream<S>) -> Result<ParseResult<Self::Item>> {
		Self::try_parse(stream)
	}

	fn try_parse<S: Stream>(stream: &mut S) -> Result<ParseResult<Self::Item>> {
		unimplemented!()
	}
}
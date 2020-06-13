mod context;
mod buf_stream;

use std::io::Seek;
use crate::Result;

pub trait Contexted {
	fn context(&self) -> &Context;
}

pub trait Stream : Seek + Contexted + Iterator<Item=Result<char>> {
	fn peek(&mut self) -> Option<Result<char>>;
	fn starts_with(&mut self, s: &str) -> Result<bool>;
}

pub use self::context::Context;
pub use self::buf_stream::BufStream;
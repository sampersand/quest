mod stream;
mod context;

use std::io::Seek;
use crate::Result;

pub trait Stream : Seek {
	fn context(&self) -> &Context;
	fn next_char(&mut self) -> Result<Option<char>>;
	fn peek_char(&mut self) -> Result<Option<char>>;
}

pub use self::context::Context;
pub use self::stream::BufStream;
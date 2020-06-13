mod context;
mod buf_stream;

use std::io::Seek;
use crate::Result;

pub trait Contexted {
	fn context(&self) -> &Context;
}

pub trait Stream : Seek + Contexted + Iterator<Item=Result<char>> {
	#[deprecated]
	fn next_char(&mut self) -> Result<Option<char>> {
		self.next().transpose()
	}

	fn peek_char(&mut self) -> Result<Option<char>>;
}

pub use self::context::Context;
pub use self::buf_stream::BufStream;
mod context;
mod buf_stream;
mod token_iter;

use std::io::Seek;
use crate::Result;

pub trait Stream : Seek + Contexted + Iterator<Item=Result<char>> {
	fn peek(&mut self) -> Option<Result<char>>;
	fn starts_with(&mut self, s: &str) -> Result<bool>;

	fn tokens(self) -> TokenIter<Self> where Self: Sized {
		TokenIter(self)
	}
}

pub use context::{Context, Contexted};
pub use token_iter::TokenIter;
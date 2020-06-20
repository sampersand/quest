mod context;
mod token_iter;
mod buf_stream;

use std::io::Seek;
use crate::Result;

/// A trait representing something that can be used to parse [`Token`](#)s from.
pub trait Stream : Seek + Contexted + Iterator<Item=Result<char>> {
	fn starts_with(&mut self, s: &str) -> Result<bool>;

	fn tokens(self) -> TokenIter<Self> where Self: Sized {
		TokenIter(self)
	}
}

pub use context::{Context, Contexted};
pub use token_iter::TokenIter;
pub use buf_stream::BufStream;
//! Traits and types for interacting with streams.
mod context;
mod token_iter;
mod buf_stream;

use std::io::{Seek, SeekFrom};
use crate::Result;

/// A trait representing something that can be used to parse [`Token`](#)s from.
///
/// We're guaranteed not to try and `seek` before the beginning of a line; whenever a new line is
/// encountered, it can be considered a completely new stream source.
pub trait Stream : Seek + Contexted + Iterator<Item=Result<char>> {
	/// Checks if the stream starts with the given string.
	///
	/// This is used as a more efficient alternative than checking each byte invidiually, and then
	/// seeking back if it doesn't.
	fn starts_with(&mut self, s: &str) -> Result<bool>;

	/// Gets the next `char` that's not an underscore.
	fn next_non_underscore(&mut self) -> Option<Result<char>> {
		match self.next()? {
			Ok('_') => self.next_non_underscore(),
			other => Some(other)
		}
	}

	/// If the stream starts with `s`, then seek forward that many bytes.
	///
	/// Returns `true` if the stream started with `s` and `false` if it didn't.
	fn next_if_starts_with(&mut self, s: &str) -> Result<bool> {
		if self.starts_with(s)? {
			self.seek(SeekFrom::Current(s.chars().count() as i64))
				.map_err(|err| parse_error!(self, CantReadStream(err)))
				.and(Ok(true))
		} else {
			Ok(false)
		}
	}

	/// Converts this stream into an iterator over tokens.
	fn tokens(self) -> TokenIter<Self> where Self: Sized {
		TokenIter(self)
	}
}

pub use context::{Context, Contexted};
pub use token_iter::TokenIter;
pub use buf_stream::BufStream;
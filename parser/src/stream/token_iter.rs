use crate::{Result, Stream, Token, Context, Contexted};

/// Converts a [`Stream`] into an iterator over tokens.
///
/// This is created by the [`Stream::tokens()`] method.
///
/// This type exists because a type implementing [`Stream`] must already implement the `Iterator`
/// trait for `Result<char>`, and thus can't also implement it for `Result<Token>`.
///
/// [`Stream`]: trait.Stream.html
/// [`Stream::tokens()`]: trait.Stream.html#method.tokens
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TokenIter<S: Stream>(pub(super) S);

impl<S: Stream> Iterator for TokenIter<S> {
	type Item = Result<Token>;

	/// Returns a token parsed by [`Token`](../token/enum.Token.html)
	fn next(&mut self) -> Option<Result<Token>> {
		Token::try_parse(&mut self.0).transpose()
	}
}

impl<S: Stream> Contexted for TokenIter<S> {
	/// Returns the context of the underlying type.
	fn context(&self) -> &Context {
		self.0.context()
	}
}

#[cfg(test)]
mod tests {
	use crate::stream::{BufStream, Stream, Contexted};

	#[test]
	fn next_and_context() {
		let mut iter = BufStream::from("who goes\nthere").tokens();
		assert_eq!(iter.next().unwrap().unwrap().to_string(), "who");
		assert_eq!(iter.context(), iter.0.context());
		assert_eq!(iter.next().unwrap().unwrap().to_string(), "goes");
		assert_eq!(iter.context(), iter.0.context());
		assert_eq!(iter.next().unwrap().unwrap().to_string(), "there");
		assert_eq!(iter.context(), iter.0.context());
	}
}


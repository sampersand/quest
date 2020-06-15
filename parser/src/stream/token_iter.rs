use crate::{Result, Stream, Token, Context, Contexted};

#[derive(Debug)]
pub struct TokenIter<S: Stream>(pub(super) S);

impl<S: Stream> Iterator for TokenIter<S> {
	type Item = Result<Token>;
	fn next(&mut self) -> Option<Result<Token>> {
		Token::try_parse(&mut self.0).transpose()
	}
}

impl<S: Stream> Contexted for TokenIter<S> {
	fn context(&self) -> &Context {
		self.0.context()
	}
}
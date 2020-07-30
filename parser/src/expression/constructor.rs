use super::PutBack;
use crate::stream::{Context, Contexted};
use crate::token::Token;
use crate::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constructor<I>(I, Vec<Token>);

impl<I> Constructor<I> {
	#[inline]
	pub fn new(iter: I) -> Self {
		Constructor(iter, vec![])
	}
}

impl<I: Iterator<Item=Result<Token>>> PutBack for Constructor<I> {
	#[inline]
	fn put_back(&mut self, tkn: Result<Token>) {
		self.1.push(tkn.unwrap());
	}
}

impl<I: Contexted> Contexted for Constructor<I> {
	#[inline]
	fn context(&self) -> &Context {
		self.0.context()
	}
}

impl<I: Iterator<Item=Result<Token>>> Iterator for Constructor<I> {
	type Item = Result<Token>;
	fn next(&mut self) -> Option<Result<Token>> {
		self.1.pop().map(Ok).or_else(|| self.0.next())
	}
}

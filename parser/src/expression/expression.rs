use crate::{Result, Block};
use crate::expression::{Constructable, Constructor};
use crate::stream::{Context, Contexted};
use crate::token::{Token, Literal, Operator, ParenType, operator::Associativity};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::iter::Peekable;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
	Literal(Literal),
	Block(Block),
	Operator(Operator, Vec<Expression>),
}

impl Display for Expression {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Expression::Literal(lit) => Display::fmt(lit, f),
			Expression::Block(block) => Display::fmt(block, f),
			Expression::Operator(op, args) => op.fmt_args(args, f)
		}
	}
}

impl From<Literal> for Expression {
	fn from(lit: Literal) -> Self {
		Expression::Literal(lit)
	}
}

impl From<Block> for Expression {
	fn from(block: Block) -> Self {
		Expression::Block(block)
	}
}


impl Constructable for Expression {
	type Item = Self;

	fn try_construct_primary<C>(ctor: &mut C) -> Result<Option<Expression>>
	where
		C: Iterator<Item=Result<Token>> + super::PutBack + Contexted
	{
		if let Some(expr) = Literal::try_construct_primary(ctor)? {
			Ok(Some(Expression::Literal(expr)))
		} else if let Some(expr) = Operator::try_construct_primary(ctor)? {
			Ok(Some(expr))
		} else if let Some(expr) = Block::try_construct_primary(ctor)? {
			Ok(Some(Expression::Block(expr)))
		// } else if let Some(tkn) = ctor.next().transpose()? {
		// 	ctor.put_back(Ok(tkn));
		// 	Err(parse_error!(ctor, Message("no primary could be found")))
		} else {
			Ok(None)
		}
	}
}

impl Expression {
	pub fn try_construct<C>(ctor: &mut C) -> Result<Expression>
	where
		C: Iterator<Item=Result<Token>> + super::PutBack + Contexted
	{
		Expression::try_construct_precedence(ctor, None)?
			.ok_or_else(|| parse_error!(ctor, ExpectedExpression))
	}

	pub fn try_construct_precedence<C>(ctor: &mut C, op: Option<Operator>) -> Result<Option<Expression>>
	where
		C: Iterator<Item=Result<Token>> + super::PutBack + Contexted
	{
		if let Some(primary) = Expression::try_construct_primary(ctor)? {
			Operator::construct_operator(ctor, primary, op).map(Some)
		} else {
			Ok(None)
		}
	}
}

impl Expression {
	pub fn parse_stream<I>(iter: I) -> Result<Self>
	where
		I: Iterator<Item=Result<Token>> + Contexted
	{

		#[derive(PartialEq, Debug)]
		enum Where { Start, GivenCode, End }

		#[derive(Debug)]
		struct WrappedBlock<I>(Where, Constructor<I>);

		impl<I: Iterator<Item=Result<Token>>> super::PutBack for WrappedBlock<I> {
			fn put_back(&mut self, item: Self::Item) {
				if self.0 == Where::End {
					self.0 = Where::GivenCode
				}

				self.1.put_back(item);
			}
		}

		impl<I: Iterator<Item=Result<Token>>> Iterator for WrappedBlock<I> {
			type Item = Result<Token>;
			fn next(&mut self) -> Option<Self::Item> {
				match self.0 {
					Where::Start => {
						self.0 = Where::GivenCode;
						Some(Ok(Token::Left(ParenType::Curly)))
					},
					Where::GivenCode => self.1.next().or_else(|| {
						self.0 = Where::End;
						Some(Ok(Token::Right(ParenType::Curly)))
					}),
					Where::End => None,
				}
			}
		}

		impl<I: Contexted> Contexted for WrappedBlock<I> {
			fn context(&self) -> &Context {
				self.1.context()
			}
		}

		Expression::try_construct(&mut WrappedBlock(Where::Start, Constructor::new(iter)))
	}
}

	

use crate::{Result, Block};
use crate::expression::{Constructable, Constructor, Executable, BoundOperator};
use crate::stream::{Context, Contexted};
use crate::token::{Token, Primative, Operator, Parenthesis};
use std::fmt::{self, Display, Formatter};

/// An expression is the fundamental unit of quest code.
///
/// Whenever a piece of code is run, what ends up happening under the covers is the [`Expression`]
/// is executed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
	/// Represents a primative Quest value---that is, it's not a block of code or an operator.
	Primative(Primative),
	/// Represents an unexecuted block of code.
	Block(Block),
	/// An operator and its associated operands.
	Operator(BoundOperator),
}

impl Display for Expression {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Expression::Primative(prim) => Display::fmt(prim, f),
			Expression::Block(block) => Display::fmt(block, f),
			Expression::Operator(op) => Display::fmt(op, f),
		}
	}
}

impl Executable for Expression {
	#[inline]
	fn execute(&self) -> quest_core::Result<quest_core::Object> {
		match self {
			Expression::Primative(prim) => prim.execute(),
			Expression::Block(block) => block.execute(),
			Expression::Operator(op) => op.execute(),
		}
	}
}

impl From<Primative> for Expression {
	fn from(prim: Primative) -> Self {
		Expression::Primative(prim)
	}
}

impl From<Block> for Expression {
	fn from(block: Block) -> Self {
		Expression::Block(block)
	}
}

impl From<BoundOperator> for Expression {
	fn from(oper: BoundOperator) -> Self {
		Expression::Operator(oper)
	}
}


impl Constructable for Expression {
	fn try_construct_primary<C>(ctor: &mut C) -> Result<Option<Self>>
	where
		C: Iterator<Item=Result<Token>> + super::PutBack + Contexted
	{
		if let Some(prim) = Primative::try_construct_primary(ctor)? {
			Ok(Some(prim.into()))
		} else if let Some(oper) = BoundOperator::try_construct_primary(ctor)? {
			Ok(Some(oper.into()))
		} else if let Some(block) = Block::try_construct_primary(ctor)? {
			Ok(Some(block.into()))
		// } else if let Some(tkn) = ctor.next().transpose()? {
		// 	ctor.put_back(Ok(tkn));
		// 	Err(parse_error!(ctor, Messaged("no primary could be found".to_string())))
		} else {
			Ok(None)
		}
	}
}

impl Expression {
	/// Try to create an expression from the given constructor.
	pub fn try_construct<C>(ctor: &mut C) -> Result<Expression>
	where
		C: Iterator<Item=Result<Token>> + super::PutBack + Contexted
	{
		Expression::try_construct_precedence(ctor, None)?
			.ok_or_else(|| parse_error!(ctor,
				CantCreateExpression(super::Error::ExpectedExpression.into())))
	}

	/// Try to construct an expression from the given constructor, gobbling up all operators until
	/// one that has a looser binding is encountered.
	pub fn try_construct_precedence<C>(ctor: &mut C, op: Option<Operator>)
		-> Result<Option<Expression>>
	where
		C: Iterator<Item=Result<Token>> + super::PutBack + Contexted
	{
		if let Some(primary) = Expression::try_construct_primary(ctor)? {
			BoundOperator::construct_operator(ctor, primary, op).map(Some)
		} else {
			Ok(None)
		}
	}
}

impl Expression {
	/// Try to parse a stream into an expression.
	///
	/// This is effectively identical to calling [`try_construct`], except `(` and `)` are added to
	/// the beginning and the end of the stream, respectively.
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
						Some(Ok(Token::Left(Parenthesis::Round)))
					},
					Where::GivenCode => self.1.next().or_else(|| {
						self.0 = Where::End;
						Some(Ok(Token::Right(Parenthesis::Round)))
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

	

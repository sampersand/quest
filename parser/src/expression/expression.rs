use crate::{Result, Block};
use crate::expression::{Constructable, Constructor, Executable, BoundOperator};
use crate::stream::{Context, Contexted};
use crate::token::{Token, Primitive, Operator, ParenType};
use quest_core::{Object, Args};
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone, PartialEq, Eq)]
pub enum Expression {
	Primitive(Primitive),
	Block(Block),
	Operator(BoundOperator),
	FunctionCall(Box<Self>, Block)
}

impl Debug for Expression {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::Primitive(prim) => Debug::fmt(prim, f),
			Self::Block(block) => Debug::fmt(block, f),
			Self::Operator(oper) => Debug::fmt(oper, f),
			Self::FunctionCall(call, args) 
				=> f.debug_tuple("FunctionCall").field(call).field(args).finish()
		}
	}
}

impl Display for Expression {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::Primitive(prim) => Display::fmt(prim, f),
			Self::Block(block) => Display::fmt(block, f),
			Self::Operator(op) => Display::fmt(op, f),
			Self::FunctionCall(this, block) =>
				Display::fmt(this, f).and_then(|_| Display::fmt(block, f))
		}
	}
}

fn call_function(this: &Expression, block: &Block) -> quest_core::Result<Object> {
	use crate::block::LineResult;
	let this = this.execute()?;

	match block.run_block()? {
		Some(LineResult::Single(s)) => this.call_attr_lit("()", &[&s]),
		Some(LineResult::Multiple(m)) => this.call_attr_lit("()", m.iter().collect::<Args>()),
		None => this.call_attr_lit("()", &[])
	}
}

impl Executable for Expression {
	fn execute(&self) -> quest_core::Result<Object> {
		match self {
			Self::Primitive(prim) => prim.execute(),
			Self::Block(block) => block.execute(),
			Self::Operator(op) => op.execute(),
			Self::FunctionCall(this, block) => call_function(&this, &block)
		}
	}
}

impl From<Primitive> for Expression {
	#[inline]
	fn from(prim: Primitive) -> Self {
		Self::Primitive(prim)
	}
}

impl From<Block> for Expression {
	#[inline]
	fn from(block: Block) -> Self {
		Self::Block(block)
	}
}

impl From<BoundOperator> for Expression {
	#[inline]
	fn from(oper: BoundOperator) -> Self {
		Self::Operator(oper)
	}
}


impl Constructable for Expression {
	type Item = Self;

	fn try_construct_primary<C>(ctor: &mut C) -> Result<Option<Self>>
	where
		C: Iterator<Item=Result<Token>> + super::PutBack + Contexted
	{
		if let Some(prim) = Primitive::try_construct_primary(ctor)? {
			Ok(Some(prim.into()))
		} else if let Some(oper) = BoundOperator::try_construct_primary(ctor)? {
			Ok(Some(oper.into()))
		} else if let Some(block) = Block::try_construct_primary(ctor)? {
			Ok(Some(block.into()))
		// } else if let Some(tkn) = ctor.next().transpose()? {
		// 	ctor.put_back(Ok(tkn));
		// 	Err(parse_error!(ctor, Message("no primary could be found")))
		} else {
			Ok(None)
		}
	}
}

impl Expression {
	pub fn try_construct<C>(ctor: &mut C) -> Result<Self>
	where
		C: Iterator<Item=Result<Token>> + super::PutBack + Contexted
	{
		Self::try_construct_precedence(ctor, None)?
			.ok_or_else(|| parse_error!(ctor, ExpectedExpression))
	}

	pub fn try_construct_precedence<C>(ctor: &mut C, op: Option<Operator>) -> Result<Option<Self>>
	where
		C: Iterator<Item=Result<Token>> + super::PutBack + Contexted
	{
		if let Some(primary) = Self::try_construct_primary(ctor)? {
			BoundOperator::construct_operator(ctor, primary, op).map(Some)
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
						Some(Ok(Token::Left(ParenType::Round)))
					},
					Where::GivenCode => self.1.next().or_else(|| {
						self.0 = Where::End;
						Some(Ok(Token::Right(ParenType::Round)))
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

		let x = Self::try_construct(&mut WrappedBlock(Where::Start, Constructor::new(iter)))?;
		println!("{:#}", x);
		// println!("{:#?}", x);
		Ok(x)
	}
}

	

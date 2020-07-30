use crate::token::{Token, Operator, operator::Associativity};
use crate::expression::{Expression, Constructable, PutBack, Executable};
use crate::stream::Contexted;
use crate::Result;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
enum OperArgs {
	Unary,
	Binary(Expression),
	Ternary(Expression, Expression)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundOperator {
	oper: Operator,
	this: Box<Expression>,
	args: Box<OperArgs>
}

impl Display for BoundOperator {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match &*self.args {
			OperArgs::Unary if self.oper.assoc() != Associativity::UnaryOperOnLeft
				=> todo!("non-UnaryOperOnLeft unary operators"),

			OperArgs::Unary => {
				Display::fmt(&self.oper, f)?;
				Display::fmt(&self.this, f)
			},

			OperArgs::Binary(rhs) if self.oper == Operator::Call => {
				Display::fmt(&self.this, f)?;
				Display::fmt(rhs, f)
			},
			OperArgs::Binary(rhs) => {
				Display::fmt(&self.this, f)?;
				if self.oper > Operator::Pow { Display::fmt(&' ', f)?; }
				Display::fmt(&self.oper, f)?;
				if self.oper > Operator::Pow { Display::fmt(&' ', f)?; }
				Display::fmt(rhs, f)
			},

			OperArgs::Ternary(mid, rhs) if self.oper == Operator::DotAssign => {
				Display::fmt(&self.this, f)?;
				Display::fmt(&Operator::Dot, f)?;
				Display::fmt(mid, f)?;
				Display::fmt(&' ', f)?;
				Display::fmt(&Operator::Assign, f)?;
				Display::fmt(&' ', f)?;
				Display::fmt(rhs, f)
			},
			OperArgs::Ternary(_mid, _rhs) => todo!("non-DotAssign ternary operators")
		}
	}
}

impl Executable for BoundOperator {
	fn execute(&self) -> quest_core::Result<quest_core::Object> {
		let this = self.this.execute()?;
		let oper = self.oper.repr();

		match self.args.as_ref() {
			OperArgs::Unary => this.call_attr_lit(oper, &[]),
			OperArgs::Binary(r) => this.call_attr_lit(oper, &[&r.execute()?]),
			OperArgs::Ternary(m, r) => this.call_attr_lit(oper, &[&m.execute()?, &r.execute()?])
		}
	}
}

impl Constructable for BoundOperator {
	type Item = Self;

	fn try_construct_primary<C>(ctor: &mut C) -> Result<Option<Self>>
	where
		C: Iterator<Item=Result<Token>> + PutBack + Contexted
	{
		// helper function to ensure all unary ops are made the same way
		fn make_unary<C>(oper: Operator, ctor: &mut C) -> Result<Option<BoundOperator>>
		where
			C: Iterator<Item=Result<Token>> + PutBack + Contexted
		{
			Ok(Some(BoundOperator {
				oper,
				this: Box::new(Expression::try_construct(ctor)?),
				args: Box::new(OperArgs::Unary)
			}))
		}
		// only unary operators on the left are constructable as they're 
		match ctor.next().transpose()? {
			Some(Token::Operator(oper)) if oper.assoc() == Associativity::UnaryOperOnLeft
				=> make_unary(oper, ctor),
			// allow for unary `+` and `-`
			Some(Token::Operator(Operator::Scoped)) => make_unary(Operator::RootScope, ctor),
			Some(Token::Operator(Operator::Add)) => make_unary(Operator::Pos, ctor),
			Some(Token::Operator(Operator::Sub)) => make_unary(Operator::Neg, ctor),
			Some(tkn) => { ctor.put_back(Ok(tkn)); Ok(None) }
			None => Ok(None),
		}
	}
}


fn build_op<C>(oper: Operator, ctor: &mut C, mut this: Expression) -> Result<Expression>
where
	C: Iterator<Item=Result<Token>> + PutBack + Contexted
{
	let rhs = Expression::try_construct_precedence(ctor, Some(oper))?
		.ok_or_else(|| parse_error!(ctor, ExpectedExpression))?;

	this = BoundOperator {
		oper,
		this: Box::new(this),
		args: Box::new(OperArgs::Binary(rhs))
	}.into();

	match ctor.next().transpose()? {
		Some(Token::Operator(Operator::Assign)) if oper == Operator::Dot => 
			this = build_op(Operator::DotAssign, ctor, this)?,
		Some(Token::Operator(next_op)) => this = build_op(next_op, ctor, this)?,
		Some(tkn) => ctor.put_back(Ok(tkn)),
		None => {}
	}


	if oper == Operator::DotAssign {
		// a hack to convert the lhs and rhs into ternary values.
		this = match this {
			Expression::Operator(BoundOperator { args, this, .. }) => match *args {
				OperArgs::Binary(rhs) => match *this {
					Expression::Operator(BoundOperator { args, this, .. }) => match *args {
						OperArgs::Binary(mid) => BoundOperator {
							oper,
							this,
							args: Box::new(OperArgs::Ternary(mid, rhs))
						}.into(),
						_ => unreachable!("bad args2: {:?}", args)
					},
					_ => unreachable!("bad rhs: {:?}", rhs)
				},
				_ => unreachable!("bad args1: {:?}", args)
			},
			_ => unreachable!("bad this: {:?}", this)
		};
	} else if oper == Operator::Call {
		// a hack to convert to function call.
		this = match this {
			Expression::Operator(BoundOperator { args, this, oper }) =>
				match *args {
					OperArgs::Binary(rhs) =>
						match rhs {
							Expression::Block(block) => Expression::FunctionCall(this, block),
							_ =>
								Expression::Operator(BoundOperator { this, oper,
									args: Box::new(OperArgs::Binary(rhs)) })
						},
					args => Expression::Operator(BoundOperator { this, oper, args: Box::new(args) })
				},
			other => other
		};

		debug_assert!(matches!(this, Expression::FunctionCall(..)), "{:#?}", this);
	}

	Ok(this)
}

impl BoundOperator {
	pub fn construct_operator<C>(ctor: &mut C, lhs: Expression, parent_op: Option<Operator>)
		-> Result<Expression>
	where
		C: Iterator<Item=Result<Token>> + PutBack + Contexted
	{

		match ctor.next().transpose()? {
			Some(Token::Operator(oper)) if parent_op.map(|parent_op| oper < parent_op).unwrap_or(true)
				=> build_op(oper, ctor, lhs),
			Some(t @ Token::Operator(_))
				| Some(t @ Token::Endline)
				| Some(t @ Token::Comma)
				| Some(t @ Token::Right(_)) => { ctor.put_back(Ok(t)); Ok(lhs) },
			Some(tkn) => {
				ctor.put_back(Ok(tkn));

				// any other token indicates that we're being called
				if parent_op.map(|parent_op| Operator::Call < parent_op).unwrap_or(true) {
					build_op(Operator::Call, ctor, lhs)
					// match build_op(Operator::Call, ctor, lhs)? {
					// 	Expression::Operator(BoundOperator { args, this, .. }) =>
					// 		match *args {
					// 			OperArgs::Binary(rhs) =>
					// 				match rhs {
					// 					Expression::Block(block) => Ok(Expression::FunctionCall(this, block)),
					// 					_ => unreachable!()
					// 				},
					// 			_ => unreachable!()
					// 		},
					// 	_ => unreachable!()
					// }
				} else {
					ctor.put_back(Ok(Token::Operator(Operator::Call)));
					Ok(lhs)
				}
			},
			None => Ok(lhs),
		}
	}
}

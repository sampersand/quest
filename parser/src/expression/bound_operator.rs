use crate::token::{Token, Operator, operator::Associativity, ParenType};
use crate::expression::{Expression, Constructable, PutBack, Executable};
use crate::stream::Contexted;
use crate::Result;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum OperArgs {
	Unary,
	Binary(Expression),
	Ternary(Expression, Expression)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundOperator {
	pub(crate) oper: Operator,
	pub(crate) this: Box<Expression>,
	pub(crate) args: Box<OperArgs>
}

impl Display for BoundOperator {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match &*self.args {
			OperArgs::Unary if self.oper.assoc() == Associativity::UnaryOperOnLeft =>
				write!(f, "{}{}", self.oper, self.this),
			OperArgs::Unary => write!(f, "{}{}", self.this, self.oper),
			OperArgs::Binary(rhs) if self.oper <= Operator::Dot => 
				write!(f, "{}{}{}", self.this, self.oper, rhs),
			OperArgs::Binary(rhs) if self.oper == Operator::Call => 
				write!(f, "{} {}", self.this, rhs),
			OperArgs::Binary(rhs) if self.oper < Operator::Assign => 
				write!(f, "({}) {} ({})", self.this, self.oper, rhs),
			OperArgs::Binary(rhs) =>
				write!(f, "{} {} {}", self.this, self.oper, rhs),
			OperArgs::Ternary(mid, rhs) if self.oper == Operator::DotAssign =>
				write!(f, "{}.{} = {}", self.this, mid, rhs),
			OperArgs::Ternary(mid, rhs) if self.oper == Operator::IndexAssign =>
				write!(f, "{}[{}] = {}", self.this, mid, rhs),
			OperArgs::Ternary(mid, rhs) =>
				write!(f, "{}{}({}, {})", self.this, self.oper, mid, rhs)
		}
	}
}

impl BoundOperator {
}
impl Executable for BoundOperator {

	fn execute(&self) -> quest_core::Result<quest_core::Object> {
		let this = self.this.execute()?;

		match self.args.as_ref() {
			OperArgs::Binary(rhs) if self.oper == Operator::Call => match rhs {
				Expression::Block(block) if block.paren_type() == ParenType::Round =>
					return match block.run_block()? {
						Some(crate::block::LineResult::Single(s)) =>
							this.call_attr_lit(self.oper.into(), &[&s]),
						Some(crate::block::LineResult::Multiple(m)) =>
							this.call_attr_lit(self.oper.into(), m.iter().collect::<Vec<&_>>()),
						None =>
							this.call_attr_lit(self.oper.into(), &[])
					},
				_ => {}
			},
			_ => {}
		};

		let oper = self.oper.repr();

		match self.args.as_ref() {
			OperArgs::Unary => this.call_attr_lit(oper, &[]),
			OperArgs::Binary(rhs) => this.call_attr_lit(oper, &[&rhs.execute()?]),
			OperArgs::Ternary(mid, rhs) =>
				this.call_attr_lit(oper, &[&mid.execute()?, &rhs.execute()?])
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
			Some(Token::Operator(Operator::Mul)) => make_unary(Operator::Splat, ctor),
			Some(Token::Operator(Operator::Pow)) => make_unary(Operator::SplatSplat, ctor),

			Some(tkn) => { ctor.put_back(Ok(tkn)); Ok(None) }
			None => Ok(None),
		}
	}
}


fn build_op<C>(oper: Operator, ctor: &mut C, mut this: Expression) -> Result<Expression>
where
	C: Iterator<Item=Result<Token>> + PutBack + Contexted
{
	use crate::token::Primitive;

	let rhs = Expression::try_construct_precedence(ctor, Some(oper))?
		.ok_or_else(|| parse_error!(ctor, ExpectedExpression))?;

	this = BoundOperator {
		oper,
		this: Box::new(this),
		args: Box::new(OperArgs::Binary(rhs))
	}.into();

	// A hack to convert a raw identifier into a piece of text.
	this =
		match this {
			Expression::Operator(BoundOperator { this, args, oper: Operator::Assign }) |
				Expression::Operator(BoundOperator { this, args, oper: Operator::Colon }) |
				Expression::Operator(BoundOperator { this, args, oper: Operator::Arrow })
			=>
				Expression::Operator(BoundOperator { args, oper, this: 
					match *this {
						Expression::Primitive(Primitive::Variable(var)) =>
							Expression::Primitive(Primitive::Text(var.into())).into(),
						Expression::Block(block) => Expression::Block(block.convert_to_parameters()).into(),
						other => other.into()
					}}),
			Expression::Operator(BoundOperator { this, args, oper: Operator::Dot }) |
				Expression::Operator(BoundOperator { this, args, oper: Operator::DotQuestion }) |
				Expression::Operator(BoundOperator { this, args, oper: Operator::Scoped })
			=>
				Expression::Operator(BoundOperator { this, oper, args: 
					match *args {
						OperArgs::Binary(Expression::Primitive(Primitive::Variable(var))) =>
							OperArgs::Binary(Expression::Primitive(Primitive::Text(var.into()))).into(),
						other => other.into()
					}}),
			other => other
		};

	match ctor.next().transpose()? {
		Some(Token::Operator(Operator::Assign)) if oper == Operator::Dot => 
			this = build_op(Operator::DotAssign, ctor, this)?,
		Some(Token::Operator(Operator::Assign)) if oper == Operator::Call => 
			this = build_op(Operator::IndexAssign, ctor, this)?,
		Some(Token::Operator(next_op)) => this = build_op(next_op, ctor, this)?,
		Some(tkn) => ctor.put_back(Ok(tkn)),
		None => {}
	}

	if oper == Operator::DotAssign || oper == Operator::IndexAssign {
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
	}

	if oper == Operator::DeferedEndl {
		this = match this {
			Expression::Operator(BoundOperator { args, this, oper }) =>
				match *args {
					OperArgs::Binary(rhs) => Expression::Operator(BoundOperator { this, oper, args:
						Box::new(OperArgs::Binary(Expression::Block(crate::Block {
							lines: vec![crate::block::Line::Single(rhs)],
							paren_type: ParenType::Curly,
							context: Default::default()
						})))
					}),
					args => panic!("{:?}", args)//Expression::Operator(BoundOperator { args: Box::new(args), this, oper })
				},
			other => panic!("{:?}", other)
		};
	}

	if oper == Operator::Call {
		// a hack to convert to function call.

		this = match this {
			Expression::Operator(BoundOperator { args, this, oper }) =>
				match *args {
					OperArgs::Binary(Expression::Block(block)) => Expression::FunctionCall(this, block),
					args => Expression::Operator(BoundOperator { this, oper, args: Box::new(args) }),
				},
			other => other
		};

		this = match this {
			Expression::FunctionCall(lhs, block) if block.paren_type() == ParenType::Curly => match *lhs {
				Expression::Operator(BoundOperator { oper: Operator::Call, this, mut args }) => {
					if let OperArgs::Binary(Expression::Block(ref mut bn)) = &mut *args {
						let block = Expression::Block(block);
						if let Some(last) = bn.lines.last_mut() {
							match last {
								crate::block::Line::Single(expr) => *last = crate::block::Line::Multiple(vec![expr.clone(), block]),
								crate::block::Line::Multiple(vec) => vec.push(block)
							}
						} else {
							bn.lines.push(crate::block::Line::Single(block));
						}
					}
					Expression::Operator(BoundOperator { oper: Operator::Call, this, args })

				},
				lhs @ Expression::Operator(BoundOperator { .. }) =>
					Expression::FunctionCall(Box::new(lhs), crate::block::Block { 
						context: block.context.clone(),
						paren_type: ParenType::Round,
						lines: vec![crate::block::Line::Single(Expression::Block(block))],
					}),
				lhs => Expression::FunctionCall(Box::new(lhs), block)
			},
			other => other
		};
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
			// This endline thing is a failed experiment...
			// Some(Token::Endline(false)) if parent_op.is_some() && parent_op != Some(Operator::Call) => return Self::construct_operator(ctor, lhs, parent_op),
			Some(Token::Operator(oper)) if parent_op
				.map(|parent_op| oper < parent_op || 
						oper <= parent_op && oper.assoc() == crate::token::operator::Associativity::RightToLeft
				).unwrap_or(true)
				=> build_op(oper, ctor, lhs),
			Some(t @ Token::Operator(_))
				| Some(t @ Token::Endline(_))
				| Some(t @ Token::Comma)
				| Some(t @ Token::Right(_)) => { ctor.put_back(Ok(t)); Ok(lhs) },
			Some(tkn) => {
				ctor.put_back(Ok(tkn));

				// any other token indicates that we're being called
				if parent_op.map(|parent_op| Operator::Call < parent_op).unwrap_or(true) {
					build_op(Operator::Call, ctor, lhs)
				} else {
					ctor.put_back(Ok(Token::Operator(Operator::Call)));
					Ok(lhs)
				}
			},
			None => Ok(lhs),
		}
	}
}

use crate::token::{Token, Operator, operator::Associativity, ParenType};
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

impl BoundOperator {
	
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
			OperArgs::Ternary(mid, rhs) =>
				write!(f, "{}{}({}, {})", self.this, self.oper, mid, rhs)
		}
	}
}

impl Executable for BoundOperator {

	fn execute(&self) -> quest_core::Result<quest_core::Object> {
		let this = self.this.execute()?;

		match self.args.as_ref() {
			OperArgs::Binary(rhs) if self.oper == Operator::Call => match rhs {
				Expression::Block(block) if block.paren_type() == ParenType::Round =>
					return match block.run_block()? {
						Some(crate::block::LineResult::Single(s)) =>
							this.call_attr(&self.oper, &[&s]),
						Some(crate::block::LineResult::Multiple(m)) =>
							this.call_attr(&self.oper, m.iter().collect::<Vec<&_>>()),
						None =>
							this.call_attr(&self.oper, &[])
					},
				_ => {}
			},
			_ => {}
		};

		let args_vec: Vec<quest_core::Object> = match self.args.as_ref() {
			OperArgs::Unary => vec![],
			OperArgs::Binary(rhs) => vec![rhs.execute()?],
			OperArgs::Ternary(mid, rhs) => vec![mid.execute()?, rhs.execute()?],
		};

		let args_vec: Vec<&quest_core::Object> = args_vec.iter().collect();

		this.call_attr(&self.oper, args_vec)
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
				} else {
					ctor.put_back(Ok(Token::Operator(Operator::Call)));
					Ok(lhs)
				}
			},
			None => Ok(lhs),
		}
	}
	}

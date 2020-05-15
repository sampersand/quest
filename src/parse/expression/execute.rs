use crate::parse::{Expression, Token, Literal, token::{ParenType, Operator}, expression::{Block, Line}};
use crate::obj::{self, Object, Result, Args};

impl Block {
	fn execute(self, binding: &Object) -> Result<Option<Object>> {
		let Block { paren, mut body, returns} = self;
		let ret = match paren {
			ParenType::Paren => 
				if let Some(last) = body.pop() {
					for line in body {
						line.execute(binding)?;
					}
					last.execute(binding)?
				} else {
					Object::default()
				},
			ParenType::Brace => todo!("ParenType::Brace object type."),
			ParenType::Bracket => todo!("ParenType::Bracket return value."),
		};

		if returns {
			Ok(Some(ret))
		} else {
			Ok(None)
		}
	}
}

impl Line {
	fn execute(self, binding: &Object) -> Result<Object> {
		match self {
			Line::Singular(line) => line.execute(binding),
			Line::Multiple(args) => args.into_iter()
				.map(|arg| arg.execute(binding))
				.collect::<Result<Vec<_>>>()
				.map(|args| obj::types::List::from(args).into())
		}
	}
}

impl Operator {
	fn execute(self, obj: Box<Expression>, args: Vec<Box<Expression>>, binding: &Object) -> Result<Object> {
		obj.execute(binding)?
			.call_attr(&self.into(), Args::new(
				args.into_iter().map(|arg| arg.execute(binding)).collect::<Result<Vec<_>>>()?,
				binding.clone()
			))
	}
}

impl Expression {
	pub fn execute_default(self) -> Result<Object> {
		self.execute(&Object::new(obj::types::Kernel))
	}
	pub fn execute(self, binding: &Object) -> Result<Object> {
		match self {
			Expression::Literal(Literal::Number(num)) => Ok(num.into()),
			Expression::Literal(Literal::Text(text)) => Ok(text.into()),
			Expression::Literal(Literal::Variable(var)) => 
				Object::new(var).call("()", Args::new(vec![], binding.clone())),
			Expression::Block(block) => block.execute(binding).map(Option::unwrap_or_default),
			Expression::PrefixOp(op, obj) => op.execute(obj, vec![], binding),
			Expression::InfixOp(op, obj, arg) => op.execute(obj, vec![arg], binding),
			Expression::TerninaryOp(op, obj, arg1, arg2) => op.execute(obj, vec![arg1, arg2], binding),
			Expression::FunctionCall(obj, block) => {
				obj.execute(binding)?.call_attr(
					&block.paren.into(),
					Args::new(
						if let Some(obj) = block.execute(binding)? {
							if let Some(list) = obj.downcast_ref::<obj::types::List>() {
								Vec::<Object>::from(list.clone())
							} else {
								vec![obj.clone()]
							}
						} else {
							vec![]
						},
						binding.clone()
					)
				)
			}
			// other => unimplemented!("{:?}", other)
		}
// 		execute
// #[derive(Debug)]
// pub enum Expression {
// 	Literal(Literal),
// 	Block(Block),
// 	FunctionCall(Box<Expression>, Block),
// 	PrefixOp(Operator, Box<Expression>),
// 	InfixOp(Operator, Box<Expression>, Box<Expression>),
// 	TerninaryOp(Operator, Box<Expression>, Box<Expression>, Box<Expression>),
// }

	}
}
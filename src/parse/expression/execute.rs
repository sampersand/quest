use crate::parse::{Expression, Token, Literal, token::{ParenType, Operator}, expression::{Block, Line}};
use crate::obj::{self, Object, Result, Args, types::rustfn::Binding};

impl Operator {
	fn execute(self, obj: Box<Expression>, args: Vec<Box<Expression>>, binding: &Binding) -> Result<Object> {
		obj.execute(binding)?
			.call_attr(&self.into(), Args::new(
				args.into_iter().map(|arg| arg.execute(binding)).collect::<Result<Vec<_>>>()?,
				binding.clone()
			))
	}
}

impl Expression {
	pub fn execute_default(&self) -> Result<Object> {
		self.execute(&Object::new(obj::types::Kernel))
	}
	pub fn execute(&self, binding: &Binding) -> Result<Object> {
		match self {
			Expression::Literal(Literal::Number(num)) => Ok(num.clone().into()),
			Expression::Literal(Literal::Text(text)) => Ok(text.clone().into()),
			Expression::Literal(Literal::Variable(var)) => 
				Object::new(var.clone()).call("()", Args::new(vec![], binding.clone())),
			Expression::Block(block) => block.execute(binding).map(Option::unwrap_or_default),
			Expression::PrefixOp(op, obj) => op.execute(obj.clone(), vec![], binding),
			Expression::InfixOp(op, obj, arg) => op.execute(obj.clone(), vec![arg.clone()], binding),
			Expression::TerninaryOp(op, obj, arg1, arg2) => op.execute(obj.clone(), vec![arg1.clone(), arg2.clone()], binding),
			Expression::FunctionCall(obj, block) => {
				obj.execute(binding)?.call_attr(
					&block.paren().into(),
					/* this is janky */
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
		}
	}
}
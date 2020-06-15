use crate::{Expression, Token, Literal, token::{ParenType, Operator}, expression::{Block, Line}};
use quest::{Object, Result, Args};

impl Operator {
	#[allow(clippy::vec_box, clippy::boxed_local)]
	fn execute(self, obj: Box<Expression>, args: Vec<Box<Expression>>) -> Result<Object> {
		obj.execute()?.call_attr(&self, Args::new(
			args.into_iter().map(|arg| arg.execute()).collect::<Result<Vec<_>>>()?
		))
	}
}

impl Expression {
	pub fn execute(&self) -> Result<Object> {
		match self {
			Expression::Literal(Literal::Number(num)) => Ok(num.clone().into()),
			Expression::Literal(Literal::Text(text)) => Ok(text.clone().into()),
			Expression::Literal(Literal::Variable(var)) => 
				Object::new(var.clone()).call_attr("()", Args::default()),
			Expression::Block(block) => block.execute().map(Option::unwrap_or_default),
			Expression::PrefixOp(op, obj) => op.execute(obj.clone(), vec![]),
			Expression::InfixOp(op, obj, arg) => op.execute(obj.clone(), vec![arg.clone()]),
			Expression::TerninaryOp(op, obj, arg1, arg2) =>
				op.execute(obj.clone(), vec![arg1.clone(), arg2.clone()]),
			Expression::FunctionCall(obj, block) => {
				obj.execute()?.call_attr(
					&Object::from(block.paren()),
					/* this is janky */
					Args::new(
						if let Some(obj) = block.execute()? {
							if let Some(list) = obj.downcast_ref::<quest::types::List>() {
								Vec::<Object>::from(list.clone())
							} else {
								vec![obj.clone()]
							}
						} else {
							vec![]
						}
					)
				)
			}
		}
	}
}
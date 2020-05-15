use crate::parse::{Expression, Token, Literal};
use crate::obj::{self, Object, Result, Args};

impl Expression {
	pub fn execute_default(self) -> Result<Object> {
		self.execute(&Object::new(obj::types::Kernel))
	}
	pub fn execute(self, binding: &Object) -> Result<Object> {
		match self {
			Expression::Literal(Literal::Number(num)) => Ok(num.into()),
			Expression::Literal(Literal::Text(text)) => Ok(text.into()),
			Expression::Literal(Literal::Variable(var)) => 
				Object::new(var).call("()", Args::new(binding, vec![])),

			_ => unimplemented!()
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
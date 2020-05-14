use crate::parse::{Result, Error, Token, token::{Literal, Operator, ParenType}};
/*
E -> <literal>   // literals are expressions
E -> { E }       // gets a block
E -> ( E )       // parenthetical aside
E -> E ( E )     // calling a function
E -> E <op> E 	  // infix operator
E -> <op> E      // prefix operator,
E -> E <op>      // postfix operator, incl. `;`
*/

#[derive(Debug)]
pub enum Expression {
	// E -> <literal>
	Literal(Literal),
	// E -> { E }
	Block(Box<Expression>),
	// E -> ( E )
	Grouping(Box<Expression>),
	// E -> E ( E )
	FunctionCall(Box<Expression>, Box<Expression>),
	// E -> E <infix> E 
	InfixOperator(Box<Expression>, Operator, Box<Expression>),
	// E -> <prefix> E 
	PrefixOperator(Operator, Box<Expression>),
	// E -> E <postfix>
	PostfixOperator(Box<Expression>, Operator),
}

#[derive(Debug)]
struct TokenIteratorReversable<'a>(&'a [Token], usize);

impl<'a> Iterator for TokenIteratorReversable<'a> {
	type Item = &'a Token;
	fn next(&mut self) -> Option<Self::Item> {
		self.1 += 1;
		self.0.get(self.1 - 1)
	}
}

enum SubExpression {
	Literal(Literal),
	Block(Box<Expression>),
	Grouping(Box<Expression>),
	Right(ParenType),
	Op(Operator)
}


// fn next_subexpr<I: Iterator<Item=Token>>(iter: I) -> Result<Option<SubExpression>> {
// 	Ok(match iter.next().transpose()? {
// 		None => None,
// 		Some(Token::Literal(lit)) => Some(SubExpression::Literal(lit)),
// 		Some(Token::Operator(op)) => Some(SubExpression::Op(operator))
// 		// pub enum Token {
// 		// 	Literal(Literal),
// 		// 	Operator(Operator),
// 		// 	Left(ParenType),
// 		// 	Right(ParenType),
// 		// }
// 	}
// }

impl Expression {
		// unimplemented!()
	// 	Some(match v.get(0)? {
	// 		Token::Literal(literal) => Expression::Literal(literal),
	// 		Token::Left(ParenType::Brace) => todo!(),
	// 		Token::Left(ParenType::Brace) => todo!(),
	// 	})
	// }
	pub fn try_from_iter<I: Iterator<Item=Result<Token>>>(mut iter: I) -> Result<Self> {
		let i = iter.collect::<Result<Vec<_>>>()?;
		let 

		// match iter.next().transpose()? {
		// 	pub enum Token {
		// 		Literal(Literal),
		// 		Operator(Operator),
		// 		Left(ParenType),
		// 		Right(ParenType),
		// 	}
		// }
		// while let Some(iter) = iter.next().transpose()? {

			unimplemented!()
		// }
		// Expression::try_from_iter(iter)
	}
}
















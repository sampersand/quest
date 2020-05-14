use crate::parse::{Result, Error, Token, token::{Literal, Operator, ParenType}};
use std::iter::Peekable;

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
	// E -> 
	Blank,
	// E -> <literal>
	Literal(Literal),
	// E -> { E }
	Block(Box<Expression>),
	// E -> ( E )
	Grouping(Box<Expression>),
	// E -> E ( E )
	FunctionCall(Box<Expression>, Box<Expression>),
	// E -> E [ E ]
	Index(Box<Expression>, Box<Expression>),
	// E -> E <infix> E 
	InfixOperator(Box<Expression>, Operator, Box<Expression>),
	// E -> <prefix> E 
	PrefixOperator(Operator, Box<Expression>),
	// E -> E <postfix>
	PostfixOperator(Box<Expression>, Operator),
}

impl Expression {
	pub fn try_from_iter<I: Iterator<Item=Result<Token>>>(mut iter: I) -> Result<Self> {
		Expression::try_from_iter_peekable(&mut iter.peekable())
	}

	pub fn try_from_iter_peekable<I: Iterator<Item=Result<Token>>>(iter: &mut Peekable<I>) -> Result<Self> {
		use ParenType::*;
		use Token::*;

		let expr = match iter.next().transpose()? {
			None => return Ok(Expression::Blank),
			Some(Literal(lit)) => Expression::Literal(lit),
			Some(Operator(op)) => Expression::PrefixOperator(op, 
				Box::new(Expression::try_from_iter_peekable(iter)?)),
			Some(Right(bracket)) => return Err(Error::Message(format!("dangling bracket: {:?}", bracket))),
			Some(Left(Paren)) => {
				let inner = Expression::try_from_iter_peekable(iter)?;
				match iter.next().transpose()? {
					Some(Right(Paren)) => Expression::Grouping(Box::new(inner)),
					Some(Right(other)) => return Err(Error::BadClosingParen(Paren, other)),
					Some(..) => return Err(Error::NoClosingParen),
					None => return Err(Error::NoClosingParen)
				}
			},
			Some(Left(Brace)) => {
				let mut inner = Expression::try_from_iter_peekable(iter)?;
				match iter.next().transpose()? {
					Some(Right(Brace)) => Expression::Block(Box::new(inner)),
					Some(Right(other)) => return Err(Error::BadClosingParen(Brace, other)),
					Some(..) => return Err(Error::NoClosingParen),
					None => return Err(Error::NoClosingParen)
				}
			},
			other => unimplemented!("{:?}", other)
		};

		match iter.next().transpose()? {
			None => Ok(expr),
			Some(Left(Paren)) => {
				let args = Expression::try_from_iter_peekable(iter)?;
				match iter.next().transpose()? {
					Some(Right(Paren)) => Ok(Expression::FunctionCall(Box::new(expr), Box::new(args))),
					Some(Right(other)) => return Err(Error::BadClosingParen(Paren, other)),
					Some(..) => return Err(Error::NoClosingParen),
					None => return Err(Error::NoClosingParen)
				}
			},
			Some(Left(Bracket)) => {
				let args = Expression::try_from_iter_peekable(iter)?;
				match iter.next().transpose()? {
					Some(Right(Bracket)) => Ok(Expression::Index(Box::new(expr), Box::new(args))),
					Some(Right(other)) => return Err(Error::BadClosingParen(Bracket, other)),
					Some(..) => return Err(Error::NoClosingParen),
					None => return Err(Error::NoClosingParen)
				}
			},
			Some(Right(paren)) => Err(Error::Message(format!("unexpected rparen: {:?}, {:?}", expr, paren))),
			Some(Left(paren)) => Err(Error::Message(format!("unexpected lparen: {:?}", paren))),
			Some(Literal(lit)) => Err(Error::Message(format!("unexpected literal: {:?}", lit))),
			// this doesn't take into account order of operations...
			Some(Operator(op)) => Ok(Expression::InfixOperator(Box::new(expr), op, Box::new(Expression::try_from_iter_peekable(iter)?))),
		}
	}
}



// impl Expression {
// 	fn next_base_expression<I: Iterator<Item=Result<Token>>>(iter: &mut Peekable<I>) -> Result<Expression> {
// 		let x = match iter.next().transpose()? {
// 			None => Ok(Expression::Blank),
// 			Some(Token::Literal(literal)) => Ok(Expression::Literal(literal)),
// 			Some(Token::Left(ParenType::Brace)) => {
// 				let next = Expression::next_expression(iter)?;
// 				match iter.next().transpose()? {
// 					Some(Token::Right(ParenType::Brace)) => Ok(Expression::Block(Box::new(next))),
// 					Some(Token::Right(other)) => Err(Error::Message(format!("bad closing paren: {:?}", other))),
// 					_ => Err(Error::Message(format!("dangling curly brace"))),
// 				}
// 			},
// 			Some(Token::Left(ParenType::Paren)) => {
// 				let next = Expression::next_expression(iter)?;
// 				match iter.next().transpose()? {
// 					Some(Token::Right(ParenType::Paren)) => Ok(Expression::Grouping(Box::new(next))),
// 					Some(Token::Right(other)) => Err(Error::Message(format!("bad closing paren: {:?}", other))),
// 					_ => Err(Error::Message(format!("dangling paren"))),
// 				}
// 			},
// 			Some(Token::Operator(prefix)) => match Expression::next_expression(iter)? {
// 				Expression::Blank => Err(Error::Message(format!("dangling prefix operator"))),
// 				expr => Ok(Expression::PrefixOperator(prefix, Box::new(expr))),
// 			},
// 			other => unimplemented!("{:?}", other)
// 		};
// 		println!("{:?}", x);
// 		x
// 	}

// 	fn next_expression<I: Iterator<Item=Result<Token>>>(iter: &mut Peekable<I>) -> Result<Expression> {
// 		let expr = Expression::next_base_expression(iter)?;
// 		if let Some(Ok(tkn)) = iter.peek() {
// 			match tkn {
// 				Token::Left(ParenType::Paren) => {
// 					assert!(matches!(iter.next(), Some(Ok(Token::Left(ParenType::Paren)))));
// 					println!("a");
// 					let next = Expression::next_expression(iter)?;
// 					println!("b");
// 					match iter.next().transpose()? {
// 						Some(Token::Right(ParenType::Paren)) => return Ok(Expression::FunctionCall(Box::new(expr), Box::new(next))),
// 						Some(Token::Right(other)) => return Err(Error::Message(format!("bad closing paren: {:?}", other))),
// 						_ => return Err(Error::Message(format!("dangling paren"))),
// 					};
// 				},
// 				Token::Operator(oper) => return Ok(Expression::InfixOperator(Box::new(expr), *oper, Box::new(Expression::next_expression(iter)?))),
// 				_ => return Ok(expr)
// 			// }
// 			// 	// E -> 
// 			// 	Blank,
// 			// 	// E -> <literal>
// 			// 	Literal(Literal),
// 			// 	// E -> { E }
// 			// 	Block(Box<Expression>),
// 			// 	// E -> ( E )
// 			// 	Grouping(Box<Expression>),
// 			// 	// E -> E ( E )
// 			// 	FunctionCall(Box<Expression>, Box<Expression>),
// 			// 	// E -> E <infix> E 
// 			// 	InfixOperator(Box<Expression>, Operator, Box<Expression>),
// 			// 	// E -> <prefix> E 
// 			// 	PrefixOperator(Operator, Box<Expression>),
// 			// 	// E -> E <postfix>
// 			// 	PostfixOperator(Box<Expression>, Operator),
// 			// }

// 				// Token::Literal()
// 			}
// 		}
// 		Ok(expr)
// 	}
// 	// 	Some(match v.get(0)? {
// 	// 		Token::Literal(literal) => Expression::Literal(literal),
// 	// 		Token::Left(ParenType::Brace) => todo!(),
// 	// 		Token::Left(ParenType::Brace) => todo!(),
// 	// 	})
// 	// }
// 	pub fn try_from_iter<I: Iterator<Item=Result<Token>>>(mut iter: I) -> Result<Self> {
// 		// let i = iter.collect::<Result<Vec<_>>>()?;
// 		Expression::next_expression(&mut iter.peekable())
// 		// }
// 		// Expression::try_from_iter(iter)
// 	}
// }















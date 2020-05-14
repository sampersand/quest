mod err;
pub use self::err::Error;
use crate::parse::{Result, Token, token::{self, Literal, ParenType, Operator}};
use std::iter::Peekable;

#[derive(Debug)]
pub enum Expression {
	Blank,
	Literal(Literal),
	Block(ParenType, Box<Expression>),
	FunctionCall(ParenType, Box<Expression>, Box<Expression>),
	Operator(Operator, Vec<Expression>),
}
/*
expr -> literal | block | meth_call | op | 
literal -> LITERAL
block -> '(' expr ')' | '{' expr '}' | '[' expr ']'
meth_call -> expr '(' expr ')'
op -> PREFIX expr | expr POSTFIX | expr INFIX expr
*/

// fn parse_primary<I: Iterator<Item=Result<Token>>>(iter: &mut Peekable<I>) -> Result<Expression> {
// 	match iter.next().transpose()? {
// 		None => Ok(Expression::Blank),
// 		Some(Token::Literal(lit)) => Ok(Expression::Literal(lit)),
// 		Some(Token::Left(paren)) => unimplemented!(),
// 		Some(Token::Right(paren)) => Err(Error::UnexpectedRightParen(paren).into()),
// 		Some(Token::Operator(op)) => unimplemented!()
// 	}
// }

// fn parse_infix<I>(iter: &mut Peekable<I>, mut lhs: Expression, min: Operator) -> Result<Expression>
// where I: Iterator<Item=Result<Token>>
// {
// 	// copied from https://en.wikipedia.org/wiki/Operator-precedence_parser
// 	// i'll fix it up later, but for now i just need something working.
// 	let mut lookahead = iter.peek();
// 	loop {
// 		let op: Operator = match lookahead {
// 			Some(Ok(Token::Operator(opref))) if *opref >= min => 
// 				if let Token::Operator(op) = iter.next().unwrap().unwrap() {
// 					op
// 				} else {
// 					unreachable!();
// 				},
// 			_ => break
// 		};
// 		let mut rhs = parse_primary(iter)?;
// 		lookahead = iter.peek();
// 		loop {
// 			let next_op: Operator = match lookahead {
// 				Some(Ok(Token::Operator(next_op))) if (*next_op > op || *next_op == op &&
// 					(op.assoc() == token::operator::Associativity::RightToLeft)) => 
// 					if let Token::Operator(next_op) = iter.next().unwrap().unwrap() {
// 						next_op
// 					} else {
// 						unreachable!();
// 					},
// 				_ => break
// 			};

// 			rhs = parse_infix(iter, rhs, next_op)?;
// 			lookahead = iter.peek();
// 		}
// 		lhs = Expression::Operator(op, vec![lhs, rhs]);
// 	}

// 	Ok(lhs)
//  //    while lookahead is a binary operator whose precedence is >= min_precedence
//  //        op := lookahead
//  //        advance to next token
//  //        rhs := parse_primary ()
//  //        lookahead := peek next token
//  //        while lookahead is a binary operator whose precedence is greater
//  //                 than op's, or a right-associative operator
//  //                 whose precedence is equal to op's
//  //            rhs := parse_expression_1 (rhs, lookahead's precedence)
//  //            lookahead := peek next token
//  //        lhs := the result of applying op with operands lhs and rhs
//  //    return lhs
// }

#[derive(Debug)]
enum TokenOrBlock {
	Token(Token),
	Block(ParenType, Vec<TokenOrBlock>)
}

fn get_block<'a, I>(iter: &'a mut I, end: ParenType) -> impl Iterator<Item=Result<TokenOrBlock>> + 'a
where I: Iterator<Item=Result<Token>> + 'a {
	struct GetBlock<'a, I>(&'a mut I, ParenType);

	impl<'a, I: Iterator<Item=Result<Token>>> Iterator for GetBlock<'a, I> {
		type Item = Result<TokenOrBlock>;
		fn next(&mut self) -> Option<Self::Item> {
			match self.0.next()? {
				Ok(Token::Right(paren)) if paren == self.1 => None,
				Ok(Token::Left(paren)) => Some(get_block(self.0, paren).collect::<Result<_>>()
					.map(|blk| TokenOrBlock::Block(paren, blk))),
				Ok(token) => Some(Ok(TokenOrBlock::Token(token))),
				Err(err) => Some(Err(err))
			}
		}
	}

	GetBlock(iter, end)
}

impl Expression {
	pub fn try_from_iter<I: Iterator<Item=Result<Token>> + std::fmt::Debug>(iter: &mut I) -> Result<Self> {
		let ref mut peek = iter.peekable();
		let mut lhs = Expression::try_from_peekable(peek)?;
		while peek.peek().is_some() {
			println!("peek: {:?}", peek);
			lhs = Expression::try_from_lookahead(peek, lhs, None)?;
		}
		// assert!(matches!(iter.next(), None));
		Ok(lhs)
	}

	fn try_from_peekable<I: Iterator<Item=Result<Token>>>(iter: &mut Peekable<I>) -> Result<Self> {
		let primary = Expression::get_primary(iter)?;
		Expression::try_from_lookahead(iter, primary, None)
	}

	fn get_primary<I: Iterator<Item=Result<Token>>>(iter: &mut Peekable<I>) -> Result<Self> {
		match iter.next().transpose()? {
			None => Ok(Expression::Blank),
			Some(Token::Literal(lit)) => Ok(Expression::Literal(lit)),
			Some(Token::Left(paren)) => {
				let body = Expression::try_from_peekable(iter)?;
				match iter.next().transpose()? {
					Some(Token::Right(rparen)) if rparen == paren => Ok(Expression::Block(paren, Box::new(body))),
					Some(Token::Right(rparen)) => Err(Error::MismatchedParen(paren, rparen).into()),
					_ => Err(Error::MissingRightParen(paren).into()),
				}
			},
			Some(Token::Right(paren)) => Err(Error::UnexpectedRightParen(paren).into()),
			Some(Token::Operator(op)) if op.assoc() == token::operator::Associativity::UnaryOperOnLeft => Ok(
				Expression::Operator(op, vec![Expression::try_from_peekable(iter)?])
			),
			Some(Token::Operator(op)) => Err(Error::UnexpectedOperator(op).into()),
		}
	}



	fn try_from_lookahead<I: Iterator<Item=Result<Token>>>(iter: &mut Peekable<I>, mut lhs: Expression, end: Option<Operator>) -> Result<Self> {
		while let Some(token) = iter.peek() {
			match token {
				Err(err) => panic!("todo: err handling here ({:?})", err),
				Ok(Token::Left(paren)) => {
					let paren = *paren; // deref it so we can use iter again.
					assert_eq!(iter.next().unwrap().unwrap(), Token::Left(paren));
					let args = Expression::try_from_peekable(iter)?;
					lhs = Expression::FunctionCall(paren, Box::new(lhs), Box::new(args));
				},
				Ok(Token::Operator(op)) if op.assoc() != token::operator::Associativity::UnaryOperOnLeft => {
					let op = *op;
					if !end.map(|x| op >= x).unwrap_or(true) {
						break;
					}

					assert_eq!(iter.next().unwrap().unwrap(), Token::Operator(op));
					let primary = Expression::get_primary(iter)?;
					let rhs = Expression::try_from_lookahead(iter, primary, Some(op))?;
					lhs = Expression::Operator(op, vec![lhs, rhs]);
				},
				_ => break
			};
		}
		Ok(lhs)
		// match iter.peek() {`
		// match iter.peek() {
		// 	None => Ok(expr),
		// 	Some(Ok(Token::Left(paren))) => {
		// 		let paren = *paren; // deref it so we can use iter again.
		// 		let n = iter.next();
		// 		debug_assert_eq!(n.unwrap().unwrap(), Token::Left(paren));
		// 		let args = Expression::try_from_peekable(iter)?;
		// 		Ok(Expression::FunctionCall(paren, Box::new(expr), Box::new(args)))
		// 	},
		// 	Some(Token::Operator(op)) if op.assoc() == token::operator::Associativity::UnaryOperOnLeft => {
		// 		Expression::Operator(op, vec![Expression::try_from_peekable(iter)?])
		// 	},
		// 	None | Some(Ok(Token::Literal(..))) | Some(Ok(Token::Right(..))) => Ok(expr)
		// }
	}

	// fn try_from_iter_prec<I: Iterator<Item=Result<Token>>>(iter: &mut Peekable<I>, min: Option<Operator>) -> Result<Self> {
	// 	#[derive(Debug)]
	// 	enum ExprOrToken {
	// 		Expr(Expression),
	// 		Left(ParenType),
	// 	}
	// 	let mut stack = Vec::<ExprOrToken>::new();
	// 	while let Some(token) = iter.next().transpose()? {
	// 		stack.push(match token {
	// 			Token::Literal(lit) => ExprOrToken::Expr(Expression::Literal(lit)),
	// 			Token::Left(paren) => ExprOrToken::Left(paren),
	// 			Token::Operator(op) => {
	// 				stack.push(ExprOrToken::try_from_iter_prec(iter, op.))
	// 			}
	// 		})
	// 	}

	// 	unimplemented!("{:?}", stack)
	// 	// Expression::try_from_iter_bound(&mut iter.peekable(), None)
	// }

	// fn try_from_iter_bound<I: Iterator<Item=Result<Token>>>(iter: &mut Peekable<I>, end: Option<Operator>) -> Result<Self> {

	// 	unimplemented!()

	// }
}
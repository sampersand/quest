mod err;
pub use self::err::Error;
use crate::parse::{Result, Token, token::{self, Literal, ParenType, Operator}};
use std::iter::Peekable;

#[derive(Debug)]
pub enum Expression {
	Blank,
	Literal(Literal),
	Block(ParenType, Box<Expression>),
	Operator(Operator, Vec<Expression>),
}
/*
expr -> literal | block | meth_call | op | 
literal -> LITERAL
block -> '(' expr ')' | '{' expr '}' | '[' expr ']'
meth_call -> expr '(' expr ')'
op -> PREFIX expr | expr POSTFIX | expr INFIX expr
*/

fn parse_primary<I: Iterator<Item=Result<Token>>>(iter: &mut Peekable<I>) -> Result<Expression> {
	match iter.next().transpose()? {
		None => Ok(Expression::Blank),
		Some(Token::Literal(lit)) => Ok(Expression::Literal(lit)),
		Some(Token::Left(paren)) => unimplemented!(),
		Some(Token::Right(paren)) => Err(Error::UnexpectedRightParen(paren).into()),
		Some(Token::Operator(op)) => unimplemented!()
	}
}

fn parse_infix<I>(iter: &mut Peekable<I>, mut lhs: Expression, min: Operator) -> Result<Expression>
where I: Iterator<Item=Result<Token>>
{
	// copied from https://en.wikipedia.org/wiki/Operator-precedence_parser
	// i'll fix it up later, but for now i just need something working.
	let mut lookahead = iter.peek();
	loop {
		let op: Operator = match lookahead {
			Some(Ok(Token::Operator(opref))) if *opref >= min => 
				if let Token::Operator(op) = iter.next().unwrap().unwrap() {
					op
				} else {
					unreachable!();
				},
			_ => break
		};
		let mut rhs = parse_primary(iter)?;
		lookahead = iter.peek();
		loop {
			let next_op: Operator = match lookahead {
				Some(Ok(Token::Operator(next_op))) if (*next_op > op || *next_op == op &&
					(op.assoc() == token::operator::Associativity::RightToLeft)) => 
					if let Token::Operator(next_op) = iter.next().unwrap().unwrap() {
						next_op
					} else {
						unreachable!();
					},
				_ => break
			};

			rhs = parse_infix(iter, rhs, next_op)?;
			lookahead = iter.peek();
		}
		lhs = Expression::Operator(op, vec![lhs, rhs]);
	}

	Ok(lhs)
 //    while lookahead is a binary operator whose precedence is >= min_precedence
 //        op := lookahead
 //        advance to next token
 //        rhs := parse_primary ()
 //        lookahead := peek next token
 //        while lookahead is a binary operator whose precedence is greater
 //                 than op's, or a right-associative operator
 //                 whose precedence is equal to op's
 //            rhs := parse_expression_1 (rhs, lookahead's precedence)
 //            lookahead := peek next token
 //        lhs := the result of applying op with operands lhs and rhs
 //    return lhs
}

impl Expression {
	pub fn try_from_iter<I: Iterator<Item=Result<Token>>>(iter: &mut I) -> Result<Self> {
		let ref mut iter = iter.peekable();
		let primary = parse_primary(iter)?;
		parse_infix(iter, primary, Operator::Dot)
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
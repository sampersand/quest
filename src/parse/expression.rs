mod err;
pub use self::err::Error;
use crate::parse::{Result, Token, token::{self, Literal, ParenType, Operator, operator::Associativity}};
use std::iter::Peekable;


#[derive(Debug)]
struct Line(Vec<Expression>);

#[derive(Debug)]
pub struct Block {
	paren: ParenType,
	body: Vec<Line>,
}

#[derive(Debug)]
pub enum Expression {
	Literal(Literal),
	Block(Block),
	FunctionCall(Box<Expression>, Block),
	PrefixOp(Operator, Box<Expression>),
	InfixOp(Operator, Box<Expression>, Box<Expression>)
}



// expr -> primary | function_call | infix
// primary -> <LITERAL> | block | <PREFIX_OP> expr
// block -> '(' block_inner ')' | '{' block_inner '}' | '[' block_inner ']'
// block_inner -> (block_inner_line? ';')* block_inner_line?
// block_inner_line -> expr (',' expr)*
// block_inner -> (expr ';')* (expr (',' expr)*)?
// function_call -> expr block
// infix -> expr INFIX_OP expr 	(with correct order-of-oper)


impl Expression {
	pub fn try_from_iter<I: Iterator<Item=Token>>(iter: &mut I) -> Result<Self> {
		let ref mut iter = std::iter::once(Token::Left(ParenType::Paren)).chain(iter)
			.chain(std::iter::once(Token::Right(ParenType::Paren))).peekable();
		let expr = next_expression(iter)?;
		assert_eq!(iter.next(), None, "stuff remains in iter");
		Ok(expr)
	}
}

fn next_expression_bound<I: Iterator<Item=Token>>(iter: &mut Peekable<I>, lhs: Expression, end: Option<Operator>) -> Result<Expression> {
	match iter.peek() {
		Some(Token::Left(paren)) => {
			let paren = *paren;
			assert_eq!(iter.next().unwrap(), Token::Left(paren));
			Ok(Expression::FunctionCall(Box::new(lhs), next_block(iter, paren)?))
		},
		// <-- if we had postfix operators, they'd go here.
		Some(Token::Operator(op)) if op.arity() == 2 && end.as_ref().map(|end| *op >= *end).unwrap_or(true) => {
			let op = *op;
			assert_eq!(iter.next().unwrap(), Token::Operator(op));
			let mut rhs = next_expression(iter)?;
			// let mut rhs = next_primary(iter)?;
			// if let Some(Token::Operator(rop)) = iter.peek() {
			// 	let rop = *rop;
			// 	if rop < op || (op.cmp(&rop) == std::cmp::Ordering::Equal && rop.assoc() == Associativity::RightToLeft) {
			// 		rhs = next_expression_bound(iter, rhs, Some(rop))?;
			// 	}
			// }

			// loop {
			// 	let lookahead = iter.peek();
			// 	match lookahead {
			// 		Some(Token::Operator(rop)) if (*rop < op ||
			// 			op.cmp(rop) == std::cmp::Ordering::Equal && rop.assoc() == Associativity::RightToLeft) => {
			// 			let rop = *rop;
			// 			rhs = next_expression_bound(iter, rhs, Some(rop))?;
			// 		},
			// 		_ => break
			// 	}
			// };
			Ok(Expression::InfixOp(op, Box::new(lhs), Box::new(rhs)))
		},
		None | Some(_) => Ok(lhs),
	}
}

fn next_expression<I: Iterator<Item=Token>>(iter: &mut Peekable<I>) -> Result<Expression> {
	let primary = next_primary(iter)?;
	next_expression_bound(iter, primary, None)
}

// primary -> LITERAL | PREFIX_OP expr | block
fn next_primary<I: Iterator<Item=Token>>(iter: &mut Peekable<I>) -> Result<Expression> {
	match iter.next().ok_or(Error::NoTokens)? {
		Token::Literal(lit) => Ok(Expression::Literal(lit)),
		Token::Operator(op) if op.assoc() == Associativity::UnaryOperOnLeft => Ok(
			Expression::PrefixOp(op, Box::new(next_expression(iter)?))
		),
		Token::Left(paren) => Ok(Expression::Block(next_block(iter, paren)?)),
		token => dbg!(Err(Error::UnexpectedToken(token).into()))
	}
}

// block -> '(' block_inner ')' | '[' block_inner ']' | '{' block_inner '}'
// block_inner -> (block_line? ';')* block_line?
// block_line -> (expr (',' expr)*)?
fn next_block<I: Iterator<Item=Token>>(iter: &mut Peekable<I>, paren: ParenType) -> Result<Block> {
	let mut block = Block { paren, body: vec![] };

	fn next_line<I: Iterator<Item=Token>>(iter: &mut Peekable<I>) -> Result<Line> {
		let mut args = Line(vec![]);
		loop {
			args.0.push(next_expression(iter)?);
			match iter.peek() {
				Some(Token::Endline) | Some(Token::Right(..)) => break,
				Some(Token::Comma) => assert_eq!(iter.next().unwrap(), Token::Comma),
				_ => return dbg!(Err(Error::UnexpectedToken(iter.next().unwrap()).into()))
			}
		}

		Ok(args)
	}

	loop {
		match iter.peek() {
			None => return Err(Error::MissingRightParen(paren).into()),
			Some(Token::Right(rparen)) =>
				if *rparen == block.paren {
					assert_eq!(iter.next().unwrap(), Token::Right(paren));
					break;
				} else {
					return Err(Error::MismatchedParen(paren, *rparen).into());
				},
			Some(Token::Endline) => assert_eq!(iter.next().unwrap(), Token::Endline),
			_ => block.body.push(next_line(iter)?)
		}
	}

	Ok(block)
}






// fn next_block<I: Iterator<Item=Token>>(iter: &mut Peekable<I>, paren: ParenType) -> Result<Block> {
// 	let mut block = Block { paren, body: vec![] };

// 	fn next_line<I: Iterator<Item=Token>>(iter: &mut Peekable<I>) -> Result<Line> {
// 		let mut args = Line(vec![]);
// 		loop {
// 			args.0.push(next_expression(iter)?);
// 			match iter.peek() {
// 				Some(Token::Endline) | Some(Token::Right(..)) => break,
// 				Some(Token::Comma) => assert_eq!(iter.next().unwrap(), Token::Comma),
// 				_ => return dbg!(Err(Error::UnexpectedToken(iter.next().unwrap()).into()))
// 			}
// 		}

// 		Ok(args)
// 	}

// 	loop {
// 		match iter.peek() {
// 			None => return Err(Error::MissingRightParen(paren).into()),
// 			Some(Token::Right(rparen)) =>
// 				if *rparen == block.paren {
// 					assert_eq!(iter.next().unwrap(), Token::Right(paren));
// 					break;
// 				} else {
// 					return Err(Error::MismatchedParen(paren, *rparen).into());
// 				},
// 			Some(Token::Endline) => assert_eq!(iter.next().unwrap(), Token::Endline),
// 			_ => block.body.push(next_line(iter)?)
// 		}
// 	}

// 	Ok(block)

// 	// fn next_block_inner_line<I: Iterator<Item=Token>>(iter: &mut Peekable<I>) -> Result<BlockInnerLine> {
// 	// 	let mut exprs = BlockInnerLine(vec![next_expression(iter)?]);
// 	// 	while let Some(peek) = iter.peek() {
// 	// 		if *peek == Token::Comma {
// 	// 			exprs.0.push(next_expression(iter)?)
// 	// 		} else {
// 	// 			break;
// 	// 		}
// 	// 	}
// 	// 	Ok(exprs)
// 	// }
// 	// let mut body = vec![];
// 	// while let Some(token) = iter.peek() {
// 	// 		Token::Right(rparen) if *rparen == paren => {
// 	// 			let rparen = *rparen;
// 	// 			assert_eq!(iter.next().unwrap(), Token::Right(paren));
// 	// 			return Ok(Expression::Block(Block { paren, body }));
// 	// 		},
// 	// 		_ => body.push(next_block_inner_line(iter)?)
// 	// 	}
// 	// }
// 	// unimplemented!()
// }






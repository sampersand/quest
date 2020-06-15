use crate::Result;
use crate::token::{Token, ParenType};
use crate::stream::Contexted;
use crate::expression::{Constructable, Expression, PutBack};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
	lines: Vec<Vec<Expression>>,
	paren_type: ParenType,
	returns: bool
}

impl Display for Block {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}", self.paren_type.left())?;

		if self.lines.len() == 1 {
			write!(f, " ")?;
		} else if self.lines.len() > 1 {
			write!(f, "\n")?;
		}

		for (i, line) in self.lines.iter().enumerate() {
			if self.lines.len() > 1 {
				write!(f, "\t")?;
			}

			let mut is_first_expr = true;

			for expr in line {
				if is_first_expr {
					is_first_expr = false;
				} else {
					write!(f, ", ")?
				}
				Display::fmt(&expr, f)?;
			}

			if i < self.lines.len() && (i != self.lines.len() - 1 || !self.returns) {
				write!(f, ";")?
			}

			if self.lines.len() > 1 {
				write!(f, "\n")?;
			}
		}

		if self.lines.len() == 1 {
			write!(f, " ")?;
		}
		write!(f, "{}", self.paren_type.right())
	}
}

impl Block {
	pub fn paren_type(&self) -> ParenType {
		self.paren_type
	}
}

impl Constructable for Block {
	type Item = Self;
	fn try_construct_primary<C>(ctor: &mut C) -> Result<Option<Self>>
	where
		C: Iterator<Item=Result<Token>> + PutBack + Contexted
	{
		let paren = 
			match ctor.next().transpose()? {
				Some(Token::Left(paren)) => paren,
				Some(tkn) => { ctor.put_back(Ok(tkn)); return Ok(None) },
				None => return Ok(None)
			};

		let mut block = Block { lines: vec![], paren_type: paren, returns: false };
		let mut curr_line: Option<Vec<Expression>> = None;

		while let Some(tkn) = ctor.next().transpose()? {
			match tkn {
				Token::Right(rparen) if rparen == paren => {
					if let Some(curr_line) = curr_line {
						block.lines.push(curr_line);
					}

					return Ok(Some(block))
				},

				rparen @ Token::Right(..) => return Err(parse_error!(ctor, UnexpectedToken(rparen))),
				Token::Endline => {
					block.returns = false;
					if let Some(curr_line) = curr_line.take() {
						block.lines.push(curr_line);
					}
				},
				Token::Comma => { /* do nothing */ },
				other => {
					block.returns = true;
					ctor.put_back(Ok(other));
					curr_line.get_or_insert_with(|| Vec::with_capacity(1))
						.push(Expression::try_construct(ctor)?);
				}
			}
		}

		Err(parse_error!(ctor, MissingClosingParen(paren)))
	}
}












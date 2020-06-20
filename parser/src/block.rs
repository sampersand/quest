
#[macro_use]
use quest::impl_object_type;
use quest::{Object, Args, types::{self, rustfn::Binding}};

use crate::Result;
use crate::token::{Token, ParenType};
use crate::stream::Contexted;
use crate::expression::{Constructable, Expression, PutBack, Executable};
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Line {
	Single(Expression),
	Multiple(Vec<Expression>)
}

#[derive(Clone, PartialEq, Eq)]
pub struct Block {
	lines: Vec<Line>,
	paren_type: ParenType,
	returns: bool
}

impl Debug for Block {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			f.debug_struct("Block")
				.field("lines", &self.lines)
				.field("paren_type", &self.paren_type)
				.field("returns", &self.returns)
				.finish()
		} else {
			write!(f, "Block({})", self)
		}
	}
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

			Display::fmt(line, f)?;

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


impl Display for Line {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Line::Single(expr) => Display::fmt(expr, f),
			// OPTIMIZE: I'm sure there's a builtin way to make this easier
			Line::Multiple(exprs) => {
				let mut is_first_expr = true;
				for expr in exprs.iter() {
					if is_first_expr {
						is_first_expr = false;
					} else {
						write!(f, ", ")?
					}
					Display::fmt(&expr, f)?;
				}
				Ok(())
			}
		}
	}
}

pub enum LineResult {
	Single(quest::Object),
	Multiple(Vec<quest::Object>)
}

impl From<LineResult> for quest::Object {
	fn from(lr: LineResult) -> Self {
		match lr {
			LineResult::Single(obj) => obj,
			LineResult::Multiple(objs) => objs.into()
		}
	}
}

impl Line {
	fn execute(&self) -> quest::Result<LineResult> {
		match self {
			Line::Single(expr) => expr.execute().map(LineResult::Single),
			Line::Multiple(exprs) => exprs
				.iter()
				.map(Executable::execute)
				.collect::<quest::Result<_>>()
				.map(LineResult::Multiple)
		}
	}
}

impl Block {
	pub fn paren_type(&self) -> ParenType {
		self.paren_type
	}

	pub(super) fn run_block(&self) -> quest::Result<Option<LineResult>> {
		if let Some(last) = self.lines.last() {
			for line in &self.lines[..self.lines.len() - 1] {
				line.execute()?;
			}

			let ret = last.execute()?;

			if self.returns {
				return Ok(Some(if self.paren_type == ParenType::Square {
					match ret {
						LineResult::Single(expr) => LineResult::Multiple(vec![expr]),
						other => other
					}
				} else {
					ret
				}));
			}
		}

		Ok(None)
	}

	fn call(&self, args: Args) -> quest::Result<quest::Object> {
		Binding::new_stackframe(args, (|_binding| {
			self.run_block()
				.map(|x| x.map(Object::from))
				.map(Option::unwrap_or_default)
		}))
	}

}

impl Executable for Block {
	fn execute(&self) -> quest::Result<quest::Object> {
		if self.paren_type == ParenType::Curly {
			let block = Object::from(self.clone());
			block.add_parent(Binding::instance().as_ref().clone())?;
			Ok(block)
		} else {
			self.run_block()
				.map(|x| x.map(Object::from))
				.map(Option::unwrap_or_default)
		}
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
		let mut curr_line: Option<Line> = None;

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
				Token::Comma => 
					match curr_line {
						Some(Line::Multiple(_)) => { /* do nothing; commas are used to make `multiple` */},
						Some(Line::Single(first)) => curr_line = Some(Line::Multiple(vec![first])),
						None => curr_line = Some(Line::Multiple(vec![]))
					},

				other => {
					block.returns = true;
					ctor.put_back(Ok(other));
					let expr = Expression::try_construct(ctor)?;
					match curr_line {
						Some(Line::Multiple(ref mut exprs)) => exprs.push(expr),
						Some(Line::Single(first)) => curr_line = Some(Line::Multiple(vec![first, expr])),
						None => curr_line = Some(Line::Single(expr))
					}
				}
			}
		}

		Err(parse_error!(ctor, MissingClosingParen(paren)))
	}
}

mod impls {
	use super::*;

	pub fn call(mut args: Args) -> quest::Result<Object> {
		let this = args.this()?.try_downcast_ref::<Block>()?.clone();
		this.call(args)
	}
}


impl_object_type!{
for Block [(parents quest::types::Function)]:
	"@text" => (|args| Ok(format!("{:?}", *args.this()?.try_downcast_ref::<Block>()?).into())),
	"()" => impls::call
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[ignore]
	fn call() { todo!(); }
}









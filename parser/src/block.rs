use quest_core::impl_object_type;
use quest_core::{Object, Args, Binding};

use crate::Result;
use crate::token::{Token, ParenType};
use crate::stream::{Context, Contexted};
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
	context: Context,
}

impl Debug for Block {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			f.debug_struct("Block")
				.field("lines", &self.lines)
				.field("paren_type", &self.paren_type)
				.field("context", &self.context)
				.finish()
		} else {
			write!(f, "Block({})", self)
		}
	}
}


impl Display for Block {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if self.lines.is_empty() {
			return Display::fmt(&self.paren_type, f);
		}

		let (last, body) = self.lines.split_last().unwrap();
		Display::fmt(&self.paren_type.left(), f)?;

		if body.is_empty() || !f.alternate() {
			if self.paren_type == ParenType::Curly {
				write!(f, " ")?;
			}

			for line in body { // in case `!f.alternate()`
				Display::fmt(line, f)?;
				write!(f, ";")?;
			}
			Display::fmt(last, f)?;

			if self.paren_type == ParenType::Curly {
				write!(f, " ")?;
			}
		} else {
			fn write_tabbed(mut s: &str, f: &mut Formatter) -> fmt::Result {
				while !s.is_empty() {
					write!(f, "\t")?;
					let split = s.find('\n').map(|x| x + 1).unwrap_or_else(|| s.len());
					f.write_str(&s[..split])?;
					s = &s[split..];
				}
				writeln!(f, ";")
			}

			writeln!(f)?;
			for line in body {
				write_tabbed(&format!("{:#}", line), f)?;
			}

			write_tabbed(&format!("{:#}", last), f)?;
		}


		Display::fmt(&self.paren_type.right(), f)
	}
}

// impl Display for Block {
// 	#[allow(clippy::all)]
// 	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
// 		write!(f, "{}", self.paren_type.left())?;


// 		if self.lines.len() == 1 {
// 			write!(f, " ")?;
// 		} else if self.lines.len() > 1 {
// 			writeln!(f)?;
// 		}

// 		for (i, line) in self.lines.iter().enumerate() {
// 			if self.lines.len() > 1 {
// 				write!(f, "\t")?;
// 			}

// 			Display::fmt(line, f)?;

// 			if i < self.lines.len() && i != self.lines.len() - 1 {
// 				write!(f, ";")?
// 			}

// 			if self.lines.len() > 1 {
// 				writeln!(f)?;
// 			}
// 		}

// 		if self.lines.len() == 1 {
// 			write!(f, " ")?;
// 		}
// 		write!(f, "{}", self.paren_type.right())
// 	}
// }


impl Display for Line {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::Single(expr) => Display::fmt(expr, f),
			// OPTIMIZE: I'm sure there's a builtin way to make this easier
			Self::Multiple(exprs) => {
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
	Single(quest_core::Object),
	Multiple(Vec<quest_core::Object>)
}

impl LineResult {
	fn force_multiple(self) -> Self {
		match self {
			Self::Single(obj) => Self::Multiple(vec![obj]),
			multiple => multiple
		}
	}
}

impl From<LineResult> for quest_core::Object {
	fn from(lr: LineResult) -> Self {
		match lr {
			LineResult::Single(obj) => obj,
			LineResult::Multiple(objs) => objs.into()
		}
	}
}

impl Line {
	#[inline]
	fn execute(&self) -> quest_core::Result<LineResult> {
		match self {
			Self::Single(expr) => expr.execute().map(LineResult::Single),
			Self::Multiple(exprs) => exprs
				.iter()
				.map(Executable::execute)
				.collect::<quest_core::Result<_>>()
				.map(LineResult::Multiple)
		}
	}
}

impl Block {
	#[must_use]
	#[inline]
	pub fn paren_type(&self) -> ParenType {
		self.paren_type
	}

	pub(super) fn run_block(&self) -> quest_core::Result<Option<LineResult>> {
		if let Some((last, rest)) = self.lines.split_last() {
			for line in rest {
				line.execute()?;
			}

			let mut ret = last.execute()?;

			if self.paren_type == ParenType::Square {
				ret = ret.force_multiple();
			}

			Ok(Some(ret))
		} else if self.paren_type == ParenType::Square {
			Ok(Some(LineResult::Multiple(vec![])))
		} else {
			Ok(None)
		}
	}

	fn run_block_to_object(&self) -> quest_core::Result<quest_core::Object> {
		let lines = self.run_block()?;
		let lines_obj = lines.map(Object::from).unwrap_or_default();
		Ok(lines_obj)
	}
}

impl Executable for Block {
	fn execute(&self) -> quest_core::Result<quest_core::Object> {

		if self.paren_type == ParenType::Curly {
			let block = Object::from(self.clone());
			block.add_parent(Binding::instance().as_ref().clone())?;
			Ok(block)
		} else {
			self.run_block_to_object()
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
				Some(rparen @ Token::Right(..)) =>
					return Err(parse_error!(ctor, UnexpectedToken(rparen))),
				Some(tkn) => { ctor.put_back(Ok(tkn)); return Ok(None) },
				None => return Ok(None)
			};

		let mut block = Self {
			lines: vec![],
			paren_type: paren,
			context: ctor.context().clone(),
		};
		let mut curr_line: Option<Line> = None;

		while let Some(tkn) = ctor.next().transpose()? {
			match tkn {
				Token::Right(rparen) if rparen == paren => {
					if let Some(curr_line) = curr_line {
						block.lines.push(curr_line);
					}

					return Ok(Some(block))
				},

				rparen @ Token::Right(..) =>
					return Err(parse_error!(ctor, UnexpectedToken(rparen))),

				Token::Endline => 
					if let Some(curr_line) = curr_line.take() {
						block.lines.push(curr_line);
					},
				Token::Comma => 
					match curr_line {
						Some(Line::Multiple(_)) => { /* do nothing; commas are used to make `multiple` */},
						Some(Line::Single(first)) => curr_line = Some(Line::Multiple(vec![first])),
						None => curr_line = Some(Line::Multiple(vec![]))
					},

				other => {
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


impl Block {
	#[inline]
	pub fn qs_call(this: &Object, args: Args) -> quest_core::Result<Object> {
		let this_cloned = this.try_downcast_map(Self::clone)?;
		Binding::new_stackframe(Some(this.clone()), args, move |_| {
			/*match */this_cloned.run_block_to_object()/* {
				Ok(v) => Ok(v),
				Err(err @ quest_core::Error::Return { .. }) => Err(err),
				Err(err) => Err(err)
			}*/
		})
	}

	#[inline]
	pub fn qs_at_text(this: &Object, _: Args) -> quest_core::Result<Object> {
		this.try_downcast_map(Self::to_string).map(Object::from)
	}
}

impl_object_type!{
for Block [(parents quest_core::types::Function)]:
	"@text" => function Block::qs_at_text,
	"()" => function Block::qs_call
}

#[cfg(test)]
mod tests {
	

	#[test]
	#[ignore]
	fn call() { todo!(); }
}









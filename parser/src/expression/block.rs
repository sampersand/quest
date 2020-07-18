use quest_core::impl_object_type;
use quest_core::{Object, Args, Binding};

use crate::Result;
use crate::token::{Token, Parenthesis};
use crate::stream::{Context, Contexted};
use crate::expression::{Constructable, Expression, PutBack, Executable};
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Line {
	Single(Expression),
	Multiple(Vec<Expression>)
}

/// Represents a block of executable Quest code.
///
/// Unlike every other "core" quest type, this doesn't actually live within [`quest_core`]. That's
/// Because to create it, you need a [`context`], which is only possible whilst parsing.
#[derive(Clone, PartialEq, Eq)]
pub struct Block {
	lines: Vec<Line>,
	paren: Parenthesis,
	context: Context,
}

impl Debug for Block {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			f.debug_struct("Block")
				.field("lines", &self.lines)
				.field("paren", &self.paren)
				.field("context", &self.context)
				.finish()
		} else {
			write!(f, "Block({})", self)
		}
	}
}


impl Display for Block {
	#[allow(clippy::all)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}", self.paren.left())?;

		if self.lines.len() == 1 {
			write!(f, " ")?;
		} else if self.lines.len() > 1 {
			writeln!(f)?;
		}

		for (i, line) in self.lines.iter().enumerate() {
			if self.lines.len() > 1 {
				write!(f, "\t")?;
			}

			Display::fmt(line, f)?;

			if i < self.lines.len() && i != self.lines.len() - 1 {
				write!(f, ";")?
			}

			if self.lines.len() > 1 {
				writeln!(f)?;
			}
		}

		if self.lines.len() == 1 {
			write!(f, " ")?;
		}
		write!(f, "{}", self.paren.right())
	}
}


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
	/// Fetches the parenthesis type that was used when making this block.
	#[must_use]
	#[inline]
	pub fn paren(&self) -> Parenthesis {
		self.paren
	}

	/// Runs a block of code, returning its result.
	/// 
	/// if the [`paren`] is [`Square`](Parenthesis::Square), then the result is automatically
	/// assumed to be an array.
	pub(super) fn run_block(&self) -> quest_core::Result<Option<LineResult>> {
		if let Some((last, rest)) = self.lines.split_last() {
			for line in rest {
				line.execute()?;
			}

			let mut ret = last.execute()?;

			if self.paren == Parenthesis::Square {
				ret = ret.force_multiple();
			}

			Ok(Some(ret))
		} else if self.paren == Parenthesis::Square {
			Ok(Some(LineResult::Multiple(vec![])))
		} else {
			Ok(None)
		}
	}

	/// Runs a block of code, and then converts it to an [`Object`](quest_core::Object)
	fn run_block_to_object(&self) -> quest_core::Result<quest_core::Object> {
		let lines = self.run_block()?;
		let lines_obj = lines.map(Object::from).unwrap_or_default();
		Ok(lines_obj)
	}

	// fn call(&self, args: Args) -> quest_core::Result<quest_core::Object> {
	// 	Binding::new_stackframe(Some(self.clone()), args, |_| self.run_block_to_object())
	// }
}

impl Executable for Block {
	fn execute(&self) -> quest_core::Result<quest_core::Object> {

		if self.paren == Parenthesis::Curly {
			let block = Object::from(self.clone());
			block.add_parent(Binding::instance().as_ref().clone())?;
			Ok(block)
		} else {
			self.run_block_to_object()
		}
	}
}

impl Constructable for Block {
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

		let mut block = Self {
			lines: vec![],
			paren,
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

				rparen @ Token::Right(..) => return Err(parse_error!(ctor,
					CantCreateExpression(super::Error::UnexpectedToken(rparen).into()))),
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

		Err(parse_error!(ctor, CantCreateExpression(super::Error::MissingClosingParen(paren).into())))
	}
}


impl Block {
	/// Calling a block will create a new stackframe, and then execute the contents of the block
	/// within that stack frame.
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

	/// Converting a block to a string simply returns a spaced out representation of its tokens.
	#[inline]
	pub fn qs_at_text(&self, _: Args) -> quest_core::Result<Object> {
		Ok(self.to_string().into())
	}
}

impl_object_type!{
for Block [(parents quest_core::types::Function)]:
	"@text" => method_old Block::qs_at_text,
	"()" => function Block::qs_call
}

#[cfg(test)]
mod tests {
	

	#[test]
	#[ignore]
	fn call() { todo!(); }
}









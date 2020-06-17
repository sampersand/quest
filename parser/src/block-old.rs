
use crate::{Expression, ParenType};
#[macro_use]
use quest::impl_object_type;
use quest::{Object, Result, Args, types::{self, rustfn::Binding}};
use std::fmt::{self, Debug, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Line {
	Multiple(Vec<Expression>),
	Singular(Expression)
}

#[derive(Clone, PartialEq, Eq)]
pub struct Block {
	paren: ParenType,
	body: Vec<Line>,
	returns: bool,
}

impl Debug for Block {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			f.debug_struct("Block")
				.field("paren", &self.paren)
				.field("body", &self.body)
				.field("returns", &self.returns)
				.finish()
		} else {
			f.debug_tuple("Block")
				.field(&format!("[{} line(s)]", self.body.len()))
				.finish()
		}
	}
}

impl Line {
	fn execute(&self) -> Result<Object> {
		match self {
			Line::Singular(line) => line.execute(),
			Line::Multiple(args) => args.iter()
				.map(|arg| arg.execute())
				.collect::<Result<Vec<_>>>()
				.map(|args| types::List::from(args).into())
		}
	}
}

impl Block {
	pub fn new(paren: ParenType, body: Vec<Line>, returns: bool) -> Self {
		Block { paren, body, returns }
	}

	pub fn paren(&self) -> ParenType {
		self.paren
	}

	fn run_block(&self) -> Result<Option<Object>> {
		if let Some(last) = self.body.last() {
			for line in &self.body[..self.body.len() - 1] {
				line.execute()?;
			}

			let ret = last.execute()?;
			if self.returns {
				return Ok(Some(ret))
			}
		}

		Ok(None)
	}

	fn call(&self, args: Args) -> Result<Object> {
		Binding::new_stackframe(args, (|_binding| {
			self.run_block().map(Option::unwrap_or_default)
		}))
	}

	pub fn execute(&self) -> Result<Option<Object>> {
		match self.paren {
			ParenType::Paren => self.run_block().map(|x| {
				// let x = x.a.unwrap_or_default();
				if self.returns {
					x
				}  else {
					None
				}
				// if self.returns { Some(x) } else { None }
			}),

			ParenType::Bracket => self.run_block().map(|x| {
				let x = x.unwrap_or_else(|| vec![].into());
				if self.returns { Some(x) } else { None }
			}),

			ParenType::Brace => {
				let block = Object::from(self.clone());
				block.add_parent(Binding::instance().as_ref().clone())?;
				Ok(Some(block))
			},
			// ParenType::Bracket => todo!("ParenType::Bracket return value."),
		}
	}
}

mod impls {
	use super::*;

	pub fn call(mut args: Args) -> Result<Object> {
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




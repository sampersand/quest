use crate::parse::{Expression, ParenType};
use crate::obj::{Object, Result, Args, types::{self, rustfn::Binding}};
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
		f.debug_struct("Block")
			.field("paren", &self.paren)
			.field("body", &format!("[{} line(s)]", self.body.len()))
			.field("returns", &self.returns)
			.finish()
	}
}

impl Line {
	fn execute(&self, binding: &Binding) -> Result<Object> {
		match self {
			Line::Singular(line) => line.execute(binding),
			Line::Multiple(args) => args.iter()
				.map(|arg| arg.execute(binding))
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

	fn call(&self, args: &Args, child: bool) -> Result<Object> {
		let ref child = if child { args.child_binding()? } else { args.binding().clone() };

		if let Some(last) = self.body.last() {
			for line in &self.body[..self.body.len() - 1] {
				line.execute(child)?;
			}
			let ret = last.execute(child)?;
			if self.returns {
				return Ok(ret)
			}
		}

		Ok(Object::default())
	}

	pub fn execute(&self, binding: &Binding) -> Result<Option<Object>> {
		let ret = match self.paren {
			ParenType::Paren => self.call(&Args::new_slice(&[], binding.clone()), false)?,
			ParenType::Brace => return Ok(Some(self.clone().into())),
			ParenType::Bracket => todo!("ParenType::Bracket return value."),
		};

		if self.returns {
			Ok(Some(ret))
		} else {
			Ok(None)
		}
	}
}

mod impls {
	use super::*;

	pub fn call(args: Args) -> Result<Object> {
		args.this_downcast_ref::<Block>()?.call(&args.args(..)?, true)
	}
}



impl_object_type!{
for Block [(parent super::Function)]:
	"()" => impls::call
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[ignore]
	fn call() { todo!(); }
}






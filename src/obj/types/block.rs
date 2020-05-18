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

	fn call(&self, this: Option<Object>, args: &Args) -> Result<Object> {
		let ref child = if let Some(mut this) = this {
			this.set_attr(
				"__args__".into(),
				types::List::from(args.clone()).into(),
				&args.binding()
			)?;
			this
		} else {
			args.child_binding()?
		};

		if let Some(last) = self.body.last() {
			for line in &self.body[..self.body.len() - 1] {
				line.execute(child)?;
			}
			last.execute(child)
		} else {
			Ok(Object::default())
		}
	}

	pub fn execute(&self, binding: &Binding) -> Result<Option<Object>> {
		let ret = match self.paren {
			ParenType::Paren 
			// not sure if we want to keep bracket here...
				| ParenType::Bracket => self.call(None, &Args::new_slice(&[], binding.clone()))?,
			ParenType::Brace => return Ok(Some(self.clone().into())),
			// ParenType::Bracket => todo!("ParenType::Bracket return value."),
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
		let bound_object = args.this()?.get_attr(&"__bound_object__".into(), args.binding()).ok();

		args.this_downcast_ref::<Block>()?.call(bound_object, &args.args(..)?)
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






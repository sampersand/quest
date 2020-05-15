use crate::parse::{Expression, ParenType};
use crate::obj::{Object, Result, types::{self, rustfn::Binding}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Line {
	Multiple(Vec<Expression>),
	Singular(Expression)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
	paren: ParenType,
	body: Vec<Line>,
	returns: bool
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

	fn call(&self, binding: &Binding) -> Result<Object> {
		if let Some(last) = self.body.last() {
			for line in &self.body {
				line.execute(binding)?;
			}
			last.execute(binding)
		} else {
			Ok(Object::default())
		}
	}

	pub fn execute(&self, binding: &Binding) -> Result<Option<Object>> {
		let ret = match self.paren {
			ParenType::Paren => self.call(binding)?,
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

impl_object_type!{for Block, super::Function;
	"()" => (|args| {
		// do something here do make a new binding with the specified args.
		args.this::<Block>()?.call(args.binding())
	}),

}




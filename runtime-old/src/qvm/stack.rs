use crate::Value;

#[derive(Debug)]
pub struct Stack(Vec<Value>);

impl Stack {
	pub const fn new() -> Self {
		Self(Vec::new())
	}

	pub fn push(&mut self, value: Value) {
		self.0.push(value)
	}

	pub fn pop(&mut self) -> Option<Value> {
		self.0.pop()
	}
}

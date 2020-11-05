use crate::Value;
use std::cmp::Ordering;

#[derive(Debug, Clone, Default)]
pub struct Register(Value);

impl Register {
	pub const ZERO: Self = Self::new(Value::ZERO);

	pub const fn new(value: Value) -> Self {
		Self(value)
	}

	pub fn store(&mut self, value: Value) {
		self.0 = value;
	}

	pub fn load(&self) -> &Value {
		&self.0
	}

	pub fn take(&mut self) -> Value {
		std::mem::replace(&mut self.0, Self::ZERO.0)
	}

	pub fn cmp(&self) -> Ordering {
		todo!()
	}
}

use crate::{Literal, Value};
use std::collections::HashMap;

const NUM_BUCKETS: usize = u8::MAX as usize;

pub struct Attributes {
	builtin: [&'static [(Literal, Value)]; NUM_BUCKETS],
	user_defined: HashMap<Literal, Value>
}

impl Attributes {
	fn new() -> Self { todo!() }

	pub fn has_attr(&self, attr: Literal) -> bool {
		todo!()
	}

	pub fn get_attr(&self, attr: Literal) -> Option<Value> {
		todo!()
	}

	pub fn del_attr(&self, attr: Literal) -> Option<Value> {
		todo!()
	}

	pub fn set_attr(&self, attr: Literal, value: Value) {
		todo!()
	}
}

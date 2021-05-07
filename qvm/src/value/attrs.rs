use crate::{Literal, Value};
use std::collections::HashMap;
use parking_lot::RwLock;

const NUM_BUCKETS: usize = u8::MAX as usize;
type LiteralHasher = std::collections::hash_map::RandomState;

// struct Attrs {
// 	lock: RwLock<()>,
// 	parents: Vec<Value>,
// 	user_defined: HashMap<Literal, Value, LiteralHasher>,
// 	cached: [(Literal, Value); , LiteralHasher>,
// }

pub struct Attributes {
	parents: Vec<Value>,
	builtin: [&'static [(Literal, Value)]; NUM_BUCKETS],
	user_defined: RwLock<HashMap<Literal, Value, LiteralHasher>>
}


fn hash(attr: Literal) -> u64 {
	use std::hash::{BuildHasher, Hasher};
	let mut hasher = LiteralHasher::new().build_hasher();

	hasher.write_u32(attr.bits());

	hasher.finish()
}

impl Attributes {
	fn new() -> Self { todo!() }

	pub fn has_attr(&self, attr: Literal) -> bool {
		if attr.is_builtin() {
			for &(literal, value) in self.builtin[hash(attr) as usize] {
				if literal == attr {
					return true;
				}
			}

			// if it's not here, it may be in the `user_defined` attrs.
		}

		self.user_defined.read().contains_key(&attr)
	}

	pub fn get_attr(&self, attr: Literal) -> Option<Value> {
		if attr.is_builtin() {
			for &(literal, value) in self.builtin[hash(attr) as usize] {
				if literal == attr {
					return Some(value);
				}
			}

			// if it's not here, it may be in the `user_defined` attrs.
		}

		self.user_defined.read().get(&attr).cloned()
	}

	pub fn del_attr(&self, attr: Literal) -> Option<Value> {
		if attr.is_builtin() {
			for &(literal, value) in self.builtin[hash(attr) as usize] {
				if literal == attr {
					return Some(value);
				}
			}

			// if it's not here, it may be in the `user_defined` attrs.
		}

		self.user_defined.read().get(&attr).cloned()
	}

	pub fn set_attr(&self, attr: Literal, value: Value) {
		todo!()
	}
}

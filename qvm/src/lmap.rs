use crate::{Literal, Value};
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};

/// A [`Literal`] map.
#[derive(Default, Clone)]
pub struct LMap {
	map: HashMap<Literal, Value>
}

impl Debug for LMap {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_map().entries(&self.map).finish()
	}
}

impl LMap {
	#[inline]
	pub fn new() -> Self {
		Self::default()
	}

	#[inline]
	pub fn has(&self, key: Literal) -> bool {
		self.map.contains_key(&key)
	}

	#[inline]
	pub fn get(&self, key: Literal) -> Option<&Value> {
		self.map.get(&key)
	}

	#[inline]
	pub fn get_mut(&mut self, key: Literal) -> Option<&mut Value> {
		self.map.get_mut(&key)
	}

	#[inline]
	pub fn set(&mut self, key: Literal, value: Value) {
		self.map.insert(key, value);
	}

	#[inline]
	pub fn del(&mut self, key: Literal) -> Option<Value> {
		self.map.remove(&key)
	}
}

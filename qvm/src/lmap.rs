use crate::{Literal, Value};
use std::collections::HashMap;

/// A [`Literal`] map.
#[derive(Debug, Default, Clone)]
pub struct LMap {
	map: HashMap<Literal, Value>
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

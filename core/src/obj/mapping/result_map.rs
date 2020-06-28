use crate::{Result, obj::EqKey};
use super::*;
use std::fmt::{self, Debug, Formatter};

#[derive(Clone)]
pub struct ResultMap(Vec<(Key, Value)>);

impl Default for ResultMap {
	fn default() -> Self {
		Self::new()
	}
}

impl Debug for ResultMap {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_map()
			.entries(self.0.iter().map(|&(ref k, ref v)| (k, v)))
			.finish()
	}
}


impl ResultMap  {
	pub fn new() -> Self {
		ResultMap(vec![])
	}

	pub fn keys(&self) -> Vec<&Key> {
		self.0.iter().map(|(f, _)| f).collect()
	}

	pub fn has<K: EqKey + ?Sized>(&self, key: &K) -> Result<bool> {
		for (ref k, _) in self.0.iter() {
			if key.eq_key(k)? {
				return Ok(true);
			}
		}

		Ok(false)
	}

	pub fn insert(&mut self, key: Key, value: Value) -> Result<()> {
		for (ref k, ref mut v) in self.0.iter_mut() {
			if key.eq_key(k)? {
				*v = value;
				return Ok(());
			}
		}

		self.0.push((key, value));
		Ok(())
	}

	pub fn get<K: EqKey + ?Sized>(&self, key: &K) -> Result<Option<&Value>> {
		for (ref k, ref v) in self.0.iter() {
			if key.eq_key(k)? {
				return Ok(Some(v));
			}
		}
		Ok(None)
	}

	pub fn remove<K: EqKey + ?Sized>(&mut self, key: &K) -> Result<Option<Value>> {
		let mut index = None;
		for (i, (ref k, _)) in self.0.iter().enumerate() {
			if key.eq_key(k)? {
				index = Some(i);
				break;
			}
		}

		if let Some(index) = index {
			Ok(Some(self.0.remove(index).1))
		}  else {
			Ok(None)
		}
	}
}
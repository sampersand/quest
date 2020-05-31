use crate::obj::{Object, Result, EqResult, mapping::Key};
use std::borrow::Borrow;
use std::fmt::{self, Debug, Formatter};

type Value = Object;

#[derive(Clone, Default)]
pub struct ObjectMap(Vec<(Key, Value)>);

impl Debug for ObjectMap {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_map()
			.entries(self.0.iter().map(|&(ref k, ref v)| (k, v)))
			.finish()
	}
}

impl ObjectMap  {
	pub fn insert(&mut self, key: Key, val: Value) -> Result<Option<Object>> {
		for (ref k, ref mut v) in self.0.iter_mut() {
			if k.equals(&key)? {
				return Ok(Some(std::mem::replace(v, val)));
			}
		}

		self.0.push((key.into(), val));
		Ok(None)
	}

	pub fn get<K>(&self, key: &K) -> Result<Option<Value>>
	where K: Debug + ?Sized, Key: EqResult<K> {
		for (ref k, ref v) in self.0.iter() {
			if k.equals(key)? {
				return Ok(Some(v.clone()));
			}
		}

		Ok(None)
	}
}
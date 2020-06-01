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

	pub fn remove<K>(&mut self, key: &K) -> Result<Option<Value>>
	where K: Debug + ?Sized + EqResult<Key> {
		let mut index = None;
		for (i, (ref k, _)) in self.0.iter().enumerate() {
			if key.equals(k)? {
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

	pub fn get<K>(&self, key: &K) -> Result<Option<Value>>
	where K: Debug + ?Sized + EqResult<Key> {
		for (ref k, ref v) in self.0.iter() {
			if key.equals(k)? {
				return Ok(Some(v.clone()));
			}
		}

		Ok(None)
	}
}
use crate::obj::{Object, Result, mapping::Key, Binding};

type Value = Object;

#[derive(Debug, Clone, Default)]
pub struct ObjectMap(Vec<(Key, Value)>);

impl ObjectMap  {
	pub fn insert(&mut self, key: Key, val: Value, binding: &Binding) -> Result<Option<Object>> {
		for (ref k, ref mut v) in self.0.iter_mut() {
			if k.equals(&key, binding)? {
				return Ok(Some(std::mem::replace(v, val)));
			}
		}

		self.0.push((key.into(), val));
		Ok(None)
	}

	pub fn get(&self, key: &Key, binding: &Binding) -> Result<Option<Value>> {
		for (ref k, ref v) in self.0.iter() {
			if k.equals(&key, binding)
		}
	}
}
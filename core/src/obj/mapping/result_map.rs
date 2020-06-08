use crate::{Result, EqResult};
use std::fmt::{self, Debug, Formatter};

#[derive(Clone)]
pub struct ResultMap<K, V>(Vec<(K, V)>);

impl<K, V> Default for ResultMap<K, V> {
	fn default() -> Self {
		ResultMap::new()
	}
}

impl<K: Debug, V: Debug> Debug for ResultMap<K, V> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_map()
			.entries(self.0.iter().map(|&(ref k, ref v)| (k, v)))
			.finish()
	}
}

impl<K, V> ResultMap<K, V>  {
	pub fn new() -> Self {
		ResultMap(vec![])
	}
}

impl<K, V> ResultMap<K, V>  {
	pub fn insert<Q: ?Sized>(&mut self, key: Q, value: V) -> Result<Option<V>>
	where
		Q: EqResult<K> + Into<K>
	{
		for (ref k, ref mut v) in self.0.iter_mut() {
			if key.equals(&k)? {
				return Ok(Some(std::mem::replace(v, value)));
			}
		}
		self.0.push((key.into(), value));
		Ok(None)
	}

	pub fn get<Q: ?Sized>(&self, key: &Q) -> Result<Option<&V>>
	where
		Q: EqResult<K>
	{
		for (ref k, ref v) in self.0.iter() {
			if key.equals(k)? {
				return Ok(Some(v));
			}
		}
		Ok(None)
	}

	pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Result<Option<V>>
	where
		Q: EqResult<K>
	{
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
}
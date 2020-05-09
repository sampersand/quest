use crate::obj::{Object, Result};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};

// this is totally hacky, and shouldbe replaced with something better in the future.
#[derive(Clone)]
pub struct Mapping {
	map: Vec<(Object, Object)>,
	parent: Option<Arc<RwLock<Mapping>>>
}

impl Debug for Mapping {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_map()
			.entries(self.map.iter().map(|&(ref k, ref v)| (k, v)))
			.finish()
	}
}

impl Mapping {
	pub fn new(parent: Option<Arc<RwLock<Mapping>>>) -> Self {
		Mapping { map: Default::default(), parent }
	}

	pub fn insert(&mut self, key: Object, val: Object) -> Result {
		// todo: this doesn't take into account whether or not `key` exists already...
		self.map.push((key, val.clone()));
		Ok(val)
		// m.insert(
		// 	super::Text::new("+").into_object(),
		// 	super::Text::new("+").into_object());
	}

	pub fn get(&self, key: &Object, obj: &Object) -> Result {
		for (ref k, ref v) in self.map.iter() {
			// do we always want to return on errors from `call_into_bool?`
			if k.call("==", &[key])?.try_into_bool()? {
				return Ok(v.clone());
			}
		}

		if let Some(ref parent) = self.parent {
			parent.read().expect("read error").get(key, obj)
		} else {
			Err(format!("key {:?} doesnt' exist for {:?}", key, obj).into())
		}

	}

}





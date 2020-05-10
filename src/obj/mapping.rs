use crate::obj::{Object, Result};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};

// this is totally hacky, and shouldbe replaced with something better in the future.
#[derive(Clone, Default)]
pub struct Mapping {
	map: Vec<(Object, Object)>,
	parent: Option<Object>
}

impl Debug for Mapping {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		struct Map<'a>(&'a [(Object, Object)]);
		impl Debug for Map<'_> {
			fn fmt(&self, f: &mut Formatter) -> fmt::Result {
				f.debug_map()
					.entries(self.0.iter().map(|&(ref k, ref v)| (k, v)))
					.finish()
			}
		}

		struct Parent(bool);
		impl Debug for Parent {
			fn fmt(&self, f: &mut Formatter) -> fmt::Result {
				if self.0 {
					write!(f, "Some({{ ... }})")
				} else {
					write!(f, "None")
				}
			}
		}

		f.debug_struct("Mapping")
			.field("map", &Map(self.map.as_slice()))
			.field("parent", &Parent(self.parent.is_some()))
			.finish()
	}
}

impl Mapping {
	pub fn new(parent: Option<Object>) -> Self {
		Mapping { map: Default::default(), parent }
	}

	pub fn insert(&mut self, attr: Object, val: Object) -> Result {
		if attr.call("==", &[&"__parent__".into()])?.into_bool().unwrap_or(false) {
			self.parent = Some(val.clone());
			return Ok(val);
		}

		for (ref k, ref mut v) in self.map.iter_mut() {
			if k.call("==", &[&attr])?.try_into_bool()? {
				*v = val.clone();
				return Ok(val);
			}
		}

		self.map.push((attr, val.clone()));
		Ok(val)
	}

	pub fn get(&self, attr: &Object) -> Result {
		if attr.call("==", &[&"__parent__".into()])?.into_bool().unwrap_or(false) {
			return self.parent.clone().ok_or_else(|| "attr `__parent__` doesn't exist.".into());
		}

		for (ref k, ref v) in self.map.iter() {
			if attr.call("==", &[k])?.try_into_bool()? {
				return Ok(v.clone());
			}
		}

		if let Some(ref parent) = self.parent {
			parent.get_attr(attr)
		} else {
			Err(format!("attr {:?} does not exist.", attr).into())
		}
	}

	pub fn remove(&mut self, attr: &Object) -> Result {
		if attr.call("==", &[&"__parent__".into()])?.into_bool().unwrap_or(false) {
			return self.parent.take().ok_or_else(|| "attr `__parent__` doesn't exist.".into());
		}

		// let mut idx: usize;
		for (idx, (ref k, ref v)) in self.map.iter().enumerate() {
			if k.call("==", &[attr])?.try_into_bool()? {
				return Ok(self.map.remove(idx).1);
			}
		}

		Err(format!("attr {:?} does not exist.", attr).into())
	}

}








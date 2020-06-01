use crate::obj::{self, Object, EqResult, Args, types};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};

mod key;
mod object_map;
pub use self::key::Key;
use self::object_map::ObjectMap;

// this is totally hacky, and shouldbe replaced with something better in the future.
#[derive(Clone, Default)]
pub struct Mapping {
	map: ObjectMap,
	parent: Option<Object>
}

impl Debug for Mapping {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
			.field("map", &self.map)
			.field("parent", &Parent(self.parent.is_some()))
			.finish()
	}
}


const PARENT_KEY: Key = Key::Literal("__parent__");
const ID_KEY: Key = Key::Literal("__id__");

impl Mapping {
	pub fn new(parent: Option<Object>) -> Self {
		Mapping { map: Default::default(), parent }
	}

	pub fn insert(&mut self, attr: Key, val: Object) -> obj::Result<Object> {
		if attr.equals(&PARENT_KEY)? {
			self.parent = Some(val.clone());
		} else {
			self.map.insert(attr.into(), val.clone())?;
		}

		Ok(val)
	}

	pub fn has<K>(&self, attr: &K, obj: &Object) -> bool
	where K: Debug + ?Sized + EqResult<Key> {
		self.get(attr, obj).is_ok()
	}

	pub fn get<K>(&self, attr: &K, obj: &Object) -> obj::Result<Object>
	where K: Debug + ?Sized + EqResult<Key> {
		if attr.equals(&PARENT_KEY)? {
			self.parent.clone().ok_or_else(|| "attr `__parent__` doesn't exist.".into())
		} else if attr.equals(&ID_KEY)? {
			Ok(obj.id().into())
		} else if let Some(obj) = self.map.get(attr)? {
			Ok(obj)
		} else {
			if let Some(ref parent) = self.parent {
				if let Ok(val) = parent.get_attr(attr) {
					return Ok(val);
				}
			}
			Err(format!("attr {:?} does not exist for {:?}.", attr, obj).into())
		}
	}

	pub fn remove<K>(&mut self, attr: &K, obj: &Object) -> obj::Result<Object>
	where K: Debug + ?Sized + EqResult<Key> {
		if attr.equals(&PARENT_KEY)? {
			return Ok(self.parent.take().unwrap_or_default());
		} else if let Some(old) = self.map.remove(attr)? {
			Ok(old)
		} else {
			Err(format!("attr {:?} does not exist for {:?}.", attr, obj).into())
		}
	}
}





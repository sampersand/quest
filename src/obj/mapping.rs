use crate::obj::{self, Object, traits::*, Args, types};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};

mod key;
mod value;
mod result_map;
pub use self::value::Value;
pub use self::key::Key;
use self::result_map::ResultMap;

// this is totally hacky, and shouldbe replaced with something better in the future.
#[derive(Clone, Default)]
pub struct Mapping {
	map: ResultMap<Key, Value>,
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

	pub fn insert<K: ?Sized, V>(&mut self, key: K, value: V) -> obj::Result<Object>
	where
		K: EqResult<Key> + Into<Key>, V: Into<Value>
	{
		let value = value.into();

		if key.equals(&PARENT_KEY)? {
			let value = Object::from(value);
			self.parent = Some(value.clone());
			return Ok(value);
		}

		match self.map.insert(key, value) {
			Ok(None) => Ok(Object::default()),
			Ok(Some(value)) => Ok(value.into()),
			Err(err) => Err(err)
		}
	}

	pub fn has<K>(&self, key: &K) -> bool
	where
		K: Debug + ?Sized + EqResult<Key>
	{
		self.get(key).is_ok()
	}

	fn get_special_key<K>(&self, key: &K) -> obj::Result<Option<Object>>
	where
		K: Debug + ?Sized + EqResult<Key>
	{
		if key.equals(&PARENT_KEY)? {
			Ok(self.parent.clone())
		} else if key.equals(&ID_KEY)? {
			unimplemented!()
		} else {
			Ok(None)
		}
	}

	pub fn get<K>(&self, key: &K) -> obj::Result<Object>
	where
		K: Debug + ?Sized + EqResult<Key>
	{

		if let Some(value) = self.get_special_key(key)? {
			Ok(value)
		} else if let Some(value) = self.map.get(key)? {
			Ok(value.clone().into())
		} else {
			if let Some(ref parent) = self.parent {
				if let Ok(val) = parent.get_attr(key) {
					return Ok(val);
				}
			}

			if let Some(mixins) = self.map.get("__mixins__")? {
				let mixins = Object::from(mixins.clone());
				for mixin in mixins.downcast_call::<types::List>()?.as_ref() {
					if let Some(val) = mixin.get_attr(key).ok() {
						return Ok(val);
					}
				}
			}
			Err(format!("attr {:?} does not exist", key).into())
		}
	}

	pub fn remove<K: ?Sized>(&mut self, key: &K) -> obj::Result<Object>
	where
		K: Debug + EqResult<Key>
	{
		if key.equals(&PARENT_KEY)? {
			return Ok(self.parent.take().unwrap_or_default());
		}

		self.map.remove(key)?
			.map(|value| value.into())
			.ok_or_else(|| format!("attr {:?} does not exist for.", key).into())
	}
}


use crate::{Object, Result, Error};
use std::fmt::Debug;
use crate::literals::{PARENTS, ID, ATTR_MISSING};

mod key;
mod value;
mod parents;
mod result_map;
use result_map::ResultMap;
pub use value::Value;
pub use key::{EqKey, Key};
pub use parents::Parents;

// this is totally hacky, and shouldbe replaced with something better in the future.
#[derive(Clone, Debug, Default)]
pub struct Mapping {
	map: ResultMap,
	pub(super) obj: std::sync::Weak<super::Internal>,
	parents: Parents
}

impl Mapping {
	pub fn new<P: Into<Parents>>(parents: P) -> Self {
		Mapping {
			map: Default::default(),
			obj: Default::default(),
			parents: parents.into()
		}
	}

	fn owner(&self) -> Result<Object> {
		self.obj.upgrade()
			.ok_or_else(|| Error::Internal("`obj` doesnt exist?"))
			.map(Object)
	}


	#[inline]
	pub fn add_parent(&mut self, parent: Object) -> Result<()> {
		self.parents.add_parent(parent)
	}

	pub fn keys(&self) -> Vec<Key> {
		let mut keys = self.map.keys()
			.into_iter()
			.map(|k| k.clone())
			.collect::<Vec<_>>();

		keys.push(PARENTS);
		keys.push(ID);
		keys
	}

	pub fn insert(&mut self, key: Key, value: Value) -> Result<()> {
		// this is a proble, as we might get a RWLOCK error if the key is a piece of text or something.
		// example code:
		// Text.12 = {};
		// Text."14" = {}; # => crashes
		if key.eq_key(&PARENTS)? {
			self.parents = Object::from(value).into();
			return Ok(());
		}

		self.map.insert(key, value)
	}

	pub fn has<K: EqKey + ?Sized>(&self, key: &K) -> Result<bool> {
		if key.eq_key(&PARENTS)? || key.eq_key(&ID)? {
			return Ok(true);
		}

		self.map.has(key)
	}

	pub fn get<K: EqKey + ?Sized>(&self, key: &K) -> Result<Option<Value>> {
		if key.eq_key(&PARENTS)? {
			return Ok(Some(self.parents.as_object().into()));
		}

		if key.eq_key(&ID)? {
			return Ok(Some(Object::from(self.owner()?.0.id).into()));
		}

		if let Some(val) = self.map.get(key)? {
			return Ok(Some(val.clone()));
		}

		if self.map.has(&ATTR_MISSING)? {
			// self.call_attr("__attr_missing__", &[attr.to_object()])
			unimplemented!("how do we handle attr missing and parents?")
		}

		for parent in self.parents.iter()? {
			if let Some(obj) = parent.0.mapping.read().expect("cant read").get(key)? {
				return Ok(Some(obj))
			}
		}

		Ok(None)
	}

	pub fn remove<K: EqKey + ?Sized>(&mut self, key: &K) -> Result<Option<Object>> {
		if key.eq_key(&PARENTS)? {
			return Ok(Some(std::mem::take(&mut self.parents).as_object()));
		}

		self.map.remove(key).map(|val_opt| val_opt.map(Object::from))
	}
}



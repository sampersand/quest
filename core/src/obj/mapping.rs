use crate::{Object, Result, Error, EqResult};
use std::fmt::Debug;

mod key;
mod value;
mod parents;
mod result_map;
pub use value::Value;
pub use key::Key;
pub use parents::Parents;
use self::result_map::ResultMap;

// this is totally hacky, and shouldbe replaced with something better in the future.
#[derive(Clone, Debug, Default)]
pub struct Mapping {
	map: ResultMap<Key, Value>,
	pub(super) obj: std::sync::Weak<super::Internal>,
	parents: Parents
}

const PARENTS_KEY: Key = Key::Literal("__parents__");
const ID_KEY: Key = Key::Literal("__id__");

impl Mapping {
	pub fn new<P: Into<Parents>>(parents: P) -> Self {
		Mapping {
			map: Default::default(),
			obj: Default::default(),
			parents: parents.into()
		}
	}

	pub fn add_parent(&mut self, parent: Object) -> Result<()> {
		self.parents.add_parent(parent)
	}
}

impl Mapping {
	pub fn insert<K: ?Sized, V>(&mut self, key: K, value: V) -> Result<Object>
	where
		K: EqResult<Key> + Into<Key>,
		V: Into<Value> + Into<Parents>
	{
		if key.equals(&PARENTS_KEY)? {
			self.parents = value.into();
			Ok(self.parents.as_object())
		} else {
			self.insert_not_parents(key, value)
		}
	}

	pub fn insert_not_parents<K: ?Sized, V>(&mut self, key: K, value: V) -> Result<Object>
	where
		K: EqResult<Key> + Into<Key>,
		V: Into<Value>
	{
		assert!(!key.equals(&PARENTS_KEY).unwrap(), "can't call insert_not_parents with PARENTS_KEY");
		match self.map.insert(key, value.into()) {
			Ok(None) => Ok(Object::default()),
			Ok(Some(value)) => Ok(value.into()),
			Err(err) => Err(err)
		}
	}



	pub fn has<K>(&self, key: &K) -> Result<bool>
	where
		K: Debug + ?Sized + EqResult<Key>
	{
		self.get(key).map(|x| x.is_some())
	}

	fn get_special_key<K>(&self, key: &K) -> Result<Option<Object>>
	where
		K: Debug + ?Sized + EqResult<Key>
	{
		if key.equals(&PARENTS_KEY)? {
			Ok(Some(self.parents.as_object()))
		} else if key.equals(&ID_KEY)? {
			self.obj.upgrade()
				.ok_or_else(|| Error::Internal("`obj` doesnt exist?"))
				.map(|obj| Object::from(obj.id))
				.map(Some)
		} else {
			Ok(None)
		}
	}

	pub fn get<K>(&self, key: &K) -> Result<Option<Object>>
	where
		K: Debug + ?Sized + EqResult<Key>
	{

		if let Some(value) = self.get_special_key(key)? {
			Ok(Some(value))
		} else if let Some(value) = self.map.get(key)? {
			Ok(Some(value.clone().into()))
		} else {
			for parent in self.parents.iter()? {
				if let Some(obj) = parent.0.mapping.read().expect("cant read").get(key)? {
					return Ok(Some(obj))
				}
			}

			Ok(None)
		}
	}

	pub fn remove<K: ?Sized>(&mut self, key: &K) -> Result<Option<Object>>
	where
		K: Debug + EqResult<Key>
	{
		if key.equals(&PARENTS_KEY)? {
			return Ok(Some(std::mem::take(&mut self.parents).as_object()));
		}

		self.map.remove(key).map(|val_opt| val_opt.map(Into::into))
	}
}

use crate::{Object, Result};
use super::Value;
use std::hash::Hash;
use std::borrow::Borrow;
use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub enum Parents {
	None,
	Builtin(Vec<Object>),
	Object(Object)
}

impl Default for Parents {
	#[inline]
	fn default() -> Self {
		Parents::None
	}
}

impl From<Value> for Parents {
	fn from(value: Value) -> Self {
		Parents::Object(value.into())
	}
}

impl From<Parents> for Value {
	fn from(parents: Parents) -> Self {
		Value::from(Object::from(parents))
	}
}

impl From<Parents> for Object {
	fn from(parents: Parents) -> Self {
		match parents {
			Parents::None => Object::default(),
			Parents::Builtin(vec) => vec.into(),
			Parents::Object(obj) => obj
		}
	}
}

impl From<Vec<Object>> for Parents {
	#[inline]
	fn from(vec: Vec<Object>) -> Self {
		Parents::Builtin(vec)
	}
}

impl From<()> for Parents {
	#[inline]
	fn from(_: ()) -> Self {
		Parents::None
	}
}

impl From<Object> for Parents {
	#[inline]
	fn from(obj: Object) -> Self {
		Parents::Object(obj)
	}
}

impl FromIterator<Object> for Parents {
	fn from_iter<I: IntoIterator<Item=Object>>(iter: I) -> Self {
		Self::from(iter.into_iter().collect::<Vec<Object>>())
	}
}

impl Parents {
	pub fn add_parent(&mut self, parent: Object) -> Result<()> {
		match self {
			Parents::None => *self = Parents::Builtin(vec![parent]),
			Parents::Builtin(vec) => vec.push(parent),
			Parents::Object(obj) => { obj.call_attr_lit("push", &[&parent])?; },
		}

		Ok(())
	}
	pub fn to_object(&self) -> Object {
		match self {
			Parents::None => Object::default(),
			Parents::Builtin(vec) => vec.clone().into(),
			Parents::Object(obj) => obj.clone()
		}
	}

	fn with_iter<F: FnOnce(std::slice::Iter<'_, Object>) -> Result<R>, R>(&self, f: F) -> Result<R> {
		match self {
			Parents::None => f([].iter()),
			Parents::Builtin(parents) => f(parents.iter()),
			Parents::Object(object) => 
				object.downcast_call::<crate::types::List>().and_then(|list| f(list.iter()))
		}
	}

	pub fn keys(&self) -> Result<Vec<Object>> {
		self.with_iter(|iter| Ok(iter.map(|x| x.clone()).collect()))
	}

	pub fn has_lit<K: Hash + Eq + ?Sized>(&self, key: &K) -> Result<bool>
	where
		for <'a> &'a str: Borrow<K>
	{
		self.with_iter(|iter| {
			for parent in iter {
				if parent.has_attr_lit(key)? {
					return Ok(true)
				}
			}
			Ok(false)
		})
	}

	pub fn get_lit<K: Hash + Eq + ?Sized>(&self, key: &K) -> Result<Option<Value>>
	where
		for <'a> &'a str: Borrow<K>
	{
		self.with_iter(|iter| {
			for parent in iter {
				if let Some(value) = parent.get_value_lit(key)? {
					return Ok(Some(value))
				}
			}
			Ok(None)
		})
	}

	pub fn has_obj(&self, key: &Object) -> Result<bool> {
		self.with_iter(|iter| {
			for parent in iter {
				if parent.has_attr(key)? {
					return Ok(true)
				}
			}
			Ok(false)
		})
	}

	pub fn get_obj(&self, key: &Object) -> Result<Option<Value>> {
		self.with_iter(|iter| {
			for parent in iter {
				if let Some(value) = parent.get_value(key)? {
					return Ok(Some(value))
				}
			}
			Ok(None)
		})
	}
}

// impl IntoIterator for Parents {
// 	type Item = Object;
// 	type IntoIter = Vec<Object> as Iterator;
// 	fn into_iter(self) -> Self::IntoIter {
// 		Vec::from(self).into()
// 	}
// }



















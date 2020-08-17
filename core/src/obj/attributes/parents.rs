use crate::{Object, Result, Literal};
use crate::types::List;
use super::Value;
use std::iter::FromIterator;
use parking_lot::RwLock;
use std::hash::Hash;
use std::borrow::Borrow;

#[derive(Debug)]
pub struct Parents(RwLock<Inner>);

#[derive(Debug, Clone)]
enum Inner {
	None,
	Builtin(Vec<Object>),
	Object(Object)
}

impl Clone for Parents {
	#[inline]
	fn clone(&self) -> Self {
		Self::from_inner(self.0.read().clone())
	}
}

impl Parents {
	#[inline]
	fn from_inner(inner: Inner) -> Self {
		Parents(RwLock::new(inner))
	}
}

impl Default for Parents {
	#[inline]
	fn default() -> Self {
		Self::from_inner(Inner::None)
	}
}

impl From<Value> for Parents {
	#[inline]
	fn from(value: Value) -> Self {
		Self::from_inner(Inner::Object(value.into()))
	}
}

impl From<Parents> for Value {
	#[inline]
	fn from(parents: Parents) -> Self {
		Value::from(Object::from(parents))
	}
}

impl From<Parents> for Object {
	fn from(parents: Parents) -> Self {
		match parents.0.into_inner(){
			Inner::None => Object::default(),
			Inner::Builtin(vec) => vec.into(),
			Inner::Object(obj) => obj
		}
	}
}

impl From<Vec<Object>> for Parents {
	#[inline]
	fn from(vec: Vec<Object>) -> Self {
		Self::from_inner(Inner::Builtin(vec))
	}
}

impl From<Vec<&'static Object>> for Parents {
	#[inline]
	fn from(vec: Vec<&'static Object>) -> Self {
		Self::from_inner(Inner::Builtin(vec.into_iter().map(Object::clone).collect()))
	}
}

impl From<()> for Parents {
	#[inline]
	fn from(_: ()) -> Self {
		Self::from_inner(Inner::None)
	}
}

impl From<Object> for Parents {
	#[inline]
	fn from(obj: Object) -> Self {
		Self::from_inner(Inner::Object(obj))
	}
}

impl From<&'static Object> for Parents {
	#[inline]
	fn from(obj: &'static Object) -> Self {
		Self::from_inner(Inner::Object(obj.clone()))
	}
}

impl FromIterator<Object> for Parents {
	fn from_iter<I: IntoIterator<Item=Object>>(iter: I) -> Self {
		Self::from(iter.into_iter().collect::<Vec<Object>>())
	}
}

impl Parents {
	pub fn add_parent(&mut self, parent: Object) -> Result<()> {
		let mut inner = self.0.write();
		match *inner {
			Inner::None => *inner = Inner::Builtin(vec![parent]),
			Inner::Builtin(ref mut vec) => vec.push(parent),
			Inner::Object(ref obj) => { obj.call_attr_lit("push", &[&parent])?; },
		}

		Ok(())
	}

	pub fn to_object(&self) -> Object {
		let mut inner = self.0.write();
		match *inner {
			Inner::None => {
				let obj = Object::default();
				*inner = Inner::Object(obj.clone());
				obj
			},
			Inner::Builtin(ref mut vec) => {
				let obj = Object::from(std::mem::replace(vec, vec![]));
				*inner = Inner::Object(obj.clone());
				obj
			},
			Inner::Object(ref obj) => obj.clone()
		}
	}

	fn with_iter<F: FnOnce(std::slice::Iter<Object>) -> Result<R>, R>(&self, f: F) -> Result<R> {
		match *self.0.read() {
			Inner::None => f([].iter()),
			Inner::Builtin(ref parents) => f(parents.iter()),
			Inner::Object(ref object) => object.call_downcast::<List>().and_then(|l| f(l.iter()))
		}
	}

	pub fn keys(&self) -> Result<Vec<Object>> {
		self.with_iter(|iter| Ok(iter.cloned().collect()))
	}

	pub fn has_lit<L: ?Sized>(&self, key: &L) -> Result<bool>
	where
		Literal: Borrow<L>,
		L: Hash + Eq
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

	pub fn get_lit<L: ?Sized>(&self, key: &L) -> Result<Option<Value>>
	where
		Literal: Borrow<L>,
		L: Hash + Eq
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

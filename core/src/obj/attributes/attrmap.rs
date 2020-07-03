use std::borrow::Borrow;
use crate::{Object, Result};
use crate::types::{RustFn, Text, Boolean};
use std::hash::{self, Hash, Hasher};
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};

use super::Value;
pub type Literal = &'static str;

#[derive(Clone, Default)]
pub struct AttrMap {
	literals: HashMap<Literal, Value>,
	// TODO: allow for `Text`s to be stored in `literals`.
	objects: Vec<(Object, Value)>
}

impl Debug for AttrMap {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_map()
			.entries(self.literals.iter())
			.entries(self.objects.iter().map(|(ref k, ref v)| (k, v)))
			.finish()
	}
}

fn eq_literal(obj: &Object, lit: Literal) -> Result<bool> {
	obj.call_attr_lit("==", &[&lit.into()])?
		.call_downcast_map(|x: &Boolean| x.into_inner())
}


impl AttrMap {
	#[inline]
	pub fn new() -> Self {
		AttrMap::default()
	}

	// in the future, this can be an exact size iterator
	pub fn keys<'a>(&'a self) -> impl Iterator<Item=Object> + 'a {
		self.literals.keys()
			.map(|k| Object::from(*k))
			.chain(self.objects.iter().map(|(k, _)| k.clone()))
	}

	#[inline]
	pub fn has_lit<K: Hash + Eq + ?Sized>(&self, key: &K) -> bool
	where
		for <'a> &'a str: Borrow<K>
	{
		self.literals.contains_key(key)
	}

	#[inline]
	pub fn get_lit<K: Hash + Eq + ?Sized>(&self, key: &K) -> Option<&Value>
	where
		for <'a> &'a str: Borrow<K>
	{
		self.literals.get(key)
	}

	#[inline]
	pub fn set_lit(&mut self, key: Literal, val: Value) {
		self.literals.insert(key, val);
	}

	#[inline]
	pub fn del_lit<K: Hash + Eq + ?Sized>(&mut self, key: &K) -> Option<Value>
	where
		for <'a> &'a str: Borrow<K>
	{
		self.literals.remove(key)
	}

	pub fn has_obj(&self, key: &Object) -> Result<bool> {
		for (ref k, _) in self.objects.iter() {
			if key.eq_obj(k)? {
				return Ok(true);
			}
		}

		Ok(false)
	}

	pub fn get_obj(&self, key: &Object) -> Result<Option<&Value>> {
		for (ref k, ref v) in self.objects.iter() {
			if key.eq_obj(k)? {
				return Ok(Some(v));
			}
		}

		Ok(None)
	}

	pub fn set_obj(&mut self, key: Object, value: Value) -> Result<()> {
		for (ref k, ref mut v) in self.objects.iter_mut() {
			if key.eq_obj(k)? {
				*v = value;
				return Ok(())
			}
		}

		self.objects.push((key, value));
		Ok(())
	}

	pub fn del_obj(&mut self, key: &Object) -> Result<Option<Value>> {
		let mut stop_index = None;
		for (i, (ref k, _)) in self.objects.iter().enumerate() {
			if key.eq_obj(k)? {
				stop_index = Some(i);
				break;;
			}
		}

		Ok(stop_index.map(|idx| self.objects.swap_remove(idx)).map(|x| x.1))
	}
}


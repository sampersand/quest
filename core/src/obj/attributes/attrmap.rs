use crate::{Object, Result, Literal};
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use std::hash::Hash;
use std::borrow::Borrow;

use super::Value;

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

impl AttrMap {
	/// Get a list of keys associated with this map.
	pub fn keys<'a>(&'a self) -> impl Iterator<Item=Object> + 'a {
		self.literals.keys()
			.map(|k| Object::from(k.to_string()))
			.chain(self.objects.iter().map(|(k, _)| k.clone()))
	}

	/// Checks to see if this map has `key`.
	#[inline]
	pub fn has_lit<L: ?Sized>(&self, key: &L) -> bool
	where
		Literal: Borrow<L>,
		L: Hash + Eq
	{
		self.literals.contains_key(key)
	}

	/// Gets the value associated with `key`.
	#[inline]
	pub fn get_lit<L: ?Sized>(&self, key: &L) -> Option<&Value>
	where
		Literal: Borrow<L>,
		L: Hash + Eq
	{
		self.literals.get(key)
	}

	/// Sets `key` to `value`.
	#[inline]
	pub fn set_lit(&mut self, key: Literal, value: Value) {
		self.literals.insert(key, value);
	}

	/// Deletes the value associated with `key`, returning it.
	#[inline]
	pub fn del_lit<L: ?Sized>(&mut self, key: &L) -> Option<Value>
	where
		Literal: Borrow<L>,
		L: Hash + Eq
	{
		self.literals.remove(key)
	}

	/// Checks to see if this map has `key`.
	pub fn has_obj(&self, key: &Object) -> Result<bool> {
		for (ref k, _) in self.objects.iter() {
			if key.eq_obj(k)? {
				return Ok(true);
			}
		}

		Ok(false)
	}

	/// Gets the value associated with `key`.
	pub fn get_obj(&self, key: &Object) -> Result<Option<&Value>> {
		for (ref k, ref v) in self.objects.iter() {
			if key.eq_obj(k)? {
				return Ok(Some(v));
			}
		}

		Ok(None)
	}

	/// Sets `key` to `value`.
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

	/// Deletes the value associated with `key`, returning it.
	pub fn del_obj(&mut self, key: &Object) -> Result<Option<Value>> {
		let mut stop_index = None;
		for (i, (ref k, _)) in self.objects.iter().enumerate() {
			if key.eq_obj(k)? {
				stop_index = Some(i);
				break;
			}
		}

		Ok(stop_index.map(|idx| self.objects.swap_remove(idx)).map(|x| x.1))
	}
}


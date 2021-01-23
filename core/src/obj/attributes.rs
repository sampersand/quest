use crate::{Object, Result, Literal, SharedCow};
use crate::types::Text;
use std::fmt::{self, Debug, Formatter};
use std::borrow::Borrow;
use std::hash::Hash;

mod parents;
mod attrmap;
mod value;

use attrmap::AttrMap;
pub use value::Value;
pub use parents::Parents;

#[derive(Debug, Clone, Default)]
struct Inner {
	map: AttrMap,
	parents: Parents
}

/// The attributes associated with an [`Object`](crate::Object).
#[derive(Default)]
pub struct Attributes {
	data: SharedCow<Inner>,
	id: usize
}

impl Debug for Attributes {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		// we explicitly don't include data because it can cause infinite regression.
		f.debug_struct("Attributes")
			.field("id", &self.id)
			.finish()
	}
}

impl Clone for Attributes {
	fn clone(&self) -> Self {
		Self::from_data(self.data.clone())
	}
}

impl Attributes {
	/// Create an empty `Attributes`, initialized with the given parents
	pub fn new(parents: impl Into<Parents>) -> Self {
		Self::from_data(
			SharedCow::new(Inner {
				parents: parents.into(),
				map: Default::default()
			})
		)
	}

	fn from_data(data: SharedCow<Inner>) -> Self {
		use std::sync::atomic::{AtomicUsize, Ordering};
		static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

		Attributes { data, id: ID_COUNTER.fetch_add(1, Ordering::Relaxed) }
	}

	/// Gets the id associated with these attributes.
	#[inline]
	pub fn id(&self) -> usize {
		self.id
	}

	/// Add a parent to the list of parents.
	pub fn add_parent(&self, parent: Object) -> Result<()> {
		self.data.write().parents.add_parent(parent)
	}

	/// Add a parent to the list of parents.
	pub fn prepend_parent(&self, parent: Object) -> Result<()> {
		self.data.write().parents.prepend_parent(parent)
	}

	/// Get a list of keys for this class, optionally including all keys defined on parents as well.
	pub fn keys(&self, include_parents: bool) -> Result<Vec<Object>> {
		let mut keys = vec![];

		keys.push(Literal::__PARENTS__.into());
		keys.push(Literal::__ID__.into());

		let inner = self.data.write();
		keys.extend(inner.map.keys());

		if include_parents {
			keys.extend(inner.parents.keys()?);
		}

		Ok(keys)
	}
}

impl Attributes {
	/// Checks to see if `self` directly or its parents includes `key`.
	pub fn has_lit<L: ?Sized>(&self, key: &L) -> Result<bool> 
	where
		Literal: Borrow<L>,
		L: Hash + Eq
	{
		if key == Literal::__ID__.borrow() || key == Literal::__PARENTS__.borrow() {
			Ok(true)
		} else {
			let inner = self.data.read();
			Ok(inner.map.has_lit(key) || inner.parents.has_lit(key)?)
		}
	}

	/// Gets the associated value to `key` from `self` directly or its parents.
	pub fn get_lit<L: ?Sized>(&self, key: &L) -> Result<Option<Value>>
	where
		Literal: Borrow<L>,
		L: Hash + Eq
	{
		if key == Literal::__ID__.borrow() {
			return Ok(Some(Object::from(self.id()).into()))
		}

		let inner = self.data.read();
		if key == Literal::__PARENTS__.borrow() {
			Ok(Some(inner.parents.to_object().into()))
		} else if let Some(lit) = inner.map.get_lit(key).cloned() {
			Ok(Some(lit))
		} else {
			inner.parents.get_lit(key)
		}
	}

	/// Sets the associated `key` to `value` from `self` directly or its parents.
	pub fn set_lit(&self, key: impl Into<Literal>, value: impl Into<Value>) {
		let mut inner = self.data.write();
		let key = key.into();
		let value = value.into();

		if key == Literal::__PARENTS__ {
			inner.parents = Parents::from(Object::from(value));
		} else {
			inner.map.set_lit(key, value);
		}
	}

	/// Deletes the associated value to `key` from `self` directly or its parents.
	pub fn del_lit<L: ?Sized>(&self, key: &L) -> Option<Value>
	where
		Literal: Borrow<L>,
		L: Hash + Eq
	{
		let mut inner = self.data.write();

		if key == Literal::__PARENTS__.borrow() {
			Some(std::mem::take(&mut inner.parents).into())
		} else {
			inner.map.del_lit(key)
		}
	}

	/// Checks to see if `self` directly or its parents includes `key`.
	pub fn has(&self, key: &Object) -> Result<bool> {
		if let Some(res) = key.downcast::<Text>().map(|text| self.has_lit(text.as_ref())) {
			return res
		}

		let inner = self.data.read();
		Ok(inner.map.has_obj(key)? || inner.parents.has_obj(key)?)
	}

	/// Gets the associated value to `key` from `self` directly or its parents.
	pub fn get(&self, key: &Object) -> Result<Option<Value>> {
		if let Some(res) = key.downcast::<Text>().map(|text| self.get_lit(text.as_ref())) {
			return res;
		}

		let inner = self.data.read();

		if let Some(obj) = inner.map.get_obj(key)? {
			Ok(Some(obj.clone()))
		} else {
			inner.parents.get_obj(key)
		}
	}

	/// Sets the associated `key` to `value` from `self` directly or its parents.
	pub fn set(&self, key: Object, value: Value) -> Result<()> {
		if let Some(text) = key.downcast::<Text>() {
			self.set_lit(str_to_static(text.as_ref()), value);
			return Ok(());
		}

		self.data.write().map.set_obj(key, value)
	}

	/// Deletes the associated value to `key` from `self` directly or its parents.
	pub fn del(&self, key: &Object) -> Result<Option<Value>> {
		if let Some(res) = key.downcast::<Text>().map(|text| self.del_lit(text.as_ref())) {
			return Ok(res);
		}

		self.data.write().map.del_obj(key)
	}
}

fn str_to_static(key: &str) -> &'static str {
	use std::collections::HashSet;
	use std::cell::RefCell;

	thread_local! {
		// a list of strings that have been converted so far; this is to improve efficiency.
		static STATIC_STRS: RefCell<HashSet<&'static str>> = RefCell::new(HashSet::new());
	}

	STATIC_STRS.with(|set| {
		if let Some(static_key) = set.borrow().get(key) {
			return *static_key;
		};

		// leak the string to turn it static.
		set.borrow_mut().insert(Box::leak(key.to_string().into_boxed_str()));
		match set.borrow().get(key) {
			Some(key) => *key,
			None => unreachable!("we just inserted this?")
		}
	})
}

use crate::literal::{__PARENTS__, __ID__};
use crate::{Object, Result, SharedCow};
use crate::types::Text;
use std::fmt::{self, Debug, Formatter};

mod parents;
mod attrmap;
mod value;

pub use value::Value;
use attrmap::{AttrMap, Literal_};
pub use parents::Parents;

#[derive(Debug, Clone, Default)]
struct Inner {
	map: AttrMap,
	parents: Parents
}

#[derive(Default)]
pub struct Attributes {
	data: SharedCow<Inner>,
	id: usize
}

impl Debug for Attributes {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_struct("Attributes")
			.field("id", &self.id)
			.finish()
	}
}

impl Clone for Attributes {
	fn clone(&self) -> Self {
		Attributes::from_data(self.data.clone())
	}
}

impl Attributes {
	pub fn new<P: Into<Parents>>(parents: P) -> Self {
		Attributes::from_data(
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

	#[inline]
	pub fn id(&self) -> usize {
		self.id
	}

	pub fn add_parent(&self, parent: Object) -> Result<()> {
		self.data.write().parents.add_parent(parent)
	}

	pub fn keys(&self, include_parents: bool) -> Result<Vec<Object>> {
		let mut keys = vec![];

		keys.push(__PARENTS__.into());
		keys.push(__ID__.into());

		let inner = self.data.write();
		keys.extend(inner.map.keys());
		if include_parents {
			keys.extend(inner.parents.keys()?);
		}

		Ok(keys)
	}
}

impl Attributes {
	pub fn has_lit(&self, key: &str) -> Result<bool> {
		if key == __ID__ || key == __PARENTS__ {
			Ok(true)
		} else {
			let inner = self.data.read();
			Ok(inner.map.has_lit(key) || inner.parents.has_lit(key)?)
		}
	}

	pub fn get_lit(&self, key: &str) -> Result<Option<Value>> {
		if key == __ID__ {
			return Ok(Some(Object::from(self.id()).into()))
		}

		let inner = self.data.read();
		if key == __PARENTS__ {
			Ok(Some(inner.parents.to_object().into()))
		} else if let Some(lit) = inner.map.get_lit(key).cloned() {
			Ok(Some(lit))
		} else {
			inner.parents.get_lit(key)
		}
	}

	pub fn set_lit(&self, key: Literal_, val: Value) {
		let mut inner = self.data.write();

		if __PARENTS__ == key {
			inner.parents = Parents::from(Object::from(val));
		} else {
			inner.map.set_lit(key, val);
		}
	}

	pub fn del_lit(&self, key: &str) -> Option<Value> {
		let mut inner = self.data.write();

		if __PARENTS__ == key {
			Some(std::mem::take(&mut inner.parents).into())
		} else {
			inner.map.del_lit(key)
		}
	}

	pub fn has(&self, key: &Object) -> Result<bool> {
		if let Some(res) = key.downcast::<Text>().map(|text| self.has_lit(text.as_ref())) {
			return res
		}

		let inner = self.data.read();
		Ok(inner.map.has_obj(key)? || inner.parents.has_obj(key)?)
	}

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

	pub fn set(&self, key: Object, value: Value) -> Result<()> {
		if let Some(lit) = key.downcast::<Text>().map(|text| str_to_static(text.as_ref())) {
			self.set_lit(lit, value);
			return Ok(());
		}

		self.data.write().map.set_obj(key, value)
	}

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

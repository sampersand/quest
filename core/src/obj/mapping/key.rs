use crate::{Object, Result, types};

pub trait EqKey {
	fn eq_key(&self, key: &Key) -> Result<bool>;
}

/// A Key that can be used to access an [`Object`](#)'s mapping.
///
/// This can either be a `&'static str`, which is the preferred way to access
/// attributes from within rust, or an [`Object`](#), which can be any Quest
/// object.
#[derive(Debug, Clone)]
pub enum Key {
	/// A Quest object that can be used as an index.
	Object(Object),
	/// A literal key, which can be used to drastically improve lookup times for
	/// Builtin rust objects.
	Literal(&'static str),
}

impl From<Object> for Key {
	fn from(obj: Object) -> Self {
		Key::Object(obj)
	}
}

impl From<&'static str> for Key {
	fn from(lit: &'static str) -> Self {
		Key::Literal(lit)
	}
}

impl From<Key> for Object {
	fn from(key: Key) -> Self {
		match key {
			Key::Object(o) => o,
			Key::Literal(l) => l.into()
		}
	}
}

impl EqKey for Key {
	fn eq_key(&self, key: &Key) -> Result<bool> {
		match self {
			Key::Literal(lit) => lit.eq_key(key),
			Key::Object(obj) => obj.eq_key(key),
		}
	}
}

impl EqKey for str {
	fn eq_key(&self, key: &Key) -> Result<bool> {
		match key {
			Key::Literal(lit) => Ok(self == *lit),
			Key::Object(obj) if obj.is_a::<types::Text>() =>
				unsafe { 
					Ok(obj.downcast_ref_unchecked::<types::Text>().as_ref() == self)
				},
			Key::Object(_) => Ok(false)

		}
	}
}

impl EqKey for Object {
	fn eq_key(&self, key: &Key) -> Result<bool> {
		match key {
			Key::Literal(lit) if self.is_a::<types::Text>() =>
				unsafe { 
					Ok(self.downcast_ref_unchecked::<types::Text>().as_ref() == *lit)
				},
			Key::Literal(_) => Ok(false),
			Key::Object(obj) => self.eq_obj(obj)
		}
	}
}

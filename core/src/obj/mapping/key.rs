use crate::{Object, Result, types, EqResult};
use std::ops::Deref;

#[derive(Debug, Clone)]
pub enum Key {
	Object(Object),
	Literal(&'static str),
}

impl Key {
	pub fn try_as_str<'a>(&'a self) -> Option<impl Deref<Target=str> + 'a> {
		enum KeyDeref<'a> {
			Literal(&'static str),
			Text(Box<dyn Deref<Target=types::Text> + 'a>)
		}

		impl<'a> Deref for KeyDeref<'a> {
			type Target = str;
			fn deref(&self) -> &str {
				match self {
					KeyDeref::Literal(lit) => lit,
					KeyDeref::Text(t) => t.deref().as_ref()
				}
			}
		}

		match self {
			Key::Literal(lit) => Some(KeyDeref::Literal(lit)),
			Key::Object(obj) =>
				obj.downcast_ref::<types::Text>()
					.map(|x| KeyDeref::Text(Box::new(x)))
		}
	}
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

impl EqResult<str> for Object {
	fn equals(&self, rhs: &str) -> Result<bool> {
		Ok(self.downcast_ref::<types::Text>()
			.map(|txt| txt.as_ref() == rhs)
			.unwrap_or(false))
	}
}

impl EqResult for Key {
	fn equals(&self, rhs: &Key) -> Result<bool> {
		match (self, rhs) {
			(Key::Literal(lit_lhs), Key::Literal(lit_rhs)) => Ok(lit_lhs == lit_rhs),
			(Key::Object(obj_lhs), Key::Object(obj_rhs)) => obj_lhs.equals(obj_rhs),
			(Key::Literal(lit), Key::Object(obj))
				| (Key::Object(obj), Key::Literal(lit)) => obj.equals(*lit)
		}
	}
}


impl EqResult<Key> for str {
	fn equals(&self, rhs: &Key) -> Result<bool> {
		match rhs {
			Key::Literal(lit) => Ok(lit == &self),
			Key::Object(obj) => obj.equals(self)
		}
	}
}

impl EqResult<Object> for Key {
	fn equals(&self, rhs: &Object) -> Result<bool> {
		match self {
			Key::Object(obj) => rhs.equals(obj),
			Key::Literal(lit) => rhs.equals(*lit)
		}
	}
}
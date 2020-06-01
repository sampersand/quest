use std::convert::TryFrom;
use std::any::Any;

use crate::obj::{Object, Result, types, EqResult};

#[derive(Debug, Clone)]
pub enum Key {
	Object(Object),
	Literal(&'static str),
}

impl TryFrom<Key> for &'static str {
	type Error = ();
	fn try_from(key: Key) -> std::result::Result<Self, Self::Error> {
		match key {
			Key::Object(_) => Err(()),
			Key::Literal(lit) => Ok(lit)
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
		use Key::*;
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
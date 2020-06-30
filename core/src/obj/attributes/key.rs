use crate::literals::Literal;
use crate::Object;

pub enum Key {
	Literal(Literal),
	Object(Object)
}

impl From<Literal> for Key {
	#[inline]
	fn from(lit: Literal) -> Self {
		Key::Literal(lit)
	}
}

impl From<Object> for Key {
	#[inline]
	fn from(obj: Object) -> Self {
		Key::Object(obj)
	}
}

impl From<Key> for Object {
	#[inline]
	fn from(key: Key) -> Self {
		match key {
			Key::Literal(lit) => lit.into(),
			Key::Object(obj) => obj
		}
	}
}
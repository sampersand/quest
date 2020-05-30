use crate::obj::{Object, Result, types};


#[derive(Debug, Clone)]
pub enum Key {
	Object(Object),
	Literal(&'static str)
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

impl Key {
	pub fn equals(&self, rhs: &Key) -> Result<bool> {
		match (self, rhs) {
			(Key::Literal(lit_lhs), Key::Literal(lit_rhs)) => Ok(lit_lhs == lit_rhs),
			(Key::Object(obj_lhs), Key::Object(obj_rhs)) => obj_lhs.equals(obj_rhs),
			(Key::Literal(lit), Key::Object(obj)) | (Key::Object(obj), Key::Literal(lit)) => {
				Ok(obj.downcast_ref::<types::Text>()
					.map(|text| text.as_ref() == *lit)
					.unwrap_or(false))
			}
		}
	}
}


// impl Key {
// 	fn does_eql(&self, rhs: &Object) -> obj::Result<bool> {
// 		Ok(match self {
// 			Key::Object(o) => o.call("==", Args::new_slice(&[k.clone().into()], Default::default()))?
// 					.downcast_ref::<types::Boolean>()
// 					.map(|x| bool::from(*x))
// 					.unwrap_or(false),
// 			Key::Literal(l) => {}
// 		})
// 	}
// }
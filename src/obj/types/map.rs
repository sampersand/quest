use crate::obj::{Object, types::ObjectType};
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};
use std::collections::HashMap;

type Inner = HashMap<Object, Object>;

#[derive(Clone)]
pub struct Map(Inner);

impl Debug for Map {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			write!(f, "Map({:?})", self.as_ref())
		} else {
			Debug::fmt(&self.as_ref(), f)
		}
	}
}

impl Map {
	pub fn new(map: Inner) -> Self {
		Map(map)
	}
}

impl From<Inner> for Map {
	fn from(map: Inner) -> Self {
		Map::new(map)
	}
}

impl From<Inner> for Object {
	fn from(map: Inner) -> Self {
		Map::from(map).into()
	}
}

impl AsRef<Inner> for Map {
	fn as_ref(&self) -> &Inner {
		&self.0
	}
}

impl_object_type!{for Map, super::Basic;
	"@text" => (|args| todo!("@text")),
	"@bool" => (|args| todo!("@bool")),
	"@map" => (|args| todo!("@map")),
	"@list" => (|args| todo!("@list")),
	"@clone" => (|args| todo!("@clone")),

	"len" => (|args| todo!("len")),
	"index" => (|args| todo!("index")),
	"[]" => (|args| todo!("[]")),
	"[]=" => (|args| todo!("[]=")),
	"[]~" => (|args| todo!("[]~")),
	"&" => (|args| todo!("&")),
	"|" => (|args| todo!("|")),
	"^" => (|args| todo!("^")),
	"-" => (|args| todo!("-")),
}

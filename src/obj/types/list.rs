use crate::obj::{Object, types::ObjectType};
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};


type Inner = Vec<Object>;

#[derive(Clone)]
pub struct List(Inner);

impl Debug for List {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			write!(f, "List({:?})", self.as_ref())
		} else {
			Debug::fmt(&self.as_ref(), f)
		}
	}
}

impl List {
	pub fn new(list: Inner) -> Self {
		List(list)
	}
}

impl From<Inner> for List {
	fn from(list: Inner) -> Self {
		List::new(list)
	}
}

impl From<Inner> for Object {
	fn from(list: Inner) -> Self {
		List::from(list).into()
	}
}

impl AsRef<[Object]> for List {
	fn as_ref(&self) -> &[Object] {
		self.0.as_ref()
	}
}

impl_object_type!{for List, super::Basic;
	"@text" => (|args| todo!("@text")),
	"@bool" => (|args| todo!("@bool")),
	"@map" => (|args| todo!("@map")),
	"@list" => (|args| todo!("@list")),
	"@clone" => (|args| todo!("@clone")),

	"does_include" => (|args| todo!("does_include")),
	"index" => (|args| todo!("index")),
	"is_empty" => (|args| todo!("is_empty")),
	"len" => (|args| todo!("len")),
	"[]" => (|args| todo!("[]")),
	"[]=" => (|args| todo!("[]=")),
	"join" => (|args| todo!("join")),
	"&" => (|args| todo!("&")),
	"|" => (|args| todo!("|")),
	"^" => (|args| todo!("^")),
	"-" => (|args| todo!("-")),
	// "[]~" => (|args| todo!("[]~")),
}


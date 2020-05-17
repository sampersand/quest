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

impl From<List> for Inner {
	fn from(list: List) -> Self {
		list.0
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

mod impls {
	use super::List;
	use crate::obj::{Object, Result, Args};

	pub fn at_text(args: Args) -> Result<Object> {
		todo!("List::at_text");
	}

	pub fn at_bool(args: Args) -> Result<Object> {
		todo!("List::at_bool");
	}

	pub fn at_map(args: Args) -> Result<Object> {
		todo!("List::at_map");
	}

	pub fn at_list(args: Args) -> Result<Object> {
		todo!("List::at_list");
	}

	pub fn clone(args: Args) -> Result<Object> {
		todo!("List::clone");
	}

	pub fn does_include(args: Args) -> Result<Object> {
		todo!("List::does_include");
	}

	pub fn index_of(args: Args) -> Result<Object> {
		todo!("List::index_of");
	}

	pub fn is_empty(args: Args) -> Result<Object> {
		todo!("List::is_empty");
	}

	pub fn len(args: Args) -> Result<Object> {
		todo!("List::len");
	}

	pub fn index(args: Args) -> Result<Object> {
		todo!("List::index");
	}

	pub fn index_assign(args: Args) -> Result<Object> {
		todo!("List::index_assign");
	}

	pub fn join(args: Args) -> Result<Object> {
		todo!("List::join");
	}

	pub fn bitand(args: Args) -> Result<Object> {
		todo!("List::bitand");
	}

	pub fn bitor(args: Args) -> Result<Object> {
		todo!("List::bitor");
	}

	pub fn bitxor(args: Args) -> Result<Object> {
		todo!("List::bitxor");
	}

	pub fn sub(args: Args) -> Result<Object> {
		todo!("List::sub");
	}

}


impl_object_type!{for List [(parent super::Basic) (convert "@list")]:
	"@text" => impls::at_text,
	"@map" => impls::at_map,
	"@list" => impls::at_list,
	"clone" => impls::clone,

	"does_include" => impls::does_include,
	"index_of" => impls::index_of,
	"is_empty" => impls::is_empty,
	"len" => impls::len,
	"[]" => impls::index,
	"[]=" => impls::index_assign,
	"join" => impls::join,
	"&" => impls::bitand,
	"|" => impls::bitor,
	"^" => impls::bitxor,
	"-" => impls::sub
}


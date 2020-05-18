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

mod impls {
	use super::Map;
	use crate::obj::{Object, Result, Args};

	pub fn at_text(args: Args) -> Result<Object> {
		todo!("Map::at_text")
	}

	pub fn at_bool(args: Args) -> Result<Object> {
		todo!("Map::at_bool")
	}

	pub fn at_map(args: Args) -> Result<Object> {
		todo!("Map::at_map")
	}

	pub fn at_list(args: Args) -> Result<Object> {
		todo!("Map::at_list")
	}

	pub fn clone(args: Args) -> Result<Object> {
		todo!("Map::clone")
	}

	pub fn len(args: Args) -> Result<Object> {
		todo!("Map::len")
	}

	pub fn index_of(args: Args) -> Result<Object> {
		todo!("Map::index_of")
	}

	pub fn index(args: Args) -> Result<Object> {
		todo!("Map::index")
	}

	pub fn index_assign(args: Args) -> Result<Object> {
		todo!("Map::index_assign")
	}

	pub fn index_del(args: Args) -> Result<Object> {
		todo!("Map::index_del")
	}

	pub fn bitand(args: Args) -> Result<Object> {
		todo!("Map::bitand")
	}

	pub fn bitor(args: Args) -> Result<Object> {
		todo!("Map::bitor")
	}

	pub fn bitxor(args: Args) -> Result<Object> {
		todo!("Map::bitxor")
	}

	pub fn sub(args: Args) -> Result<Object> {
		todo!("Map::sub")
	}
}

impl_object_type!{
for Map [(parent super::Basic) (convert "@map")]:
	"@text" => impls::at_text,
	"@bool" => impls::at_bool,
	"@map" => impls::at_map,
	"@list" => impls::at_list,
	"clone" => impls::clone,

	"len" => impls::len,
	"index_of" => impls::index_of,
	"[]" => impls::index,
	"[]=" => impls::index_assign,
	"[]~" => impls::index_del,
	"&" => impls::bitand,
	"|" => impls::bitor,
	"^" => impls::bitxor,
	"-" => impls::sub,
}
use crate::obj::{Object, DataEnum, Mapping, types::ObjectType};
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Null;

impl Null {
	pub fn new() -> Self {
		Null
	}
}

impl From<Null> for DataEnum {
	fn from(_: Null) -> Self {
		Self::Null
	}
}

impl From<()> for Object {
	fn from(_: ()) -> Self {
		Self::new(Null)
	}
}

impl Object {
	fn is_null(&self) -> bool {
		self.0.data == DataEnum::Null
	}
}

macro_rules! assert_is_null {
	($args:expr) => {{
		let this = $args.get(0).unwrap();
		assert!(this.is_null(), "bad `this` given: {:#?}", this);
	}};
}


impl_object_type!{for Null, super::Basic;
	"()" => (|_args| {
		assert_is_null!(_args);
		Ok(Null.into())
	}),

	"==" => (|args| {
		assert_is_null!(args);
		Ok(args.get(1)?.is_null().into())
	}),

	"@bool" => (|_args| {
		assert_is_null!(_args);
		Ok(false.into())
	}),

	"@num" => (|_args| {
		assert_is_null!(_args);
		Ok(0.into())
	}),

	"@text" => (|_args| {
		assert_is_null!(_args);
		Ok("null".into())
	}),

	"clone" => (|_args| {
		assert_is_null!(_args);
		Ok(Null.into())
	})
}

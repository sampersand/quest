use crate::obj::{Object, Mapping, types::ObjectType};
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Null;

impl From<()> for Object {
	fn from(_: ()) -> Self {
		Null.into()
	}
}

impl From<()> for Null {
	fn from(_: ()) -> Self {
		Null
	}
}

impl Object {
	fn is_null(&self) -> bool {
		self.downcast_ref::<Null>().is_some()
	}
}

macro_rules! assert_is_null {
	($args:expr) => {{
		$args.this_obj::<Null>()
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

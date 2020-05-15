use crate::obj::{Object, Mapping, types::{self, ObjectType}};
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Null;

pub const NULL: Null = Null::new();

impl Null {
	pub const fn new() -> Self {
		Null
	}
}

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

impl From<Null> for types::Boolean {
	fn from(_: Null) -> Self {
		const FALSE: types::Boolean = types::boolean::FALSE;
		FALSE
	}
}

impl From<Null> for types::Number {
	fn from(_: Null) -> Self {
		const ZERO: types::Number = types::number::ZERO;
		ZERO
	}
}

impl From<Null> for types::Text {
	fn from(_: Null) -> Self {
		const NULL: types::Text = types::Text::new_static("null");
		NULL
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
		Ok(args.this::<Null>()?.eq(&*args.get_downcast::<Null>(1)?).into())
	}),

	"@bool" => (|args| {
		Ok(Boolean::from(*args.this::<Null>()?).into())
	}),

	"@num" => (|args| {
		Ok(Number::from(*args.this::<Null>()?).into())
	}),

	"@text" => (|args| {
		Ok(Text::from(*args.this::<Null>()?).into())
	}),

	"clone" => (|args| {
		Ok(args.this::<Null>()?.clone().into())
	})
}

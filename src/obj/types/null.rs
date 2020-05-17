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


mod impls {
	use super::Null;
	use crate::obj::{Object, Result, Args, types};

	pub fn at_bool(_args: Args) -> Result<Object> {
		debug_assert!(_args.this().expect("bad this given").is_a::<Null>());
		Ok(types::boolean::FALSE.into())
	}

	pub fn at_num(_args: Args) -> Result<Object> {
		debug_assert!(_args.this().expect("bad this given").is_a::<Null>());
		Ok(types::number::ZERO.into())
	}

	pub fn at_text(_args: Args) -> Result<Object> {
		debug_assert!(_args.this().expect("bad this given").is_a::<Null>());
		const NULL_TEXT: types::Text = types::Text::new_static("null");
		Ok(NULL_TEXT.into())
	}

	pub fn call(_args: Args) -> Result<Object> {
		debug_assert!(_args.this().expect("bad this given").is_a::<Null>());
		Ok(Object::default())
	}

	pub fn eql(args: Args) -> Result<Object> {
		debug_assert!(args.this().expect("bad this given").is_a::<Null>());
		Ok(args.arg_downcast::<Null>(0).is_ok().into())
	}

	pub fn clone(_args: Args) -> Result<Object> {
		debug_assert!(_args.this().expect("bad this given").is_a::<Null>());
		Ok(Null.into())
	}
}

impl_object_type!{for Null, super::Basic;
	"@bool" => (impls::at_bool),
	"@num" => (impls::at_num),
	"@text" => (impls::at_text),
	"clone" => (impls::clone),
	"()" => (impls::call),
	"==" => (impls::eql),
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn at_bool() {
		assert_call_eq!(for Boolean;
			types::boolean::FALSE, at_bool(NULL) -> Boolean,
		);
	}

	#[test]
	fn at_num() {
		assert_call_eq!(for Boolean;
			types::number::ZERO, at_num(NULL) -> Number,
		);
	}

	#[test]
	fn at_text() {
		assert_call_eq!(for Boolean;
			Text::new_static("null"), at_text(NULL) -> Text,
		);
	}

	#[test]
	fn clone() {
		assert_call_eq!(for Boolean;
			NULL, clone(NULL) -> Null,
		);
	}

	#[derive(Debug)]
	struct Dummy;

	impl From<Dummy> for Object {
		fn from(_: Dummy) -> Object {
			Object::new_with_parent(Dummy, None)
		}
	}

	#[test]
	fn call() {
		assert_call_eq!(for Boolean;
			NULL, call(NULL) -> Null,
			NULL, call(NULL, Dummy) -> Null,
			NULL, call(NULL, Dummy, Dummy) -> Null,
		);
	}

	#[test]
	fn eql() {
		assert_call_eq!(for Boolean;
			boolean::FALSE, eql(Null, Dummy) -> Boolean,
			boolean::TRUE, eql(Null, Null) -> Boolean,
		);
	}
}
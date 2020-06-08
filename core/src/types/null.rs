use crate::{Object, types};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Null;

impl Display for Null {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "null")
	}
}

impl Null {
	#[inline]
	pub const fn new() -> Self {
		Null
	}

	pub const NULL: Null = Null::new();
	pub const NULL_STR: &'static str = "null";
}

impl From<()> for Object {
	fn from(_: ()) -> Self {
		Null::new().into()
	}
}

impl From<()> for Null {
	fn from(_: ()) -> Self {
		Null::new()
	}
}

impl From<Null> for types::Boolean {
	#[inline]
	fn from(_: Null) -> Self {
		types::Boolean::FALSE
	}
}

impl From<Null> for types::List {
	#[inline]
	fn from(_: Null) -> Self {
		types::List::new(vec![])
	}
}

impl From<Null> for types::Number {
	fn from(_: Null) -> Self {
		types::Number::ZERO
	}
}

impl From<Null> for types::Text {
	fn from(_: Null) -> Self {
		const NULL_TEXT: types::Text = types::Text::new_static(Null::NULL_STR);
		NULL_TEXT
	}
}


mod impls {
	use super::Null;
	use crate::{Object, Result, Args, types};

	pub fn at_bool(_args: Args) -> Result<Object> {
		debug_assert!(_args.this().expect("bad this given").is_a::<Null>());
		Ok(types::Boolean::from(Null).into())
	}

	pub fn at_list(_args: Args) -> Result<Object> {
		debug_assert!(_args.this().expect("bad this given").is_a::<Null>());
		Ok(types::List::from(Null).into())
	}

	pub fn at_num(_args: Args) -> Result<Object> {
		debug_assert!(_args.this().expect("bad this given").is_a::<Null>());
		Ok(types::Number::from(Null).into())
	}

	pub fn at_text(_args: Args) -> Result<Object> {
		debug_assert!(_args.this().expect("bad this given").is_a::<Null>());
		Ok(types::Text::from(Null).into())
	}

	pub fn call(_args: Args) -> Result<Object> {
		debug_assert!(_args.this().expect("bad this given").is_a::<Null>());
		Ok(Object::default())
	}

	pub fn eql(args: Args) -> Result<Object> {
		debug_assert!(args.this().expect("bad this given").is_a::<Null>());
		Ok(args.arg(0)?.is_a::<Null>().into())
	}

	pub fn clone(_args: Args) -> Result<Object> {
		debug_assert!(_args.this().expect("bad this given").is_a::<Null>());
		Ok(Null.into())
	}
}

impl_object_type!{
for Null [(parents super::Basic)]:
	"@bool" => impls::at_bool,
	"@list" => impls::at_list,
	"@num" => impls::at_num,
	"@text" => impls::at_text,
	"clone" => impls::clone,
	"()" => impls::call,
	"==" => impls::eql,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn at_bool() {
		assert_call_eq!(for Boolean;
			types::Boolean::FALSE, at_bool(Null::NULL) -> Boolean,
		);
	}

	#[test]
	fn at_num() {
		assert_call_eq!(for Boolean;
			types::Number::ZERO, at_num(Null::NULL) -> Number,
		);
	}

	#[test]
	fn at_text() {
		assert_call_eq!(for Boolean;
			Text::new_static("null"), at_text(Null::NULL) -> Text,
		);
	}

	#[test]
	fn clone() {
		assert_call_eq!(for Boolean;
			Null::NULL, clone(Null::NULL) -> Null,
		);
	}

	dummy_object!(struct Dummy;);

	#[test]
	fn call() {
		assert_call_eq!(for Boolean;
			Null::NULL, call(Null::NULL) -> Null,
			Null::NULL, call(Null::NULL, Dummy) -> Null,
			Null::NULL, call(Null::NULL, Dummy, Dummy) -> Null,
		);
	}

	#[test]
	fn eql() {
		assert_call_eq!(for Boolean;
			Boolean::FALSE, eql(Null, Dummy) -> Boolean,
			Boolean::TRUE, eql(Null, Null) -> Boolean,
		);
	}
}
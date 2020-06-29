use crate::{Object, Args};
use crate::types::{Boolean, List, Number, Text};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Null;

impl Display for Null {
	#[inline]
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

impl From<Null> for Boolean {
	#[inline]
	fn from(_: Null) -> Self {
		Boolean::FALSE
	}
}

impl From<Null> for List {
	#[inline]
	fn from(_: Null) -> Self {
		List::new(vec![])
	}
}

impl From<Null> for Number {
	#[inline]
	fn from(_: Null) -> Self {
		Number::ZERO
	}
}

impl From<Null> for Text {
	#[inline]
	fn from(_: Null) -> Self {
		const NULL_TEXT: Text = Text::new_static("null");
		NULL_TEXT
	}
}


mod impls {
	use super::Null;
	use crate::{Object, Result, ArgsOld, types};

	pub fn at_bool(_args: ArgsOld) -> Result<Object> {
		debug_assert!(_args.this().expect("bad this given").is_a::<Null>());
		Ok(types::Boolean::from(Null).into())
	}

	pub fn at_list(_args: ArgsOld) -> Result<Object> {
		debug_assert!(_args.this().expect("bad this given").is_a::<Null>());
		Ok(types::List::from(Null).into())
	}

	pub fn at_num(_args: ArgsOld) -> Result<Object> {
		debug_assert!(_args.this().expect("bad this given").is_a::<Null>());
		Ok(types::Number::from(Null).into())
	}

	pub fn at_text(_args: ArgsOld) -> Result<Object> {
		debug_assert!(_args.this().expect("bad this given").is_a::<Null>());
		Ok(types::Text::from(Null).into())
	}

	pub fn call(_args: ArgsOld) -> Result<Object> {
		debug_assert!(_args.this().expect("bad this given").is_a::<Null>());
		Ok(Object::default())
	}

	pub fn eql(args: ArgsOld) -> Result<Object> {
		debug_assert!(args.this().expect("bad this given").is_a::<Null>());
		Ok(args.arg(0)?.is_a::<Null>().into())
	}

	pub fn clone(_args: ArgsOld) -> Result<Object> {
		debug_assert!(_args.this().expect("bad this given").is_a::<Null>());
		Ok(Null.into())
	}
}

impl Null {
	#[allow(non_snake_case)]
	pub fn qs___inspect__(&self, _: Args) -> Result<Text, !> {
		Ok(Text::from(*self))
	}
}


impl_object_type!{
for Null [(parents super::Basic)]:
	"@text" => impls::at_text,
	"__inspect__" => method Null::qs___inspect__,
	"@bool" => impls::at_bool,
	"@list" => impls::at_list,
	"@num" => impls::at_num,
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
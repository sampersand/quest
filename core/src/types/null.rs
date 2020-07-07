use crate::{Object, Args, Result};

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
	#[inline]
	fn from(_: ()) -> Self {
		Null::new().into()
	}
}

impl From<()> for Null {
	#[inline]
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


impl Null {
	#[inline]
	pub fn qs_inspect(_: &Object, _: Args) -> Result<Object> {
		Ok(Text::from(Null).into())
	}

	#[inline]
	pub fn qs_at_bool(_: &Object, _: Args) -> Result<Object> {
		Ok(Boolean::from(Null).into())
	}

	#[inline]
	pub fn qs_at_list(_: &Object, _: Args) -> Result<Object> {
		Ok(List::from(Null).into())
	}

	#[inline]
	pub fn qs_at_num(_: &Object, _: Args) -> Result<Object> {
		Ok(Number::from(Null).into())
	}

	#[inline]
	pub fn qs_at_text(_: &Object, _: Args) -> Result<Object> {
		Ok(Text::from(Null).into())
	}

	#[inline]
	pub fn qs_call(_: &Object, _: Args) -> Result<Object> {
		Ok(Null.into())
	}

	#[inline]
	pub fn qs_eql(_: &Object, args: Args) -> Result<Object> {
		Ok(args.arg(0)?.is_a::<Null>().into())
	}
}


impl_object_type!{
for Null {
	#[inline]
	fn new_object(self) -> Object where Self: Sized {
		use lazy_static::lazy_static;
		use crate::types::ObjectType;

		lazy_static! {
			static ref NULL: Object = Object::new_with_parent(Null::NULL, vec![Null::mapping()]);
		}

		NULL.deep_clone()
	}
}
[(parents super::Basic)]:
	"@text" => function Null::qs_at_text,
	"inspect" => function Null::qs_inspect,
	"@bool" => function Null::qs_at_bool,
	"@list" => function Null::qs_at_list,
	"@num" => function Null::qs_at_num,
	"()" => function Null::qs_call,
	"==" => function Null::qs_eql,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn at_bool() {
		<Null as crate::types::ObjectType>::_wait_for_setup_to_finish();
		assert_downcast_eq!(Boolean; Null::qs_at_bool(&Null.into(), args!()).unwrap(), false);
		assert_downcast_eq!(Boolean; Null::qs_at_bool(&Null.into(), args!(true)).unwrap(), false);
	}

	#[test]
	fn at_num() {
		assert_downcast_eq!(Number; Null::qs_at_num(&Null.into(), args!()).unwrap(), Number::ZERO);
		assert_downcast_eq!(Number; Null::qs_at_num(&Null.into(), args!(true)).unwrap(),
			Number::ZERO);
	}

	#[test]
	fn at_text() {
		assert_downcast_eq!(Text; Null::qs_at_text(&Null.into(), args!()).unwrap(),
			Text::new_static("null"));
	}

	#[derive(Debug, Clone)]
	struct Dummy;
	impl_object_type! { for Dummy [(parents crate::types::Basic)]: }

	#[test]
	fn call() {
		assert!(Null::qs_call(&Null.into(), args!()).unwrap().is_a::<Null>());
		assert!(Null::qs_call(&Null.into(), args!(Dummy)).unwrap().is_a::<Null>());
		assert!(Null::qs_call(&Null.into(), args!(Dummy, Dummy)).unwrap().is_a::<Null>());
	}

	#[test]
	fn eql() {
		assert_downcast_eq!(Boolean; Null::qs_eql(&Null.into(), args!(Dummy)).unwrap(), false);
		assert_downcast_eq!(Boolean; Null::qs_eql(&Null.into(), args!(Null)).unwrap(), true);
		assert_downcast_eq!(Boolean; Null::qs_eql(&Null.into(), args!(Null, Dummy)).unwrap(), true);

		assert_missing_parameter!(Null::qs_eql(&Null.into(), args!()), 0);
	}
}
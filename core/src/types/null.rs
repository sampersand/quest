use crate::{Object, Args, Result};

use crate::types::{Boolean, List, Number, Text};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Null;

impl Display for Null {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt("null", f)
	}
}

impl Null {
	#[inline(always)]
	pub const fn new() -> Self {
		Self
	}
}

impl From<()> for Object {
	#[inline]
	fn from(_: ()) -> Self {
		Null.into()
	}
}

impl From<()> for Null {
	fn from(_: ()) -> Self {
		Null
	}
}

impl From<Null> for Boolean {
	#[inline]
	fn from(_: Null) -> Self {
		Self::FALSE
	}
}

impl From<Null> for List {
	#[inline]
	fn from(_: Null) -> Self {
		Self::new(vec![])
	}
}

impl From<Null> for Number {
	#[inline]
	fn from(_: Null) -> Self {
		Self::ZERO
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
	pub fn qs_inspect(_: &Object, _: Args) -> Result<Object> {
		Ok(Text::from(Self).into())
	}

	pub fn qs_at_bool(_: &Object, _: Args) -> Result<Object> {
		Ok(Boolean::from(Self).into())
	}

	pub fn qs_at_list(_: &Object, _: Args) -> Result<Object> {
		Ok(List::from(Self).into())
	}

	pub fn qs_at_num(_: &Object, _: Args) -> Result<Object> {
		Ok(Number::from(Self).into())
	}

	pub fn qs_at_text(_: &Object, _: Args) -> Result<Object> {
		Ok(Text::from(Self).into())
	}

	pub fn qs_call(_: &Object, _: Args) -> Result<Object> {
		Ok(Self.into())
	}

	pub fn qs_eql(_: &Object, args: Args) -> Result<Object> {
		Ok(args.arg(0)?.is_a::<Self>().into())
	}
}


/*impl crate::obj::ConvertToDataType for Null {
	#[inline]
	fn into_datatype(self) -> crate::obj::DataType {
		crate::obj::DataType::Null(self)
	}
}*/

impl_object_type!{
for Null {
	#[inline]
	fn new_object(self) -> Object {
		use lazy_static::lazy_static;
		use crate::types::ObjectType;

		lazy_static! {
			static ref NULL: Object = Object::new_with_parent(Null, vec![Null::mapping()]);
		}

		NULL.deep_clone()
	}
}
[(parents super::Basic) (no_convert)]:
	"@text" => function Self::qs_at_text,
	"inspect" => function Self::qs_inspect,
	"@bool" => function Self::qs_at_bool,
	"@list" => function Self::qs_at_list,
	"@num" => function Self::qs_at_num,
	"()" => function Self::qs_call,
	"==" => function Self::qs_eql,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn display() {
		assert_eq!(Null.to_string(), "null");
	}

	#[test]
	fn from_unit() {
		<Null as crate::types::ObjectType>::initialize().unwrap();

		assert_eq!(Null::from(()), Null);
		Object::from(()).downcast_and_then(|_: &Null| {}).unwrap();
	}

	mod qs {
		use super::*;
		#[test]
		fn at_bool() {
			assert_call_eq!(Null::qs_at_bool(Null) -> Boolean, false);

			assert_call_idempotent!(Null::qs_at_bool(Null));
		}

		#[test]
		fn at_num() {
			assert_call_eq!(Null::qs_at_num(Null) -> Number, 0);

			assert_call_idempotent!(Null::qs_at_num(Null));
		}

		#[test]
		fn at_text() {
			assert_call_eq!(Null::qs_at_text(Null) -> Text, *"null");

			assert_call_idempotent!(Null::qs_at_text(Null));
		}

		#[test]
		fn inspect() {
			assert_call_eq!(Null::qs_inspect(Null) -> Text, *"null");

			assert_call_idempotent!(Null::qs_inspect(Null));
		}

		#[test]
		fn at_list() {
			assert_call!(Null::qs_at_list(Null); List::is_empty);

			assert_call_idempotent!(Null::qs_at_list(Null));
		}

		#[derive(Debug, Clone)]
		struct Dummy;
		impl_object_type! { for Dummy [(parents crate::types::Basic)]: }

		#[test]
		fn call() {
			<Dummy as crate::types::ObjectType>::initialize().unwrap();

			assert_call_eq!(Null::qs_call(Null) -> Null, Null);
			assert_call_eq!(Null::qs_call(Null, Dummy) -> Null, Null);
			assert_call_eq!(Null::qs_call(Null, Dummy, Dummy) -> Null, Null);

			assert_call_idempotent!(Null::qs_call(Null));
		}

		#[test]
		fn eql() {
			<Dummy as crate::types::ObjectType>::initialize().unwrap();

			assert_call_eq!(Null::qs_eql(Null, Dummy) -> Boolean, false);
			assert_call_eq!(Null::qs_eql(Null, Null) -> Boolean, true);
			assert_call_eq!(Null::qs_eql(Null, Null, Dummy) -> Boolean, true);

			assert_call_missing_parameter!(Null::qs_eql(Null), 0);
			assert_call_idempotent!(Null::qs_eql(Null, Null));
			assert_call_idempotent!(Null::qs_eql(Null, Dummy));
		}
	}
}

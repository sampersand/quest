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
	#[allow(non_snake_case)]
	#[inline]
	pub fn qs___inspect__(&self, _: Args) -> Result<Text, !> {
		Ok(Text::from(*self))
	}

	#[inline]
	pub fn qs_at_bool(&self, _: Args) -> Result<Boolean, !> {
		Ok(Boolean::from(*self))
	}

	#[inline]
	pub fn qs_at_list(&self, _: Args) -> Result<List, !> {
		Ok(List::from(*self))
	}

	#[inline]
	pub fn qs_at_num(&self, _: Args) -> Result<Number, !> {
		Ok(Number::from(*self))
	}

	#[inline]
	pub fn qs_at_text(&self, _: Args) -> Result<Text, !> {
		Ok(Text::from(*self))
	}

	#[inline]
	pub fn qs_call(&self, _: Args) -> Result<Null, !> {
		Ok(*self)
	}

	#[inline]
	pub fn qs_eql(&self, args: Args) -> Result<bool, crate::error::KeyError> {
		let rhs = args.arg(0)?;
		Ok(rhs.is_a::<Null>())
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
	"@text" => method_old Null::qs_at_text,
	"__inspect__" => method_old Null::qs___inspect__,
	"@bool" => method_old Null::qs_at_bool,
	"@list" => method_old Null::qs_at_list,
	"@num" => method_old Null::qs_at_num,
	"()" => method_old Null::qs_call,
	"==" => method_old Null::qs_eql,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn at_bool() {
		assert_eq!(Null.qs_at_bool(args!()).unwrap(), Boolean::FALSE);
	}

	#[test]
	fn at_num() {
		assert_eq!(Null.qs_at_num(args!()).unwrap(), Number::ZERO);
	}

	#[test]
	fn at_text() {
		assert_eq!(Null.qs_at_text(args!()).unwrap(), Text::new_static("null"));
	}

	dummy_object!(struct Dummy;);

	#[test]
	fn call() {
		assert_eq!(Null.qs_call(args!()).unwrap(), Null);
		assert_eq!(Null.qs_call(args!(Dummy)).unwrap(), Null);
		assert_eq!(Null.qs_call(args!(Dummy, Dummy)).unwrap(), Null);
	}

	#[test]
	fn eql() {
		assert_eq!(Null.qs_eql(args!(Dummy)).unwrap(), false);
		assert_eq!(Null.qs_eql(args!(Null)).unwrap(), true);
	}
}
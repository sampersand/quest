use crate::obj::{DataEnum, Mapping, Object, types::ObjectType};
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};

type BoolType = bool;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Boolean(BoolType);

impl Boolean {
	pub fn new<T: Into<BoolType>>(num: T) -> Self {
		Boolean(num.into())
	}
}

impl From<BoolType> for Boolean {
	fn from(inp: BoolType) -> Boolean {
		Boolean(inp)
	}
}

impl From<Boolean> for DataEnum {
	fn from(this: Boolean) -> DataEnum {
		DataEnum::Boolean(this)
	}
}

impl AsRef<BoolType> for Boolean {
	fn as_ref(&self) -> &BoolType {
		&self.0
	}
}

impl Object {
	pub fn call_into_bool(&self) -> Result<BoolType, Object> {
		self.call("@bool", &[])?.try_into_bool()
	}

	pub fn try_into_bool(&self) -> Result<BoolType, Object> {
		self.try_as_bool().map(Clone::clone)
	}

	pub fn into_bool(&self) -> Option<BoolType> {
		self.as_bool().map(Clone::clone)
	}

	pub fn try_as_bool(&self) -> Result<&BoolType, Object> {
		self.as_bool().ok_or_else(|| "not a bool".to_owned().into())
	}

	pub fn as_bool(&self) -> Option<&BoolType> {
		if let DataEnum::Boolean(ref t) = self.0.data {
			Some(t.as_ref())
		} else {
			None
		}
	}

}

impl From<BoolType> for Object {
	fn from(n: BoolType) -> Object {
		Boolean::from(n).into()
	}
}

impl Debug for Boolean {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			write!(f, "Boolean({:?})", self.0)
		} else {
			Debug::fmt(&self.0, f)
		}
	}
}


macro_rules! assert_this_is_bool {
	($args:expr) => {{
		let this = $args.get(0).unwrap();
		assert!(this.as_bool().is_some(), "bad `this` given: {:#?}", this);
	}};
}


impl_object_type!{for Boolean, super::Basic;
	"@num" => (|args| {
		assert_this_is_bool!(args);
		Ok(Number::from(
			if args.get(0)?.try_into_bool()? == true {
				1
			} else {
				0
			}
		).into())
	}),

	"@text" => (|args| {
		assert_this_is_bool!(args);
		if args.get(0)?.try_into_bool()? {
			Ok(Text::from("true").into())
		} else {
			Ok(Text::from("false").into())
		}
	}),

	"@bool" => (|args| {
		assert_this_is_bool!(args);
		args.get(0)?.call("clone", &[])
	}),

	"==" => (|args| {
		assert_this_is_bool!(args);
		Ok(Boolean::from(args.get(0)?.try_into_bool()? == args.get(1)?.try_into_bool()?).into())
	}),

	"clone" => (|args| {
		assert_this_is_bool!(args);
		Ok(Boolean::from(args.get(0)?.try_into_bool()?).into())
	}),

	"!" => (|args| {
		assert_this_is_bool!(args);
		Ok(Boolean::from(!args.get(0)?.try_into_bool()?).into())
	}),

	"&" => (|args| {
		assert_this_is_bool!(args);
		Ok(Boolean::from(args.get(0)?.try_into_bool()? & args.get(1)?.call_into_bool()?).into())
	}),

	"|" => (|args| {
		assert_this_is_bool!(args);
		Ok(Boolean::from(args.get(0)?.try_into_bool()? | args.get(1)?.call_into_bool()?).into())
	}),

	"^" => (|args| {
		assert_this_is_bool!(args);
		Ok(Boolean::from(args.get(0)?.try_into_bool()? ^ args.get(1)?.call_into_bool()?).into())
	})
}
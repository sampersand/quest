use crate::obj::{self, DataEnum, Mapping, Object, types::ObjectType};
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};

type BoolType = bool;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Boolean(BoolType);

impl Boolean {
	pub fn new<T: Into<BoolType>>(num: T) -> Self {
		Boolean(num.into())
	}

	pub fn into_inner(self) -> BoolType { 
		self.0
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

impl Object {
	pub fn try_call_into_bool(&self) -> obj::Result<bool> {
		self.call("@bool", &[]).map(|x| x.downcast_clone::<Boolean>().map(|x| x.into_inner()).unwrap_or(false))
	}
}


impl_object_type!{for Boolean, super::Basic;
	"@num" => (|args| {
		Ok(Number::from(args.this::<Boolean>()?.into_inner()).into())
	}),

	"@text" => (|args| {
		Ok(Text::from(args.this::<Boolean>()?.into_inner().to_string()).into())
	}),

	"@bool" => (|args| {
		args.this_obj::<Boolean>()?.call("clone", &[])
	}),

	"==" => (|args| {
		Ok(Boolean::from(
			args.this::<Boolean>()?.into_inner() ==
			args.get(1)?.try_downcast_ref::<Boolean>()?.into_inner()
		).into())
	}),

	"clone" => (|args| {
		Ok(args.this::<Boolean>()?.clone().into())
	}),

	"!" => (|args| {
		Ok(Boolean::from(!args.this::<Boolean>()?.into_inner()).into())
	}),

	"&" => (|args| {
		Ok(Boolean::from(
			args.this::<Boolean>()?.into_inner() &
			args.get(1)?.call("@bool", &[])?.try_downcast_ref::<Boolean>()?.into_inner()).into())
	}),

	"|" => (|args| {
		Ok(Boolean::from(args.this::<Boolean>()?.into_inner() | args.get(1)?.call("@bool", &[])?.try_downcast_ref::<Boolean>()?.into_inner()).into())
	}),

	"^" => (|args| {
		Ok(Boolean::from(args.this::<Boolean>()?.into_inner() ^ args.get(1)?.call("@bool", &[])?.try_downcast_ref::<Boolean>()?.into_inner()).into())
	})
}
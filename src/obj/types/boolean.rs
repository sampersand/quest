use crate::obj::{self, Object, types::ObjectType};
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Boolean(bool);

impl Boolean {
	pub fn new(t: bool) -> Self {
		Boolean(t)
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
		Ok(self.call("@bool", &[])?.downcast_clone::<Boolean>().map(|x| x.0).unwrap_or(false))
	}
}

impl From<bool> for Object {
	fn from(inp: bool) -> Self {
		Boolean::from(inp).into()
	}
}

impl From<bool> for Boolean {
	fn from(b: bool) -> Self {
		Boolean::new(b)
	}
}

impl From<Boolean> for bool {
	fn from(b: Boolean) -> Self {
		b.0
	}
}

impl AsRef<bool> for Boolean {
	fn as_ref(&self) -> &bool {
		&self.0
	}
}


impl_object_type!{for Boolean, super::Basic;
	"@num" => (|args| {
		Ok(Number::from(args.this::<Boolean>()?.0).into())
	}),

	"@text" => (|args| {
		Ok(args.this::<Boolean>()?.0.to_string().into())
	}),

	"@bool" => (|args| {
		args.this_obj::<Boolean>()?.call("clone", &[])
	}),

	"==" => (|args| {
		if let Some(rhs) = args.get(1)?.downcast_ref::<Boolean>() {
			Ok((args.this::<Boolean>()?.0 == rhs.0).into())
		} else {
			Ok(false.into())
		}
	}),

	"clone" => (|args| {
		Ok(args.this::<Boolean>()?.0.into())
	}),

	"!" => (|args| {
		Ok((!args.this::<Boolean>()?.0).into())
	}),

	"&" => (|args| {
		Ok(Boolean::from(
			args.this::<Boolean>()?.0
			& args.get(1)?.try_call_into_bool()?
		).into())
	}),

	"|" => (|args| {
		Ok(Boolean::from(
			args.this::<Boolean>()?.0
			| args.get(1)?.try_call_into_bool()?
		).into())
	}),

	"^" => (|args| {
		Ok(Boolean::from(
			args.this::<Boolean>()?.0
			^ args.get(1)?.try_call_into_bool()?
		).into())
	})
}
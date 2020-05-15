use crate::obj::{self, Object, types::ObjectType};
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Display, Formatter};
use std::ops;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Boolean(bool);

pub const FALSE: Boolean = Boolean::new(false);
pub const TRUE: Boolean = Boolean::new(true);

impl Boolean {
	pub const fn new(t: bool) -> Self {
		Boolean(t)
	}
}

impl Debug for Boolean {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			write!(f, "Boolean({:?})", self.0)
		} else {
			Display::fmt(self, f)
		}
	}
}

impl Display for Boolean {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
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

impl ops::Not for Boolean {
	type Output = Self;
	fn not(self) -> Self::Output {
		Boolean::new(!self.0)
	}
}

impl ops::BitAnd for Boolean {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self::Output {
		Boolean::new(self.0 & rhs.0)
	}
}

impl ops::BitOr for Boolean {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self::Output {
		Boolean::new(self.0 | rhs.0)
	}
}

impl ops::BitXor for Boolean {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self::Output {
		Boolean::new(self.0 ^ rhs.0)
	}
}

impl From<Boolean> for obj::types::Number {
	fn from(b: Boolean) -> Self {
		if b.0 == true {
			obj::types::number::ONE
		} else {
			obj::types::number::ZERO
		}
	}
}

impl From<Boolean> for obj::types::Text {
	fn from(b: Boolean) -> Self {
		obj::types::Text::from(if b.0 { "true" } else { "false" })
	}
}




impl_object_type!{for Boolean, super::Basic;
	"@num" => (|args| {
		Ok(Number::from(*args.this::<Boolean>()?).into())
	}),

	"@text" => (|args| {
		Ok(Text::from(*args.this::<Boolean>()?).into())
	}),

	"@bool" => (|args| {
		args.this_obj::<Boolean>()?.call("clone", args.new_same_binding(&[] as &[_]))
	}),

	"==" => (|args| {
		if let Some(ref rhs) = args.get(1)?.downcast_ref::<Boolean>() {
			Ok(args.this::<Boolean>()?.eq(rhs).into())
		} else {
			Ok(false.into())
		}
	}),

	"clone" => (|args| {
		Ok(args.this::<Boolean>()?.clone().into())
	}),

	"!" => (|args| {
		use std::ops::Not;
		Ok(args.this::<Boolean>()?.not().into())
	}),

	"&" => (|args| {
		use std::ops::BitAnd;
		Ok(args.this::<Boolean>()?.bitand(*getarg!(Boolean; args)).into())
	}),

	"|" => (|args| {
		use std::ops::BitOr;
		Ok(args.this::<Boolean>()?.bitor(*getarg!(Boolean; args)).into())
	}),

	"^" => (|args| {
		use std::ops::BitXor;
		Ok(args.this::<Boolean>()?.bitxor(*getarg!(Boolean; args)).into())
	})
}
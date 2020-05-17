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

mod impls {
	use super::*;
	use std::ops::{Deref, BitAnd, BitOr, BitXor, Not};
	use crate::obj::{Object, Result, Args, types};

	fn call_into_boolean<'a>(args: &'a Args, index: usize) -> Result<Boolean> {
		args.get(index)?
			.call("@bool", args.new_args_slice(&[]))?
			.downcast_clone::<Boolean>()
			.ok_or_else(|| format!("argument {} is not a boolean", index).into())
	}

	pub fn at_num(args: Args) -> Result<Object> {
		println!("at num");
		Ok(types::Number::from(*args.this_downcast::<Boolean>()?).into())
	}

	pub fn at_text(args: Args) -> Result<Object> {
		Ok(types::Text::from(*args.this_downcast::<Boolean>()?).into())
	}

	pub fn at_bool(args: Args) -> Result<Object> {
		let this = args.this()?;
		debug_assert!(this.is_a::<Boolean>(), "bad `this` given");
		this.call("clone", args.get_rng(1..)?)
	}

	pub fn clone(args: Args) -> Result<Object> {
		Ok(args.this_downcast::<Boolean>()?.clone().into())
	}

	pub fn eql(args: Args) -> Result<Object> {
		if let Ok(rhs) = args.get_downcast::<Boolean>(1) {
			Ok((*args._this_downcast::<Boolean>()? == *rhs).into())
		} else {
			Ok(FALSE.into())
		}
	}

	pub fn not(args: Args) -> Result<Object> {
		Ok(args.this_downcast::<Boolean>()?.not().into())
	}

	pub fn bitand(args: Args) -> Result<Object> {
		Ok(args.this_downcast::<Boolean>()?.bitand(args.arg_call_into::<Boolean>(0)?).into())
	}

	pub fn bitor(args: Args) -> Result<Object> {
		println!("args: {:?}", args);
		Ok(args.this_downcast::<Boolean>()?.bitor(args.arg_call_into::<Boolean>(0)?).into())
	}

	pub fn bitxor(args: Args) -> Result<Object> {
		Ok(args.this_downcast::<Boolean>()?.bitxor(args.arg_call_into::<Boolean>(0)?).into())
	}

	pub fn cmp(args: Args) -> Result<Object> {
		todo!("cmp for Boolean")
	}

	pub fn hash(args: Args) -> Result<Object> {
		todo!("hash for Boolean")
	}
}

impl_object_type!{
	for Boolean [(convert "@bool")]:
	"@num"  => impls::at_num,
	"@text" => impls::at_text,
	"@bool" => impls::at_bool,
	"clone" => impls::clone,
	"=="    => impls::eql,
	"!"     => impls::not,
	"&"     => impls::bitand,
	"|"     => impls::bitor,
	"^"     => impls::bitxor,
	"<=>"   => impls::cmp,
	"hash"  => impls::hash,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn at_num() {
		assert_call_eq!(for Boolean;
			types::number::ONE, at_num(TRUE) -> Number,
			types::number::ZERO, at_num(FALSE) -> Number
		);
	}

	#[test]
	fn at_text() {
		assert_call_eq!(for Boolean;
			Text::from("true"), at_text(TRUE) -> Text,
			Text::from("false"), at_text(FALSE) -> Text
		);
	}

	#[test]
	fn at_bool() {
		assert_call_eq!(for Boolean;
			TRUE, at_bool(TRUE) -> Boolean,
			FALSE, at_bool(FALSE) -> Boolean
		);
	}

	#[test]
	fn clone() {
		assert_call_eq!(for Boolean;
			TRUE, at_bool(TRUE) -> Boolean,
			FALSE, at_bool(FALSE) -> Boolean
		);
	}

	#[test]
	fn eql() {
		assert_call_eq!(for Boolean;
			TRUE, eql(TRUE, TRUE) -> Boolean,
			FALSE, eql(TRUE, FALSE) -> Boolean,
			FALSE, eql(FALSE, TRUE) -> Boolean,
			TRUE, eql(FALSE, FALSE) -> Boolean
		);
	}

	#[test]
	fn not() {
		assert_call_eq!(for Boolean;
			FALSE, not(TRUE) -> Boolean,
			TRUE, not(FALSE) -> Boolean
		);
	}

	#[test]
	fn bitand() {
		assert_call_eq!(for Boolean;
			TRUE, bitand(TRUE, TRUE) -> Boolean,
			FALSE, bitand(TRUE, FALSE) -> Boolean,
			FALSE, bitand(FALSE, TRUE) -> Boolean,
			FALSE, bitand(FALSE, FALSE) -> Boolean
		);
	}

	#[test]
	fn bitor() {
		assert_call_eq!(for Boolean;
			TRUE, bitor(TRUE, TRUE) -> Boolean,
			TRUE, bitor(TRUE, FALSE) -> Boolean,
			TRUE, bitor(FALSE, TRUE) -> Boolean,
			FALSE, bitor(FALSE, FALSE) -> Boolean
		);
	}

	#[test]
	fn bitxor() {
		assert_call_eq!(for Boolean;
			FALSE, bitxor(TRUE, TRUE) -> Boolean,
			TRUE, bitxor(TRUE, FALSE) -> Boolean,
			TRUE, bitxor(FALSE, TRUE) -> Boolean,
			FALSE, bitxor(FALSE, FALSE) -> Boolean
		);
	}

	#[test]
	#[ignore]
	fn cmp() { todo!(); }

	#[test]
	#[ignore]
	fn hash() { todo!(); }
}
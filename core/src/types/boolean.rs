use crate::{Object, Args};
use crate::types::{Number, Text};
use std::fmt::{self, Debug, Display, Formatter};

/// The Boolean type within Quest.
///
/// Internally, this is simply a newtype wrapping a `bool`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Boolean(bool);

impl Boolean {
	/// A constant representing the boolean value "false".
	pub const FALSE: Boolean = Boolean::new(false);

	/// A constant representing the boolean value "true".
	pub const TRUE: Boolean = Boolean::new(true);

	/// Simply create a new [`Boolean`].
	#[inline]
	pub const fn new(b: bool) -> Self {
		Boolean(b)
	}

	/// Unwraps the value.
	#[inline]
	pub fn into_inner(self) -> bool {
		self.0
	}
}

impl PartialEq<bool> for Boolean {
	#[inline]
	fn eq(&self, rhs: &bool) -> bool {
		self.0 == *rhs
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
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl From<bool> for Object {
	/// Converts this into a [`Boolean`] and then into an [`Object`]
	#[inline]
	fn from(inp: bool) -> Self {
		Boolean::new(inp).into()
	}
}

impl From<bool> for Boolean {
	#[inline]
	fn from(b: bool) -> Self {
		Boolean::new(b)
	}
}

impl From<Boolean> for bool {
	#[inline]
	fn from(b: Boolean) -> Self {
		b.into_inner()
	}
}

impl AsRef<bool> for Boolean {
	#[inline]
	fn as_ref(&self) -> &bool {
		&self.0
	}
}

impl From<Boolean> for Number {
	/// Convert to a [`Number`] by mapping `true` to [`Number::ONE`] and `false` to
	/// [`Number::ZERO`]
	#[inline]
	fn from(b: Boolean) -> Self {
		match b.into_inner() {
			true => Number::ONE,
			false => Number::ZERO
		}
	}
}

impl From<Boolean> for Text {
	/// Convert to a [`Text`] by mapping `true` to `"true"` and `false` to `"false"`
	#[inline]
	fn from(b: Boolean) -> Self {
		const TRUE_TEXT: Text = Text::new_static("true");
		const FALSE_TEXT: Text = Text::new_static("false");
		match b.into_inner() {
			true => TRUE_TEXT,
			false => FALSE_TEXT
		}
	}
}

impl std::ops::BitAnd for Boolean {
	type Output = Self;

	#[inline]
	fn bitand(self, rhs: Self) -> Self {
		Self::from(self.0 & rhs.0)
	}
}

impl std::ops::BitAndAssign for Boolean {
	#[inline]
	fn bitand_assign(&mut self, rhs: Self) {
		self.0 &= rhs.0;
	}
}

impl std::ops::BitOr for Boolean {
	type Output = Self;

	#[inline]
	fn bitor(self, rhs: Self) -> Self {
		Self::from(self.0 | rhs.0)
	}
}

impl std::ops::BitOrAssign for Boolean {
	#[inline]
	fn bitor_assign(&mut self, rhs: Self) {
		self.0 |= rhs.0;
	}
}

impl std::ops::BitXor for Boolean {
	type Output = Self;

	#[inline]
	fn bitxor(self, rhs: Self) -> Self {
		Self::from(self.0 ^ rhs.0)
	}
}

impl std::ops::BitXorAssign for Boolean {
	#[inline]
	fn bitxor_assign(&mut self, rhs: Self) {
		self.0 ^= rhs.0;
	}
}

impl std::ops::Not for Boolean {
	type Output = Self;

	#[inline]
	fn not(self) -> Self {
		Self::from(!self.0)
	}
}


impl Boolean {
	/// Convert this into a [`Number`].
	///
	/// This is simply a wrapper around [`Number::from(Boolean)`](Number#impl-From<Boolean>).
	#[inline]
	pub fn qs_at_num(&self, _: Args) -> Result<Number, !> {
		Ok(Number::from(*self))
	}

	/// Convert this into a [`Text`].
	///
	/// This is simply a wrapper around [`Text::from(Boolean)`](Number#impl-From<Boolean>).
	#[inline]
	pub fn qs_at_text(&self, _: Args) -> Result<Text, !> {
		Ok(Text::from(*self))
	}

	/// Convert this into a [`Boolean`].
	///
	/// This is simply a wrapper around [`Boolean::from(Boolean)`](Boolean#impl-From<Boolean>).
	#[inline]
	pub fn qs_at_bool(&self, _: Args) -> Result<Boolean, !> {
		Ok(Boolean::from(*self))
	}

	/// Clones this.
	#[inline]
	pub fn qs_clone(&self, _: Args) -> Result<Boolean, !> {
		Ok(self.clone())
	}

	/// See if a this is equal to the first argument.
	///
	/// Unlike most methods, the first argument is not implicitly converted to a  [`Boolean`] first.
	pub fn qs_eql(&self, args: Args) -> Result<bool, crate::error::KeyError> {
		match args.arg(0)?.downcast_ref::<Boolean>() {
			Some(val) if *self == *val => Ok(true),
			_ => Ok(false)
		}
	}

	/// Compares this to the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	pub fn qs_cmp(&self, args: Args) -> crate::Result<std::cmp::Ordering> {
		let rhs = args.arg(0)?.downcast_call::<Boolean>()?;

		Ok(self.cmp(&rhs))
	}

	/// Logical NOT of this.
	#[inline]
	pub fn qs_not(&self, _: Args) -> Result<Boolean, !> {
		Ok(!*self)
	}

	/// Logical AND of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	pub fn qs_bitand(&self, args: Args) -> crate::Result<Boolean> {
		let rhs = args.arg(0)?.downcast_call::<Boolean>()?;

		Ok(*self & rhs)
	}

	/// In-place logical AND of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	pub fn qs_bitand_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.downcast_call::<Boolean>()?;

		*this.try_downcast_mut::<Boolean>()? &= rhs;

		Ok(this.clone())
	}

	/// Logical OR of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	pub fn qs_bitor(&self, args: Args) -> crate::Result<Boolean> {
		let rhs = args.arg(0)?.downcast_call::<Boolean>()?;

		Ok(*self | rhs)
	}

	/// In-place logical OR of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	pub fn qs_bitor_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.downcast_call::<Boolean>()?;

		*this.try_downcast_mut::<Boolean>()? |= rhs;

		Ok(this.clone())
	}

	/// Logical XOR of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	pub fn qs_bitxor(&self, args: Args) -> crate::Result<Boolean> {
		let rhs = args.arg(0)?.downcast_call::<Boolean>()?;

		Ok(*self ^ rhs)
	}

	/// In-place logical XOR of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	pub fn qs_bitxor_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.downcast_call::<Boolean>()?;

		*this.try_downcast_mut::<Boolean>()? ^= rhs;

		Ok(this.clone())
	}

	/// The hash for this.
	#[inline]
	pub fn qs_hash(&self, _args: Args) -> crate::Result<Object> {
		todo!("hash for Boolean")
	}
}


impl_object_type!{
for Boolean [(parents super::Basic) (convert "@bool")]:
	"@num"  => method Boolean::qs_at_num,
	"@text" => method Boolean::qs_at_text,
	"@bool" => method Boolean::qs_at_bool,
	"clone" => method Boolean::qs_clone,
	"=="    => method Boolean::qs_eql,
	"!"     => method Boolean::qs_not,
	"&"     => method Boolean::qs_bitand,
	"&="    => function Boolean::qs_bitand_assign,
	"|"     => method Boolean::qs_bitor,
	"|="    => function Boolean::qs_bitor_assign,
	"^"     => method Boolean::qs_bitxor,
	"^="    => function Boolean::qs_bitxor_assign,
	"<=>"   => method Boolean::qs_cmp,
	"hash"  => method Boolean::qs_hash,
}

#[cfg(test)]
mod tests {
	use super::*;


	#[test]
	fn at_num() {
		assert_eq!(Boolean::TRUE.qs_at_num(args!()).unwrap(), Number::ONE);
		assert_eq!(Boolean::FALSE.qs_at_num(args!()).unwrap(), Number::ZERO);
	}

	#[test]
	fn at_text() {
		assert_eq!(Boolean::TRUE.qs_at_text(args!()).unwrap(), Text::from("true"));
		assert_eq!(Boolean::FALSE.qs_at_text(args!()).unwrap(), Text::from("false"));
	}

	#[test]
	fn at_bool() {
		assert_eq!(Boolean::TRUE.qs_at_bool(args!()).unwrap(), Boolean::TRUE);
		assert_eq!(Boolean::FALSE.qs_at_bool(args!()).unwrap(), Boolean::FALSE);
	}

	#[test]
	fn clone() {
		assert_eq!(Boolean::TRUE.qs_clone(args!()).unwrap(), Boolean::TRUE);
		assert_eq!(Boolean::FALSE.qs_clone(args!()).unwrap(), Boolean::FALSE);
	}

	#[test]
	fn eql() {
		assert_eq!(Boolean::TRUE.qs_eql(args!(true)).unwrap(), true);
		assert_eq!(Boolean::TRUE.qs_eql(args!(false)).unwrap(), false);
		assert_eq!(Boolean::FALSE.qs_eql(args!(true)).unwrap(), false);
		assert_eq!(Boolean::FALSE.qs_eql(args!(false)).unwrap(), true);
	}

	#[test]
	fn not() {
		assert_eq!(Boolean::TRUE.qs_not(args!()).unwrap(), Boolean::FALSE);
		assert_eq!(Boolean::FALSE.qs_not(args!()).unwrap(), Boolean::TRUE);
	}

	#[test]
	fn bitand() {
		assert_eq!(Boolean::TRUE.qs_bitand(args!(true)).unwrap(), Boolean::TRUE);
		assert_eq!(Boolean::TRUE.qs_bitand(args!(false)).unwrap(), Boolean::FALSE);
		assert_eq!(Boolean::FALSE.qs_bitand(args!(true)).unwrap(), Boolean::FALSE);
		assert_eq!(Boolean::FALSE.qs_bitand(args!(false)).unwrap(), Boolean::FALSE);
	}

	#[test]
	#[ignore]
	fn bitand_assign() { todo!() }

	#[test]
	fn bitor() {
		assert_eq!(Boolean::TRUE.qs_bitor(args!(true)).unwrap(), Boolean::TRUE);
		assert_eq!(Boolean::TRUE.qs_bitor(args!(false)).unwrap(), Boolean::TRUE);
		assert_eq!(Boolean::FALSE.qs_bitor(args!(true)).unwrap(), Boolean::TRUE);
		assert_eq!(Boolean::FALSE.qs_bitor(args!(false)).unwrap(), Boolean::FALSE);
	}

	#[test]
	#[ignore]
	fn bitor_assign() { todo!() }

	#[test]
	fn bitxor() {
		assert_eq!(Boolean::TRUE.qs_bitxor(args!(true)).unwrap(), Boolean::FALSE);
		assert_eq!(Boolean::TRUE.qs_bitxor(args!(false)).unwrap(), Boolean::TRUE);
		assert_eq!(Boolean::FALSE.qs_bitxor(args!(true)).unwrap(), Boolean::TRUE);
		assert_eq!(Boolean::FALSE.qs_bitxor(args!(false)).unwrap(), Boolean::FALSE);
	}

	#[test]
	#[ignore]
	fn bitxor_assign() { todo!() }


	#[test]
	#[ignore]
	fn cmp() { todo!(); }

	#[test]
	#[ignore]
	fn hash() { todo!(); }
}
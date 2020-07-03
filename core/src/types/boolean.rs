use crate::{Object, Result, Args};
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
	pub const fn into_inner(self) -> bool {
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

impl AsMut<bool> for Boolean {
	#[inline]
	fn as_mut(&mut self) -> &mut bool {
		&mut self.0
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
	/// Inspect the boolean.
	#[allow(non_snake_case)]
	pub fn qs___inspect__(this: &Object, _: Args) -> Result<Text> {
		this.try_downcast_and_then::<Self, _, !, _>(|bool| Ok(format!("{:?}", bool).into()))
	}

	/// Convert this into a [`Number`].
	///
	/// This is simply a wrapper around [`Number::from(Boolean)`](Number#impl-From<Boolean>).
	#[inline]
	pub fn qs_at_num(this: &Object, _: Args) -> Result<Number> {
		this.try_downcast_and_then::<Self, _, !, _>(|bool| Ok(Number::from(*bool)))
	}

	/// Convert this into a [`Text`].
	///
	/// This is simply a wrapper around [`Text::from(Boolean)`](Number#impl-From<Boolean>).
	#[inline]
	pub fn qs_at_text(this: &Object, _: Args) -> Result<Text> {
		this.try_downcast_and_then::<Self, _, !, _>(|bool| Ok(Text::from(*bool)))
	}

	/// Convert this into a [`Boolean`].
	///
	/// This is simply a wrapper around [`Boolean::clone`](#method.clone).
	#[inline]
	pub fn qs_at_bool(this: &Object, _: Args) -> Result<Object> {
		Ok(this.clone())
	}

	/// See if a this is equal to the first argument.
	///
	/// Unlike most methods, the first argument is not implicitly converted to a  [`Boolean`] first.
	pub fn qs_eql(this: &Object, args: Args) -> Result<Boolean> {
		let rhs = args.arg(0)?;

		if this.is_identical(rhs) {
			Ok(Boolean::new(true))
		} else {
			this.try_downcast_and_then::<Self, _, !, _>(|lhs| {
				Ok(rhs.downcast_and_then::<Self, _, _>(|rhs| lhs == rhs).unwrap_or(false).into())
			})
		}
	}

	/// Compares this to the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	pub fn qs_cmp(this: &Object, args: Args) -> Result<std::cmp::Ordering> {
		let rhs = args.arg(0)?;

		this.try_downcast_and_then::<Self, _, _, _>(|lhs| {
			rhs.call_downcast_map(|rhs| lhs.cmp(rhs))
		})
	}

	/// Logical NOT of this.
	#[inline]
	pub fn qs_not(&self, _: Args) -> std::result::Result<Boolean, !> {
		Ok(!*self)
	}

	/// Logical AND of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	pub fn qs_bitand(&self, args: Args) -> Result<Boolean> {
		let rhs = args.arg(0)?.call_downcast_map(Self::clone)?;

		Ok(*self & rhs)
	}

	/// In-place logical AND of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	pub fn qs_bitand_assign(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_mut_map(|bool: &mut Self| *bool &= rhs)?;

		Ok(this.clone())
	}

	/// Logical OR of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	pub fn qs_bitor(&self, args: Args) -> Result<Boolean> {
		let rhs = args.arg(0)?.call_downcast_map(Self::clone)?;

		Ok(*self | rhs)
	}

	/// In-place logical OR of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	pub fn qs_bitor_assign(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_mut_map(|bool: &mut Self| *bool |= rhs)?;

		Ok(this.clone())
	}

	/// Logical XOR of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	pub fn qs_bitxor(&self, args: Args) -> Result<Boolean> {
		let rhs = args.arg(0)?.call_downcast_map(Self::clone)?;

		Ok(*self ^ rhs)
	}

	/// In-place logical XOR of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	pub fn qs_bitxor_assign(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_mut_map(|bool: &mut Self| *bool ^= rhs)?;

		Ok(this.clone())
	}

	/// The hash for this.
	#[inline]
	pub fn qs_hash(&self, _args: Args) -> Result<Object> {
		todo!("hash for Boolean")
	}
}


impl_object_type!{
for Boolean {
	#[inline]
	fn new_object(self) -> Object where Self: Sized {
		use lazy_static::lazy_static;
		use crate::types::ObjectType;

		lazy_static! {
			static ref TRUE: Object = Object::new_with_parent(Boolean::TRUE, vec![Boolean::mapping()]);
			static ref FALSE: Object = Object::new_with_parent(Boolean::FALSE, vec![Boolean::mapping()]);
		}

		if self.into_inner() { 
			TRUE.deep_clone()
		} else {
			FALSE.deep_clone()
		}
	}
}
[(parents super::Basic) (convert "@bool")]:
	"@text" => function Boolean::qs_at_text,
	"__inspect__" => function Boolean::qs___inspect__,
	"@num"  => function Boolean::qs_at_num,
	"@bool" => function Boolean::qs_at_bool,
	"=="    => function Boolean::qs_eql,
	"!"     => method_old Boolean::qs_not,
	"&"     => method_old Boolean::qs_bitand,
	"&="    => function Boolean::qs_bitand_assign,
	"|"     => method_old Boolean::qs_bitor,
	"|="    => function Boolean::qs_bitor_assign,
	"^"     => method_old Boolean::qs_bitxor,
	"^="    => function Boolean::qs_bitxor_assign,
	"<=>"   => function Boolean::qs_cmp,
	"hash"  => method_old Boolean::qs_hash,
}

#[cfg(test)]
mod tests {
	use super::*;


	#[test]
	fn at_num() {
		assert_eq!(Boolean::qs_at_num(&true.into(), args!()).unwrap(), Number::ONE);
		assert_eq!(Boolean::qs_at_num(&false.into(), args!()).unwrap(), Number::ZERO);
	}

	#[test]
	fn at_text() {
		assert_eq!(Boolean::qs_at_text(&Object::from(Boolean::TRUE), args!()).unwrap(), Text::from("true"));
		assert_eq!(Boolean::qs_at_text(&Object::from(Boolean::FALSE), args!()).unwrap(), Text::from("false"));
	}

	#[test]
	fn at_bool() {
		assert_eq!(Boolean::qs_at_bool(&true.into(), args!()).unwrap().downcast_and_then(Boolean::clone).unwrap(), Boolean::TRUE);
		assert_eq!(Boolean::qs_at_bool(&false.into(), args!()).unwrap().downcast_and_then(Boolean::clone).unwrap(), Boolean::FALSE);
	}

	#[test]
	fn eql() {
		assert_eq!(Boolean::qs_eql(&true.into(), args!(true)).unwrap(), true);
		assert_eq!(Boolean::qs_eql(&true.into(), args!(false)).unwrap(), false);
		assert_eq!(Boolean::qs_eql(&false.into(), args!(true)).unwrap(), false);
		assert_eq!(Boolean::qs_eql(&false.into(), args!(false)).unwrap(), true);
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
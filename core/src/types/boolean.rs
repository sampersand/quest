use crate::{Object, Result, Args};
use crate::types::{Number, Text};
use std::fmt::{self, Debug, Display, Formatter};
use std::ops;

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

impl ops::BitAnd for Boolean {
	type Output = Self;

	#[inline]
	fn bitand(self, rhs: Self) -> Self {
		Self::from(self.0 & rhs.0)
	}
}

impl ops::BitAndAssign for Boolean {
	#[inline]
	fn bitand_assign(&mut self, rhs: Self) {
		self.0 &= rhs.0;
	}
}

impl ops::BitOr for Boolean {
	type Output = Self;

	#[inline]
	fn bitor(self, rhs: Self) -> Self {
		Self::from(self.0 | rhs.0)
	}
}

impl ops::BitOrAssign for Boolean {
	#[inline]
	fn bitor_assign(&mut self, rhs: Self) {
		self.0 |= rhs.0;
	}
}

impl ops::BitXor for Boolean {
	type Output = Self;

	#[inline]
	fn bitxor(self, rhs: Self) -> Self {
		Self::from(self.0 ^ rhs.0)
	}
}

impl ops::BitXorAssign for Boolean {
	#[inline]
	fn bitxor_assign(&mut self, rhs: Self) {
		self.0 ^= rhs.0;
	}
}

impl ops::Not for Boolean {
	type Output = Self;

	#[inline]
	fn not(self) -> Self {
		Self::from(!self.0)
	}
}

impl Boolean {
	/// Inspects the [`Boolean`].
	///
	/// # Quest Examples
	/// ```quest
	/// assert(true.$__inspect__() == "true");
	/// assert(false.$__inspect__() == "false");
	/// ```
	#[allow(non_snake_case)]
	#[inline]
	pub fn qs___inspect__(this: &Object, _: Args) -> Result<Text> {
		this.try_downcast_map(|this: &Self| format!("{:?}", this).into())
	}

	/// Convert this into a [`Number`].
	///
	/// [`true`](Boolean::TRUE) becomes [`1`](Number::ONE) and [`false`](Boolean::FALSE) becomes
	/// [`0`](Number::ZERO)
	///
	/// # Quest Examples
	/// ```quest
	/// assert(1 + true == 2);
	/// assert(99 * false == 0);
	/// ```
	#[inline]
	pub fn qs_at_num(this: &Object, _: Args) -> Result<Number> {
		this.try_downcast_map(|this: &Self| Number::from(*this))
	}

	/// Convert this into a [`Text`].
	///
	/// # Quest Examples
	/// ```quest
	/// assert("are dogs cool? " + true == "are dogs cool? true");
	/// assert("? " + true == "are dogs cool? true");
	/// ```
	#[inline]
	pub fn qs_at_text(this: &Object, _: Args) -> Result<Text> {
		this.try_downcast_map(|this: &Self| Text::from(*this))
	}

	/// Convert this into a [`Boolean`].
	///
	/// # Quest Examples
	/// ```quest
	/// 
	/// ```
	#[inline]
	pub fn qs_at_bool(this: &Object, _: Args) -> Result<Object> {
		Ok(this.clone())
	}

	/// See if a this is equal to the first argument.
	///
	/// Unlike most methods, the first argument is not implicitly converted to a  [`Boolean`] first.
	///
	/// # Arguments
	/// 
	/// # Quest Examples
	/// ```quest
	/// 
	/// ```
	#[inline]
	pub fn qs_eql(this: &Object, args: Args) -> Result<Boolean> {
		let rhs = args.arg(0)?;

		this.try_downcast_map(|lhs: &Self| {
			rhs.downcast_and_then(|rhs: &Self| lhs == rhs).unwrap_or(false).into()
		})
	}

	/// Compares this to the first argument.
	///
	/// The first eargument is converted to a [`Boolean`] if it isn't already.
	///
	/// # Arguments
	/// 
	/// # Quest Examples
	/// ```quest
	/// 
	/// ```
	#[inline]
	pub fn qs_cmp(this: &Object, args: Args) -> Result<std::cmp::Ordering> {
		let rhs = args.arg(0)?;

		this.try_downcast_and_then(|lhs: &Self| {
			rhs.call_downcast_map(|rhs: &Self| lhs.cmp(rhs))
		})
	}

	/// Logical NOT of this.
	///
	/// # Quest Examples
	/// ```quest
	/// 
	/// ```
	#[inline]
	pub fn qs_not(this: &Object, _: Args) -> Result<Object> {
		this.try_downcast_map(|this: &Self| (!*this).into())
	}

	/// Logical AND of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	///
	/// # Arguments
	/// 
	/// # Quest Examples
	/// ```quest
	/// 
	/// ```
	#[inline]
	pub fn qs_bitand(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_map(|this: &Self| (*this & rhs).into())
	}

	/// In-place logical AND of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	///
	/// # Arguments
	/// 
	/// # Quest Examples
	/// ```quest
	/// 
	/// ```
	#[inline]
	pub fn qs_bitand_assign(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_mut_map(|this: &mut Self| *this &= rhs)
			.map(|_| this.clone())
	}

	/// Logical OR of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	///
	/// # Arguments
	/// 
	/// # Quest Examples
	/// ```quest
	/// 
	/// ```
	#[inline]
	pub fn qs_bitor(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_map(|this: &Self| (*this | rhs).into())
	}

	/// In-place logical OR of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	///
	/// # Arguments
	/// 
	/// # Quest Examples
	/// ```quest
	/// 
	/// ```
	#[inline]
	pub fn qs_bitor_assign(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_mut_map(|this: &mut Self| *this |= rhs)
			.map(|_| this.clone())
	}

	/// Logical XOR of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	///
	/// # Arguments
	/// 
	/// # Quest Examples
	/// ```quest
	/// 
	/// ```
	#[inline]
	pub fn qs_bitxor(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_map(|this: &Self| (*this ^ rhs).into())
	}

	/// In-place logical XOR of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	///
	/// # Arguments
	/// 
	/// # Quest Examples
	/// ```quest
	/// 
	/// ```
	#[inline]
	pub fn qs_bitxor_assign(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_mut_map(|this: &mut Self| *this ^= rhs)
			.map(|_| this.clone())
	}

	/// The hash for this.
	///
	/// # Arguments
	/// 
	/// # Quest Examples
	/// ```quest
	/// 
	/// ```
	#[inline]
	pub fn qs_hash(_this: &Object, _args: Args) -> Result<Object> {
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
			static ref TRUE: Object = Object::new_with_parent(Boolean::TRUE,
				vec![Boolean::mapping()]);

			static ref FALSE: Object = Object::new_with_parent(Boolean::FALSE,
				vec![Boolean::mapping()]);
		}

		match self.into_inner() { 
			true => TRUE.deep_clone(),
			false => FALSE.deep_clone()
		}
	}
}
[(parents super::Basic) (convert "@bool")]:
	"@text" => function Boolean::qs_at_text,
	"__inspect__" => function Boolean::qs___inspect__,
	"@num"  => function Boolean::qs_at_num,
	"@bool" => function Boolean::qs_at_bool,
	"=="    => function Boolean::qs_eql,
	"!"     => function Boolean::qs_not,
	"&"     => function Boolean::qs_bitand,
	"&="    => function Boolean::qs_bitand_assign,
	"|"     => function Boolean::qs_bitor,
	"|="    => function Boolean::qs_bitor_assign,
	"^"     => function Boolean::qs_bitxor,
	"^="    => function Boolean::qs_bitxor_assign,
	"<=>"   => function Boolean::qs_cmp,
	"hash"  => function Boolean::qs_hash,
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
		assert_downcast_eq!(Boolean; Boolean::qs_not(&true.into(), args!()), false);
		assert_downcast_eq!(Boolean; Boolean::qs_not(&false.into(), args!()), true);
	}

	#[test]
	fn bitand() {
		assert_downcast_eq!(Boolean; Boolean::qs_bitand(&true.into(), args!(true)), true);
		assert_downcast_eq!(Boolean; Boolean::qs_bitand(&true.into(), args!(false)), false);
		assert_downcast_eq!(Boolean; Boolean::qs_bitand(&false.into(), args!(true)), false);
		assert_downcast_eq!(Boolean; Boolean::qs_bitand(&false.into(), args!(false)), false);
	}

	#[test]
	#[ignore]
	fn bitand_assign() { todo!() }

	#[test]
	fn bitor() {
		assert_downcast_eq!(Boolean; Boolean::qs_bitor(&true.into(), args!(true)), true);
		assert_downcast_eq!(Boolean; Boolean::qs_bitor(&true.into(), args!(false)), true);
		assert_downcast_eq!(Boolean; Boolean::qs_bitor(&false.into(), args!(true)), true);
		assert_downcast_eq!(Boolean; Boolean::qs_bitor(&false.into(), args!(false)), false);
	}

	#[test]
	#[ignore]
	fn bitor_assign() { todo!() }

	#[test]
	fn bitxor() {
		assert_downcast_eq!(Boolean; Boolean::qs_bitxor(&true.into(), args!(true)), false);
		assert_downcast_eq!(Boolean; Boolean::qs_bitxor(&true.into(), args!(false)), true);
		assert_downcast_eq!(Boolean; Boolean::qs_bitxor(&false.into(), args!(true)), true);
		assert_downcast_eq!(Boolean; Boolean::qs_bitxor(&false.into(), args!(false)), false);
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
//! The [`Boolean`] type in Quest.

use crate::{Object, Result, Args};
use crate::types::{Number, Text, Convertible};
use std::fmt::{self, Debug, Display, Formatter};
use std::ops;

/// The Boolean type within Quest.
///
/// Internally, this is simply a newtype wrapping a `bool`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Boolean(bool);

impl Boolean {
	/// A constant representing the boolean value "false".
	pub const FALSE: Self = Self::new(false);

	/// A constant representing the boolean value "true".
	pub const TRUE: Self = Self::new(true);

	/// Simply create a new [`Boolean`].
	#[inline]
	pub const fn new(b: bool) -> Self {
		Self(b)
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
			f.debug_tuple("Boolean").field(&self.0).finish()
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
		Self::new(b)
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
	fn from(b: Boolean) -> Self {
		if b.into_inner() {
			Self::ONE
		} else {
			Self::ZERO
		}
	}
}

impl From<Boolean> for Text {
	/// Convert to a [`Text`] by mapping `true` to `"true"` and `false` to `"false"`
	#[inline]
	fn from(b: Boolean) -> Self {
		const TRUE: Text = Text::new_static("true");
		const FALSE: Text = Text::new_static("false");
		if b.into_inner() {
			TRUE
		} else {
			FALSE
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

/// Quest functions
impl Boolean {
	/// Inspects `this`.
	///
	/// # Quest Examples
	/// ```quest
	/// assert(true.$inspect() == "true");
	/// assert(false.$inspect() == "false");
	/// ```
	pub fn qs_inspect(this: &Object, args: Args) -> Result<Object> {
		Self::qs_at_text(this, args)
	}

	/// Convert `this` into a [`Number`].
	///
	/// [`true`](Boolean::TRUE) becomes [`1`](Number::ONE) and [`false`](Boolean::FALSE) becomes
	/// [`0`](Number::ZERO)
	/// # Quest Examples
	/// ```quest
	/// assert(1 + true == 2);
	/// assert(99 * false == 0);
	/// ```
	pub fn qs_at_num(this: &Object, _: Args) -> Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(Number::from(*this).into())
	}

	/// Convert `this` into a [`Text`].
	///
	/// [`true`](Boolean::TRUE) becomes `"true"` and [`false`](Boolean::FALSE) becomes `"false"`.
	pub fn qs_at_text(this: &Object, _: Args) -> Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(Text::from(*this).into())
	}

	/// Converts `this` into a [`Boolean`]
	///
	/// This simply calls [`Object::clone`](crate::Object::clone)
	pub fn qs_at_bool(this: &Object, _: Args) -> Result<Object> {
		Ok(this.clone())
	}

	/// See if a `this` is equal to the first argument.
	///
	/// Unlike most methods, the first argument is not implicitly converted to a [`Boolean`] first.
	///
	/// # Arguments
	/// 1. (required) The other object to compare against.
	pub fn qs_eql(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.downcast::<Self>();
		let this = this.try_downcast::<Self>()?;

		Ok(rhs.map(|rhs| *this == *rhs).unwrap_or(false).into())
	}

	/// Compares `this` to the first argument.
	///
	/// # Arguments
	/// 1. (required, `@bool`) The other object to compare against.
	pub fn qs_cmp(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.call_downcast::<Self>();
		let this = this.try_downcast::<Self>()?;

		Ok(rhs.map(|rhs| this.cmp(&rhs).into()).unwrap_or_default())
	}

	/// Logical NOT of `this`.
	pub fn qs_not(this: &Object, _: Args) -> Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok((!*this).into())
	}

	/// Logical AND of `this` and the first argument.
	///
	/// # Arguments
	/// 1. (required, `@bool`) The other object.
	pub fn qs_bitand(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.call_downcast::<Self>()?;
		let this = this.try_downcast::<Self>()?;

		Ok((*this & *rhs).into())
	}

	/// In-place logical AND of `this` and the first argument.
	///
	/// # Arguments
	/// 1. (required, `@bool`) The other object.
	pub fn qs_bitand_assign(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?;

		if !this.is_identical(rhs) { // `true & true = true` and `false & false = false`.
			*this.try_downcast_mut::<Self>()? &= *rhs.call_downcast::<Self>()?;
		}

		Ok(this.clone())
	}

	/// Logical OR of `this` and the first argument.
	///
	/// # Arguments
	/// 1. (required, `@bool`) The other object.
	pub fn qs_bitor(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.call_downcast::<Self>()?;
		let this = this.try_downcast::<Self>()?;

		Ok((*this | *rhs).into())
	}

	/// In-place logical OR of `this` and the first argument.
	///
	/// # Arguments
	/// 1. (required, `@bool`) The other object.
	pub fn qs_bitor_assign(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?;

		if !this.is_identical(rhs) { // `true | true = true` and `false | false = false`.
			*this.try_downcast_mut::<Self>()? |= *rhs.call_downcast::<Self>()?;
		}

		Ok(this.clone())
	}

	/// Logical XOR of `this` and the first argument.
	///
	/// # Arguments
	/// 1. (required, `@bool`) The other object.
	pub fn qs_bitxor(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.call_downcast::<Self>()?;
		let this = this.try_downcast::<Self>()?;

		Ok((*this ^ *rhs).into())
	}

	/// In-place logical XOR of this and the first argument.
	///
	/// # Arguments
	/// 1. (required, `@bool`) The other object.
	pub fn qs_bitxor_assign(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?;


		if this.is_identical(rhs) {
			*this.try_downcast_mut::<Self>()? = Self::new(false);
		} else {
			*this.try_downcast_mut::<Self>()? ^= *rhs.call_downcast::<Self>()?;
		}

		Ok(this.clone())
	}

	/// Hashes `this`.
	pub fn qs_hash(this: &Object, _: Args) -> Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(crate::utils::hash(&*this).into())
	}
}

impl Convertible for Boolean {
	const CONVERT_FUNC: &'static str = crate::literal::AT_BOOL;
}

impl_object_type!{
for Boolean {
	#[inline]
	fn new_object(self) -> Object {
		use lazy_static::lazy_static;
		use crate::types::ObjectType;

		lazy_static! {
			static ref TRUE: Object = Object::new_with_parent(Boolean::TRUE, vec![Boolean::mapping()]);
			static ref FALSE: Object = Object::new_with_parent(Boolean::FALSE,
				vec![Boolean::mapping()]);
		}

		if self.into_inner() { 
			TRUE.deep_clone()
		} else {
			FALSE.deep_clone()
		}
	}
}
[(parents super::Basic) (no_convert)]:
	"@text"   => function Self::qs_at_text,
	"inspect" => function Self::qs_inspect,
	"@num"    => function Self::qs_at_num,
	"@bool"   => function Self::qs_at_bool,
	"=="      => function Self::qs_eql,
	"!"       => function Self::qs_not,
	"&"       => function Self::qs_bitand,
	"&="      => function Self::qs_bitand_assign,
	"|"       => function Self::qs_bitor,
	"|="      => function Self::qs_bitor_assign,
	"^"       => function Self::qs_bitxor,
	"^="      => function Self::qs_bitxor_assign,
	"<=>"     => function Self::qs_cmp,
	"hash"    => function Self::qs_hash,
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn constants() {
		assert_eq!(Boolean::TRUE, Boolean::new(true));
		assert_eq!(Boolean::FALSE, Boolean::new(false));
	}

	#[test]
	fn new() {
		assert_eq!(Boolean::new(true), Boolean::TRUE);
		assert_eq!(Boolean::new(false), Boolean::FALSE);
	}

	#[test]
	fn into_inner() {
		assert_eq!(Boolean::TRUE.into_inner(), true);
		assert_eq!(Boolean::FALSE.into_inner(), false);
	}

	#[test]
	fn eql() {
		assert_eq!(Boolean::TRUE, Boolean::TRUE);
		assert_eq!(Boolean::FALSE, Boolean::FALSE);
		assert_ne!(Boolean::TRUE, Boolean::FALSE);
		assert_ne!(Boolean::FALSE, Boolean::TRUE);
	}

	#[test]
	fn display() {
		assert_eq!(format!("{}", Boolean::TRUE), "true");
		assert_eq!(format!("{}", Boolean::FALSE), "false");
	}

	#[test]
	fn into_number() {
		assert_eq!(Number::from(Boolean::TRUE), Number::ONE);
		assert_eq!(Number::from(Boolean::FALSE), Number::ZERO);
	}

	#[test]
	fn conv() {
		assert_eq!(bool::from(Boolean::from(true)), true);
		assert_eq!(bool::from(Boolean::from(false)), false);
	}

	#[test]
	fn as_ref_mut() {
		let mut b = Boolean::new(true);
		assert_eq!(b.as_ref(), &true);
		*b.as_mut() = false;
		assert_eq!(b, false);
	}

	// todo: do we test the `bitand` and friends?

	mod qs {
		use super::*;
		#[test]
		fn at_num() {
			assert_call_eq!(Boolean::qs_at_num(true) -> Number, Number::ONE);
			assert_call_eq!(Boolean::qs_at_num(false) -> Number, Number::ZERO);
			assert_call_idempotent!(Boolean::qs_at_num(true));
		}

		#[test]
		fn at_text() {
			assert_call_eq!(Boolean::qs_at_text(true) -> Text, *"true");
			assert_call_eq!(Boolean::qs_at_text(false) -> Text, *"false");
			assert_call_idempotent!(Boolean::qs_at_text(true));
		}

		#[test]
		fn inspect() {
			assert_call_eq!(Boolean::qs_inspect(true) -> Text, *"true");
			assert_call_eq!(Boolean::qs_inspect(false) -> Text, *"false");
			assert_call_idempotent!(Boolean::qs_inspect(true));
		}


		#[test]
		fn at_bool() {
			assert_call_eq!(Boolean::qs_at_bool(true) -> Boolean, true);
			assert_call_eq!(Boolean::qs_at_bool(false) -> Boolean, false);
			assert_call_non_idempotent!(Boolean::qs_at_bool(true));
		}

		#[test]
		fn eql() {
			assert_call_eq!(Boolean::qs_eql(true, true) -> Boolean, true);
			assert_call_eq!(Boolean::qs_eql(true, false) -> Boolean, false);
			assert_call_eq!(Boolean::qs_eql(false, true) -> Boolean, false);
			assert_call_eq!(Boolean::qs_eql(false, false) -> Boolean, true);

			assert_call_missing_parameter!(Boolean::qs_eql(true), 0);
			assert_call_idempotent!(Boolean::qs_eql(true, false));
		}

		#[test]
		fn not() {
			assert_call_eq!(Boolean::qs_not(true) -> Boolean, false);
			assert_call_eq!(Boolean::qs_not(false) -> Boolean, true);
			assert_call_idempotent!(Boolean::qs_not(true));
		}

		#[test]
		fn bitand() {
			assert_call_eq!(Boolean::qs_bitand(true, true) -> Boolean, true);
			assert_call_eq!(Boolean::qs_bitand(true, false) -> Boolean, false);
			assert_call_eq!(Boolean::qs_bitand(false, true) -> Boolean, false);
			assert_call_eq!(Boolean::qs_bitand(false, false) -> Boolean, false);

			assert_call_missing_parameter!(Boolean::qs_bitand(true), 0);
			assert_call_idempotent!(Boolean::qs_bitand(true, false));
		}

		#[test]
		fn bitand_assign() {
			assert_call_missing_parameter!(Boolean::qs_bitand_assign(true), 0);
			assert_call_non_idempotent!(Boolean::qs_bitand_assign(true, false) -> Boolean, false);
		}

		#[test]
		fn bitor() {
			assert_call_eq!(Boolean::qs_bitor(true, true) -> Boolean, true);
			assert_call_eq!(Boolean::qs_bitor(true, false) -> Boolean, true);
			assert_call_eq!(Boolean::qs_bitor(false, true) -> Boolean, true);
			assert_call_eq!(Boolean::qs_bitor(false, false) -> Boolean, false);

			assert_call_missing_parameter!(Boolean::qs_bitor(true), 0);
			assert_call_idempotent!(Boolean::qs_bitor(true, false));
		}

		#[test]
		fn bitor_assign() {
			assert_call_missing_parameter!(Boolean::qs_bitor_assign(true), 0);
			assert_call_non_idempotent!(Boolean::qs_bitor_assign(false, true) -> Boolean, true);
		}

		#[test]
		fn bitxor() {
			assert_call_eq!(Boolean::qs_bitxor(true, true) -> Boolean, false);
			assert_call_eq!(Boolean::qs_bitxor(true, false) -> Boolean, true);
			assert_call_eq!(Boolean::qs_bitxor(false, true) -> Boolean, true);
			assert_call_eq!(Boolean::qs_bitxor(false, false) -> Boolean, false);

			assert_call_missing_parameter!(Boolean::qs_bitxor(true), 0);
			assert_call_idempotent!(Boolean::qs_bitxor(true, false));
		}

		#[test]
		fn bitxor_assign() {
			assert_call_missing_parameter!(Boolean::qs_bitxor_assign(true), 0);
			assert_call_non_idempotent!(Boolean::qs_bitxor_assign(false, true) -> Boolean, true);
		}

		#[test]
		fn cmp() {
			let gt = Number::ONE;
			let lt = -Number::ONE;
			let eq = Number::ZERO;

			assert_call_eq!(Boolean::qs_cmp(true, false) -> Number, gt);
			assert_call_eq!(Boolean::qs_cmp(true, true) -> Number, eq);
			assert_call_eq!(Boolean::qs_cmp(false, true) -> Number, lt);
			assert_call_eq!(Boolean::qs_cmp(false, false) -> Number, eq);

			// make sure reflexive comparisons work
			let t = Object::from(true);
			let f = Object::from(false);
			assert_call_eq!(Boolean::qs_cmp(t.clone(), t) -> Number, eq);
			assert_call_eq!(Boolean::qs_cmp(f.clone(), f) -> Number, eq);

			// ensure that Null is returned for types that don't implement `@bool`
			#[derive(Debug, Clone)]
			struct Dummy;
			impl_object_type! { for Dummy [(parents crate::types::Pristine)]: }

			assert!(!Object::from(Dummy).has_attr_lit(crate::literal::AT_BOOL).unwrap());
			assert_call_eq!(Boolean::qs_cmp(true, Dummy) -> Null, Null);

			assert_call_missing_parameter!(Boolean::qs_cmp(true), 0);
			assert_call_idempotent!(Boolean::qs_cmp(false, true));
		}

		#[test]
		fn hash() {
			assert_eq!(
				call_unwrap!(Boolean::qs_hash(true) -> Number; |n| *n),
				call_unwrap!(Boolean::qs_hash(true) -> Number; |n| *n)
			);

			assert_eq!(
				call_unwrap!(Boolean::qs_hash(false) -> Number; |n| *n),
				call_unwrap!(Boolean::qs_hash(false) -> Number; |n| *n)
			);
		}
	}
}

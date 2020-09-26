//! The [`Boolean`] type in Quest.
use crate::{Object, Args};
use crate::types::{Number, Text, Convertible};
use std::fmt::{self, Debug, Display, Formatter};
use std::ops;

/// The Boolean type within Quest.
///
/// This type can only be [`true`](Self::TRUE) or [`false`](Self::FALSE).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Boolean(bool);

/// An error returned when parsing a bool using from_str fails.
pub type ParseBoolError = std::str::ParseBoolError;

impl Boolean {
	/// A constant representing the boolean value "true".
	///
	/// # Examples
	/// ```rust
	/// use quest_core::types::Boolean;
	///
	/// assert_eq!(Boolean::TRUE, true);
	/// ```
	pub const TRUE: Self = Self::new(true);

	/// A constant representing the boolean value "false".
	///
	/// # Examples
	/// ```rust
	/// use quest_core::types::Boolean;
	///
	/// assert_eq!(Boolean::FALSE, false);
	/// ```
	pub const FALSE: Self = Self::new(false);

	/// Simply create a new [`Boolean`].
	///
	/// # Examples
	/// ```rust
	/// use quest_core::types::Boolean;
	///
	/// assert_eq!(Boolean::new(true), true);
	/// ```
	#[inline]
	pub const fn new(b: bool) -> Self {
		Self(b)
	}

	/// Unwraps the value.
	///
	/// # Examples
	/// ```rust
	/// use quest_core::types::Boolean;
	///
	/// assert_eq!(Boolean::TRUE.into_inner(), true);
	/// ```
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

impl PartialOrd<bool> for Boolean {
	fn partial_cmp(&self, rhs: &bool) -> Option<std::cmp::Ordering> {
		self.0.partial_cmp(rhs)
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

impl std::str::FromStr for Boolean {
	type Err = ParseBoolError;

	/// Tries to convert `inp` to a [`Boolean`].
	///
	/// `"true"` will become [`Boolean::TRUE`] and `"false"` will become [`Boolean::FALSE`]. Any other value will yield a
	/// [`ParseBoolError`].
	fn from_str(inp: &str) -> Result<Self, Self::Err> {
		inp.parse().map(Self::new)
	}
}

impl std::convert::TryFrom<&str> for Boolean {
	type Error = ParseBoolError;

	#[inline]
	fn try_from(val: &str) -> Result<Self, Self::Error> {
		val.parse()
	}
}

impl From<bool> for Object {
	/// Converts this into a [`Boolean`] and then into an [`Object`].
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

impl std::borrow::Borrow<bool> for Boolean {
	fn borrow(&self) -> &bool {
		self.as_ref()
	}
}

impl AsMut<bool> for Boolean {
	#[inline]
	fn as_mut(&mut self) -> &mut bool {
		&mut self.0
	}
}

impl From<Boolean> for Number {
	/// Convert to a [`Number`] by mapping `true` to [`Number::ONE`] and `false` to [`Number::ZERO`].
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
	fn from(b: Boolean) -> Self {
		if b.into_inner() {
			Self::const_new("true")
		} else {
			Self::const_new("false")
		}
	}
}

impl ops::BitAnd for Boolean {
	type Output = Self;

	#[inline]
	fn bitand(self, rhs: Self) -> Self {
		Self::new(self.0 & rhs.0)
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
		Self::new(self.0 | rhs.0)
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
		Self::new(self.0 ^ rhs.0)
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
		Self::new(!self.0)
	}
}

/// Quest functions
impl Boolean {
	/// Inspects `this`.
	///
	/// # Arguments
	/// None.
	///
	/// # Returns
	/// A [`Text`] object containing either `true` or `false`.
	///
	/// # Errors
	/// If `this` isn't a [`Boolean`], a [`TypeError::WrongType`](crate::error::TypeError::WrongType) is returned.
	///
	/// # Rust Examples
	/// ```rust
	/// use quest_core::{Object, Args};
	/// use quest_core::types::{Boolean, Text};
	///
	/// assert_eq!(
	/// 	*Boolean::qs_inspect(&true.into(), Args::default()).unwrap()
	/// 		.downcast::<Text>().unwrap(),
	/// 	Text::new("true")
	/// );
	/// ```
	///
	/// # Quest Examples
	/// ```quest
	/// assert(true.$inspect() == "true");
	/// assert(false.$inspect() == "false");
	/// ```
	///
	/// # See Also
	/// - [`Boolean::qs_at_text`] -- Identical to this function.
	pub fn qs_inspect(this: &Object, args: Args) -> crate::Result<Object> {
		Self::qs_at_text(this, args)
	}

	/// Convert `this` into a [`Number`].
	///
	/// # Arguments
	/// None.
	///
	/// # Returns
	/// A [`Number`] object containing either [`1`](Number::ONE) (for  [`true`](Self::TRUE)) or [`0`](Number::ZERO) (
	/// for [`false`](Self::FALSE)).
	///
	/// # Errors
	/// If `this` isn't a [`Boolean`], a [`TypeError::WrongType`](crate::error::TypeError::WrongType) is returned.
	///
	/// # Rust Examples
	/// ```rust
	/// use quest_core::{Object, Args};
	/// use quest_core::types::{Boolean, Number};
	///
	/// assert_eq!(
	/// 	*Boolean::qs_at_num(&true.into(), Args::default()).unwrap()
	/// 		.downcast::<Number>().unwrap(),
	/// 	Number::ONE
	/// );
	/// ```
	///
	/// # Quest Examples
	/// ```quest
	/// assert(1 + true == 2);
	/// assert(99 * false == 0);
	/// ```
	pub fn qs_at_num(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(Number::from(*this).into())
	}

	/// Convert `this` into a [`Text`].
	///
	/// # Arguments
	/// None.
	///
	/// # Returns
	/// [`true`](Self::TRUE) becomes `"true"` and [`false`](Self::FALSE) becomes `"false"`.
	///
	/// # Errors
	/// If `this` isn't a [`Boolean`], a [`TypeError::WrongType`](crate::error::TypeError::WrongType) is returned.
	///
	/// # Rust Examples
	/// ```rust
	/// use quest_core::{Object, Args};
	/// use quest_core::types::{Boolean, Text};
	///
	/// assert_eq!(
	/// 	*Boolean::qs_at_text(&true.into(), Args::default()).unwrap()
	/// 		.downcast::<Text>().unwrap(),
	/// 	Text::new("true")
	/// );
	/// ```
	///
	/// # Quest Examples
	/// ```quest
	/// assert("yes: " + true == "yes: true");
	/// assert("no: " + false == "no: false");
	/// ```
	pub fn qs_at_text(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(Text::from(*this).into())
	}

	/// Converts `this` into a [`Boolean`].
	///
	/// This simply calls [`Object::clone`](crate::Object::clone).
	/// 
	/// # Arguments
	/// None.
	///
	/// # Returns
	/// `this`.
	///
	/// # Errors
	/// None.
	/// 
	/// # Rust Examples
	/// ```rust
	/// use quest_core::{Object, Args};
	/// use quest_core::types::Boolean;
	///
	/// let obj = Object::from(true);
	/// let dup = Boolean::qs_at_bool(&obj, Args::default()).unwrap();
	///
	/// assert!(obj.is_identical(&dup));
	/// ```
	///
	/// # Quest Examples
	/// ```quest
	/// $obj = true;
	/// $dup = obj.$dup();
	/// assert(obj == dup);
	/// assert(obj.$__id__ != dup.$__id__);
	/// ```
	pub fn qs_at_bool(this: &Object, _: Args) -> crate::Result<Object> {
		Ok(this.clone())
	}

	/// See if a `this` is equal to the first argument.
	///
	/// Unlike most methods, the first argument is not implicitly converted to a [`Boolean`] first. This will return
	/// `true` if 
	///
	/// # Arguments
	/// 1. (required) The other object to compare against.
	///
	/// # 
	pub fn qs_eql(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?.downcast::<Self>();
		let this = this.try_downcast::<Self>()?;

		Ok(rhs.map_or(false, |rhs| *this == *rhs).into())
	}

	/// Compares `this` to the first argument.
	///
	/// # Arguments
	/// 1. (required, `@bool`) The other object to compare against.
	pub fn qs_cmp(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?.call_downcast::<Self>();
		let this = this.try_downcast::<Self>()?;

		// Ok(rhs.map(|rhs| this.cmp(&rhs).into()).unwrap_or_default())
		Ok(rhs.ok().map_or_else(Default::default, |rhs| this.cmp(&rhs).into()))
	}

	/// Logical NOT of `this`.
	pub fn qs_not(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok((!*this).into())
	}

	/// Logical AND of `this` and the first argument.
	///
	/// # Arguments
	/// 1. (required, `@bool`) The other object.
	pub fn qs_bitand(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?.call_downcast::<Self>()?;
		let this = this.try_downcast::<Self>()?;

		Ok((*this & *rhs).into())
	}

	/// In-place logical AND of `this` and the first argument.
	///
	/// # Arguments
	/// 1. (required, `@bool`) The other object.
	pub fn qs_bitand_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?;

		if !this.is_identical(rhs) { // `true & true = true` and `false & false = false`.
			*this.try_downcast_mut::<Self>()? &= *rhs.call_downcast::<Self>()?;
		}

		Ok(this.clone())
	}

	/// Logical OR of `this` and the first argument.
	///
	/// # Arguments
	/// 1. (required, `@bool`) The other object.
	pub fn qs_bitor(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?.call_downcast::<Self>()?;
		let this = this.try_downcast::<Self>()?;

		Ok((*this | *rhs).into())
	}

	/// In-place logical OR of `this` and the first argument.
	///
	/// # Arguments
	/// 1. (required, `@bool`) The other object.
	pub fn qs_bitor_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?;

		if !this.is_identical(rhs) { // `true | true = true` and `false | false = false`.
			*this.try_downcast_mut::<Self>()? |= *rhs.call_downcast::<Self>()?;
		}

		Ok(this.clone())
	}

	/// Logical XOR of `this` and the first argument.
	///
	/// # Arguments
	/// 1. (required, `@bool`) The other object.
	pub fn qs_bitxor(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?.call_downcast::<Self>()?;
		let this = this.try_downcast::<Self>()?;

		Ok((*this ^ *rhs).into())
	}

	/// In-place logical XOR of this and the first argument.
	///
	/// # Arguments
	/// 1. (required, `@bool`) The other object.
	pub fn qs_bitxor_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?;

		if this.is_identical(rhs) {
			*this.try_downcast_mut::<Self>()? = Self::new(false);
		} else {
			*this.try_downcast_mut::<Self>()? ^= *rhs.call_downcast::<Self>()?;
		}

		Ok(this.clone())
	}

	/// Hashes `this`.
	pub fn qs_hash(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(crate::utils::hash(&*this).into())
	}
}

impl Convertible for Boolean {
	const CONVERT_FUNC: crate::Literal = crate::Literal::AT_BOOL;
}

impl_object_type!{

for Boolean {
	#[inline]
	fn new_object(self) -> Object {
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

			assert!(!Object::from(Dummy).has_attr_lit(&crate::Literal::AT_BOOL).unwrap());
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

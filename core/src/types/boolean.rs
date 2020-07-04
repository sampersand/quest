use crate::{Object, Result, Args, attrs};
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

impl attrs::Inspect for Boolean {
	/// Inspects the [`Boolean`].
	///
	/// This is identical to [`<Boolean as AtText>::call`]
	/// # Quest Examples
	/// ```quest
	/// assert(true.$inspect() == "true");
	/// assert(false.$inspect() == "false");
	/// ```
	#[inline]
	fn qs_inspect(this: &Object, args: Args) -> Result<Object> {
		<Self as attrs::AtText>::qs_at_text(this, args)
	}
}

impl attrs::AtNumber for Boolean {
	/// Convert this into a [`Number`].
	///
	/// [`true`](Boolean::TRUE) becomes [`1`](Number::ONE) and [`false`](Boolean::FALSE) becomes
	/// [`0`](Number::ZERO)
	/// # Quest Examples
	/// ```quest
	/// assert(1 + true == 2);
	/// assert(99 * false == 0);
	/// ```
	#[inline]
	fn qs_at_num(this: &Object, _: Args) -> Result<Object> {
		this.try_downcast_map(|this: &Self| Number::from(*this).into())
	}
}

impl attrs::AtText for Boolean {
	/// Convert this into a [`Text`].
	///
	/// [`true`](Boolean::TRUE) becomes `"true"` and [`false`](Boolean::FALSE) becomes `"false"`.
	#[inline]
	fn qs_at_text(this: &Object, _: Args) -> Result<Object> {
		this.try_downcast_map(|this: &Self| Text::from(*this).into())
	}
}

impl attrs::AtBoolean for Boolean {
	/// Converts this into a [`Boolean`]
	///
	/// This simply calls [`Object::deep_clone`](crate::Object::deep_clone)
	#[inline]
	fn qs_at_bool(this: &Object, _: Args) -> Result<Object> {
		Ok(this.deep_clone())
	}
}

impl attrs::Eql for Boolean {
	/// See if a this is equal to the first argument.
	///
	/// Unlike most methods, the first argument is not implicitly converted to a  [`Boolean`] first.
	///
	/// # Arguments
	/// 1. (required) The value to compare against
	#[inline]
	fn qs_eql(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?;

		this.try_downcast_map(|lhs: &Self| {
			rhs.downcast_and_then(|rhs: &Self| lhs == rhs).unwrap_or(false).into()
		})
	}
}

impl attrs::Cmp for Boolean {
	/// Compares this to the first argument.
	///
	/// # Arguments
	/// 1. (required, `@bool`) The value to compare against
	#[inline]
	fn qs_cmp(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?;

		this.try_downcast_map(|lhs: &Self| {
			rhs.call_downcast_map(|rhs: &Self| lhs.cmp(rhs).into())
				.unwrap_or_default()
		})
	}
}

impl attrs::Not for Boolean {
	/// Logical NOT of this.
	#[inline]
	fn qs_not(this: &Object, _: Args) -> Result<Object> {
		this.try_downcast_map(|this: &Self| (!*this).into())
	}
}

impl attrs::BitAnd for Boolean {
	/// Logical AND of this and the first argument.
	///
	/// # Arguments
	/// 1. (required, `@bool`) The right-hand side
	#[inline]
	fn qs_bitand(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_map(|this: &Self| (*this & rhs).into())
	}
}

impl attrs::BitAndAssign for Boolean {
	/// In-place logical AND of this and the first argument.
	///
	/// # Arguments
	/// 1. (required, `@bool`) The right-hand side
	#[inline]
	fn qs_bitand_assign(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_mut_map(|this: &mut Self| *this &= rhs)
			.map(|_| this.clone())
	}
}

impl attrs::BitOr for Boolean {
	/// Logical OR of this and the first argument.
	///
	/// # Arguments
	/// 1. (required, `@bool`) The right-hand side
	#[inline]
	fn qs_bitor(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_map(|this: &Self| (*this | rhs).into())
	}
}

impl attrs::BitOrAssign for Boolean {
	/// In-place logical OR of this and the first argument.
	///
	/// # Arguments
	/// 1. (required, `@bool`) The right-hand side
	#[inline]
	fn qs_bitor_assign(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_mut_map(|this: &mut Self| *this |= rhs)
			.map(|_| this.clone())
	}
}

impl attrs::BitXor for Boolean {
	/// Logical XOR of this and the first argument.
	///
	/// # Arguments
	/// 1. (required, `@bool`) The right-hand side
	#[inline]
	fn qs_bitxor(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_map(|this: &Self| (*this ^ rhs).into())
	}
}

impl attrs::BitXorAssign for Boolean {
	/// In-place logical XOR of this and the first argument.
	///
	/// # Arguments
	/// 1. (required, `@bool`) The right-hand side
	#[inline]
	fn qs_bitxor_assign(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_mut_map(|this: &mut Self| *this ^= rhs)
			.map(|_| this.clone())
	}
}

impl attrs::Hash for Boolean {
	#[inline]
	fn qs_hash(this: &Object, _: Args) -> Result<Object> {
		this.try_downcast_map(|this: &Self| crate::utils::hash(this).into())
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
	"@text"   => function <Boolean as attrs::AtText>::qs_at_text,
	"inspect" => function <Boolean as attrs::Inspect>::qs_inspect,
	"@num"    => function <Boolean as attrs::AtNumber>::qs_at_num,
	"@bool"   => function <Boolean as attrs::AtBoolean>::qs_at_bool,
	"=="      => function <Boolean as attrs::Eql>::qs_eql,
	"!"       => function <Boolean as attrs::Not>::qs_not,
	"&"       => function <Boolean as attrs::BitAnd>::qs_bitand,
	"&="      => function <Boolean as attrs::BitAndAssign>::qs_bitand_assign,
	"|"       => function <Boolean as attrs::BitOr>::qs_bitor,
	"|="      => function <Boolean as attrs::BitOrAssign>::qs_bitor_assign,
	"^"       => function <Boolean as attrs::BitXor>::qs_bitxor,
	"^="      => function <Boolean as attrs::BitXorAssign>::qs_bitxor_assign,
	"<=>"     => function <Boolean as attrs::Cmp>::qs_cmp,
	"hash"    => function <Boolean as attrs::Hash>::qs_hash,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::attrs::*;


	#[test]
	fn at_num() {
		<Boolean as crate::types::ObjectType>::_wait_for_setup_to_finish();
		assert_downcast_eq!(Number; Boolean::qs_at_num(&true.into(), args!()).unwrap(), Number::ONE);
		assert_downcast_eq!(Number; Boolean::qs_at_num(&false.into(), args!()).unwrap(), Number::ZERO);

		assert_downcast_eq!(Number; Boolean::qs_at_num(&true.into(), args!(false)).unwrap(), Number::ONE);
	}

	#[test]
	fn at_text() {
		<Boolean as crate::types::ObjectType>::_wait_for_setup_to_finish();
		assert_downcast_eq!(Text; Boolean::qs_at_text(&true.into(), args!()).unwrap(), Text::new_static("true"));
		assert_downcast_eq!(Text; Boolean::qs_at_text(&false.into(), args!()).unwrap(), Text::new_static("false"));

		assert_downcast_eq!(Text; Boolean::qs_at_text(&true.into(), args!(false)).unwrap(), Text::new_static("true"));
	}


	#[test]
	fn at_bool() {
		<Boolean as crate::types::ObjectType>::_wait_for_setup_to_finish();

		{
			let ref orig = Object::from(true);
			let ref dup = Boolean::qs_at_bool(orig, args!()).unwrap();
			assert!(!orig.is_identical(dup));
			assert_downcast_both_eq!(Boolean; orig, dup);
		}

		{
			let ref orig = Object::from(false);
			let ref dup = Boolean::qs_at_bool(orig, args!()).unwrap();
			assert!(!orig.is_identical(dup));
			assert_downcast_both_eq!(Boolean; orig, dup);
		}

		{
			let ref orig = Object::from(false);
			let ref dup = Boolean::qs_at_bool(orig, args!(true)).unwrap();
			assert!(!orig.is_identical(dup));
			assert_downcast_both_eq!(Boolean; orig, dup);
		}
	}

	#[test]
	fn eql() {
		<Boolean as crate::types::ObjectType>::_wait_for_setup_to_finish();

		assert_downcast_eq!(Boolean; Boolean::qs_eql(&true.into(), args!(true)).unwrap(), true);
		assert_downcast_eq!(Boolean; Boolean::qs_eql(&true.into(), args!(false)).unwrap(), false);
		assert_downcast_eq!(Boolean; Boolean::qs_eql(&false.into(), args!(true)).unwrap(), false);
		assert_downcast_eq!(Boolean; Boolean::qs_eql(&false.into(), args!(false)).unwrap(), true);

		assert_missing_parameter!(Boolean::qs_eql(&true.into(), args!()), 0);
		assert_missing_parameter!(Boolean::qs_eql(&false.into(), args!()), 0);
		assert_downcast_eq!(Boolean; Boolean::qs_eql(&false.into(), args!(false, true)).unwrap(), true);
	}

	#[test]
	fn not() {
		<Boolean as crate::types::ObjectType>::_wait_for_setup_to_finish();
		assert_downcast_eq!(Boolean; Boolean::qs_not(&true.into(), args!()).unwrap(), false);
		assert_downcast_eq!(Boolean; Boolean::qs_not(&false.into(), args!()).unwrap(), true);

		assert_downcast_eq!(Boolean; Boolean::qs_not(&true.into(), args!(true)).unwrap(), false);

	}

	#[test]
	fn bitand() {
		<Boolean as crate::types::ObjectType>::_wait_for_setup_to_finish();

		assert_downcast_eq!(Boolean; Boolean::qs_bitand(&true.into(), args!(true)).unwrap(), true);
		assert_downcast_eq!(Boolean; Boolean::qs_bitand(&true.into(), args!(false)).unwrap(), false);
		assert_downcast_eq!(Boolean; Boolean::qs_bitand(&false.into(), args!(true)).unwrap(), false);
		assert_downcast_eq!(Boolean; Boolean::qs_bitand(&false.into(), args!(false)).unwrap(), false);

		assert_missing_parameter!(Boolean::qs_bitand(&true.into(), args!()), 0);
		assert_missing_parameter!(Boolean::qs_bitand(&false.into(), args!()), 0);
		assert_downcast_eq!(Boolean; Boolean::qs_bitand(&true.into(), args!(true, false)).unwrap(), true);
	}

	#[test]
	fn bitand_assign() {
		<Boolean as crate::types::ObjectType>::_wait_for_setup_to_finish();

		{
			let ref orig = Object::from(true);

			assert!(orig.is_identical(&Boolean::qs_bitand_assign(orig, args!(orig.clone())).unwrap()));
			assert_downcast_eq!(Boolean; orig, true);

			assert!(orig.is_identical(&Boolean::qs_bitand_assign(orig, args!(true)).unwrap()));
			assert_downcast_eq!(Boolean; orig, true);

			assert!(orig.is_identical(&Boolean::qs_bitand_assign(orig, args!(true, false)).unwrap()));
			assert_downcast_eq!(Boolean; orig, true);

			assert!(orig.is_identical(&Boolean::qs_bitand_assign(orig, args!(false)).unwrap()));
			assert_downcast_eq!(Boolean; orig, false);

			assert_missing_parameter!(Boolean::qs_bitand(orig, args!()), 0);
		}

		{
			let ref orig = Object::from(false);

			assert!(orig.is_identical(&Boolean::qs_bitand_assign(orig, args!(orig.clone())).unwrap()));
			assert_downcast_eq!(Boolean; orig, false);

			assert!(orig.is_identical(&Boolean::qs_bitand_assign(orig, args!(true)).unwrap()));
			assert_downcast_eq!(Boolean; orig, false);

			assert!(orig.is_identical(&Boolean::qs_bitand_assign(orig, args!(false)).unwrap()));
			assert_downcast_eq!(Boolean; orig, false);

			assert_missing_parameter!(Boolean::qs_bitand(orig, args!()), 0);
		}

	}

	#[test]
	fn bitor() {
		<Boolean as crate::types::ObjectType>::_wait_for_setup_to_finish();
		assert_downcast_eq!(Boolean; Boolean::qs_bitor(&true.into(), args!(true)).unwrap(), true);
		assert_downcast_eq!(Boolean; Boolean::qs_bitor(&true.into(), args!(false)).unwrap(), true);
		assert_downcast_eq!(Boolean; Boolean::qs_bitor(&false.into(), args!(true)).unwrap(), true);
		assert_downcast_eq!(Boolean; Boolean::qs_bitor(&false.into(), args!(false)).unwrap(), false);

		assert_missing_parameter!(Boolean::qs_bitor(&true.into(), args!()), 0);
		assert_missing_parameter!(Boolean::qs_bitor(&false.into(), args!()), 0);
		assert_downcast_eq!(Boolean; Boolean::qs_bitor(&false.into(), args!(false, true)).unwrap(), false);
	}

	#[test]
	fn bitor_assign() {
		<Boolean as crate::types::ObjectType>::_wait_for_setup_to_finish();

		{
			let ref orig = Object::from(false);

			assert!(orig.is_identical(&Boolean::qs_bitor_assign(orig, args!(orig.clone())).unwrap()));
			assert_downcast_eq!(Boolean; orig, false);

			assert!(orig.is_identical(&Boolean::qs_bitor_assign(orig, args!(false)).unwrap()));
			assert_downcast_eq!(Boolean; orig, false);

			assert!(orig.is_identical(&Boolean::qs_bitor_assign(orig, args!(false, true)).unwrap()));
			assert_downcast_eq!(Boolean; orig, false);

			assert!(orig.is_identical(&Boolean::qs_bitor_assign(orig, args!(true)).unwrap()));
			assert_downcast_eq!(Boolean; orig, true);

			assert_missing_parameter!(Boolean::qs_bitand(orig, args!()), 0);
		}

		{
			let ref orig = Object::from(true);

			assert!(orig.is_identical(&Boolean::qs_bitor_assign(orig, args!(orig.clone())).unwrap()));
			assert_downcast_eq!(Boolean; orig, true);

			assert!(orig.is_identical(&Boolean::qs_bitor_assign(orig, args!(true)).unwrap()));
			assert_downcast_eq!(Boolean; orig, true);

			assert!(orig.is_identical(&Boolean::qs_bitor_assign(orig, args!(false)).unwrap()));
			assert_downcast_eq!(Boolean; orig, true);

			assert_missing_parameter!(Boolean::qs_bitand(orig, args!()), 0);
		}
	}


	#[test]
	fn bitxor() {
		<Boolean as crate::types::ObjectType>::_wait_for_setup_to_finish();
		assert_downcast_eq!(Boolean; Boolean::qs_bitxor(&true.into(), args!(true)).unwrap(), false);
		assert_downcast_eq!(Boolean; Boolean::qs_bitxor(&true.into(), args!(false)).unwrap(), true);
		assert_downcast_eq!(Boolean; Boolean::qs_bitxor(&false.into(), args!(true)).unwrap(), true);
		assert_downcast_eq!(Boolean; Boolean::qs_bitxor(&false.into(), args!(false)).unwrap(), false);

		assert_missing_parameter!(Boolean::qs_bitxor(&true.into(), args!()), 0);
		assert_missing_parameter!(Boolean::qs_bitxor(&false.into(), args!()), 0);
		assert_downcast_eq!(Boolean; Boolean::qs_bitxor(&false.into(), args!(false, true)).unwrap(), false);
	}

	#[test]
	fn bitxor_assign() {
		<Boolean as crate::types::ObjectType>::_wait_for_setup_to_finish();

		{
			let ref orig = Object::from(false);

			assert!(orig.is_identical(&Boolean::qs_bitxor_assign(orig, args!(orig.clone())).unwrap()));
			assert_downcast_eq!(Boolean; orig, false);

			assert!(orig.is_identical(&Boolean::qs_bitxor_assign(orig, args!(false)).unwrap()));
			assert_downcast_eq!(Boolean; orig, false);

			assert!(orig.is_identical(&Boolean::qs_bitxor_assign(orig, args!(false, true)).unwrap()));
			assert_downcast_eq!(Boolean; orig, false);

			assert!(orig.is_identical(&Boolean::qs_bitxor_assign(orig, args!(true)).unwrap()));
			assert_downcast_eq!(Boolean; orig, true);

			assert_missing_parameter!(Boolean::qs_bitand(orig, args!()), 0);
		}

		{
			let ref orig = Object::from(true);

			assert!(orig.is_identical(&Boolean::qs_bitxor_assign(orig, args!(false)).unwrap()));
			assert_downcast_eq!(Boolean; orig, true);

			assert!(orig.is_identical(&Boolean::qs_bitxor_assign(orig, args!(false, true)).unwrap()));
			assert_downcast_eq!(Boolean; orig, true);

			assert!(orig.is_identical(&Boolean::qs_bitxor_assign(orig, args!(orig.clone())).unwrap()));
			assert_downcast_eq!(Boolean; orig, false);
			// orig is now false

			assert!(orig.is_identical(&Boolean::qs_bitxor_assign(orig, args!(true)).unwrap()));
			assert_downcast_eq!(Boolean; orig, true);
			// orig is now true

			assert!(orig.is_identical(&Boolean::qs_bitxor_assign(orig, args!(true)).unwrap()));
			assert_downcast_eq!(Boolean; orig, false);

			assert_missing_parameter!(Boolean::qs_bitand(orig, args!()), 0);
		}
	}

	#[test]
	fn cmp() {
		<Boolean as crate::types::ObjectType>::_wait_for_setup_to_finish();

		let gt = Number::ONE;
		let lt = -Number::ONE;
		let eq = Number::ZERO;

		assert_downcast_eq!(Number; Boolean::qs_cmp(&true.into(), args!(false)).unwrap(), gt);
		assert_downcast_eq!(Number; Boolean::qs_cmp(&true.into(), args!(true)).unwrap(), eq);
		assert_downcast_eq!(Number; Boolean::qs_cmp(&false.into(), args!(true)).unwrap(), lt);
		assert_downcast_eq!(Number; Boolean::qs_cmp(&false.into(), args!(false)).unwrap(), eq);

		// make sure reflexive comparisons work
		let t = Object::from(true);
		assert_downcast_eq!(Number; Boolean::qs_cmp(&t, args!(t.clone())).unwrap(), eq);

		let f = Object::from(false);
		assert_downcast_eq!(Number; Boolean::qs_cmp(&f, args!(f.clone())).unwrap(), eq);

		// ensure that Null is returned for types that don't implement `@bool`
		#[derive(Debug, Clone)]
		struct Dummy;
		impl_object_type! { for Dummy [(parents crate::types::Pristine)]: }

		assert!(!Object::from(Dummy).has_attr_lit(crate::literals::AT_BOOL).unwrap());
		assert!(Boolean::qs_cmp(&true.into(), args!(Dummy)).unwrap().is_a::<crate::types::Null>());

		// make sure it responds correctly to too few parameters
		assert_missing_parameter!(Boolean::qs_cmp(&true.into(), args!()), 0);

		// make sure it responds correctly to too many parameters
		assert_downcast_eq!(Number; Boolean::qs_cmp(&false.into(), args!(false, true)).unwrap(), eq);
	}

	#[test]
	fn hash() {
		<Boolean as crate::types::ObjectType>::_wait_for_setup_to_finish();

		assert_downcast_both_eq!(Number;
			Boolean::qs_hash(&true.into(), args!()).unwrap(),
			Boolean::qs_hash(&true.into(), args!()).unwrap());

		assert_downcast_both_eq!(Number;
			Boolean::qs_hash(&false.into(), args!()).unwrap(),
			Boolean::qs_hash(&false.into(), args!()).unwrap());

		assert_downcast_both_ne!(Number;
			Boolean::qs_hash(&true.into(), args!()).unwrap(),
			Boolean::qs_hash(&false.into(), args!()).unwrap());


		// make sure it responds correctly to too many parameters
		assert_downcast_both_eq!(Number;
			Boolean::qs_hash(&true.into(), args!(false)).unwrap(),
			Boolean::qs_hash(&true.into(), args!()).unwrap());
	}
}











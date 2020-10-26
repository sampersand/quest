//! The [`Number`] type in Quest.

use std::convert::TryFrom;
use std::fmt::{self, Debug, Display, Formatter};
use std::cmp::Ordering;
use std::ops;
use crate::{Object, Args};
use crate::types::{Text, Boolean, Convertible};
use std::hash::{Hash, Hasher};
use crate::error::{TypeError, ValueError};
use tracing::instrument;

/// The type used by [`Number`] to keep track of integers.
pub type IntegerType = i64;

/// The type used by [`Number`] to keep track of floats.
pub type FloatType = f64;

/// The Number type in Quest.
///
/// There's only one struct because there's no distinction between integers and floats within Quest.
/// Because of this, most functions that require integers will [truncate](#floor) floating point
/// numbers, but the bitwise operations will raise [`NotAnInteger`] if performed with non-integers.
#[derive(Clone, Copy)]
pub struct Number(Inner);

// note: to ensure consistancy, there won't ever be a `Float` that has an integer within it;
// all integer `FloatType`s (eg `2.0`) are converted to `IntegerType` first.
#[derive(Clone, Copy)]
enum Inner {
	Integer(IntegerType),
	Float(FloatType),
	// (In the future, I intend to add in a "large number" variant)
	// should we add a "not a number" variant here?
}

impl Eq for Number {}

impl PartialEq for Number {
	fn eq(&self, rhs: &Self) -> bool {
		match (self.0, rhs.0) {
			(Inner::Integer(l), Inner::Integer(r)) => l == r,
			(Inner::Float(l), Inner::Float(r)) => l == r,
			_ => false
		}
	}
}

impl Hash for Number {
	fn hash<H: Hasher>(&self, h: &mut H) {
		match self.0 {
			Inner::Integer(i) => { 0i8.hash(h); i.hash(h) },
			Inner::Float(f) => { 1i8.hash(h); f.to_bits().hash(h) }
		}
	}
}

impl Default for Number {
	/// The default number is [zero](Number::ZERO).
	#[inline]
	fn default() -> Self {
		Self::ZERO
	}
}

impl Debug for Number {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			match self.0 {
				Inner::Integer(n) => f.debug_tuple("Number").field(&n).finish(),
				Inner::Float(n) => f.debug_tuple("Number").field(&n).finish(),
			}
		} else {
			Display::fmt(self, f)
		}
	}
}

impl Display for Number {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self.0 {
			Inner::Integer(n) => Display::fmt(&n, f),
			Inner::Float(n) => Display::fmt(&n, f),
		}
	}
}

impl Number {
	/// The number zero.
	pub const ZERO: Self = Self(Inner::Integer(0 as IntegerType));

	/// The number one.
	pub const ONE: Self = Self(Inner::Integer(1 as IntegerType));

	/// The mathematical constant π.
	pub const PI: Self = Self(Inner::Float(std::f64::consts::PI));

	/// The mathematical constant e.
	pub const E: Self = Self(Inner::Float(std::f64::consts::E));

	/// The concept of "not a number".
	pub const NAN: Self = Self(Inner::Float(FloatType::NAN));

	/// Infinity!
	pub const INF: Self = Self(Inner::Float(FloatType::INFINITY));

	/// Create a new number.
	#[inline]
	pub fn new<N: Into<Self>>(num: N) -> Self {
		num.into()
	}

	/// Rounds `self` to the next highest integer (nothing's done if `self` is an integer).
	pub fn ceil(&self) -> Self {
		match self.0 {
			Inner::Integer(..) => *self,
			Inner::Float(f) => f.ceil().into()
		}
	}

	/// Rounds `self` to the next lowest integer (nothing's done if `self` is an integer).
	pub fn floor(&self) -> Self {
		match self.0 {
			Inner::Integer(..) => *self,
			Inner::Float(f) => f.floor().into()
		}
	}

	pub fn truncate(&self) -> IntegerType {
		TryFrom::try_from(self.round()).unwrap()
	}

	/// Rounds `self` to the nearest integer (nothing's done if `self` is an integer).
	pub fn round(&self) -> Self {
		match self.0 {
			Inner::Integer(..) => *self,
			Inner::Float(f) => f.round().into()
		}
	}

	/// Returns the absolute value of `self`.
	pub fn abs(&self) -> Self {
		match self.0 {
			Inner::Integer(i) => i.abs().into(),
			Inner::Float(f) => f.abs().into()
		}
	}

	/// Try to parse a [`Number`] from the input with the given radix.
	pub fn from_str_radix(inp: &str, radix: u32) -> Result<Self, FromStrError> {
		if radix < 2 || radix > 36 {
			return Err(FromStrError::BadRadix(radix))
		}

		IntegerType::from_str_radix(inp.trim(), radix)
			.map(Self::from)
			.map_err(FromStrError::BadInteger)
	}

	/// Converts a [`Number`] into a string with the given radix.
	pub fn to_string_radix(&self, radix: u32) -> Result<String, ToStringRadixError> {
		if radix < 2 || radix > 36 {
			return Err(ToStringRadixError::InvalidRadix(radix))
		}

		let this = IntegerType::try_from(*self).map_err(ToStringRadixError::NotAnInteger)?;

		match radix {
			2 => Ok(format!("{:b}", this)),
			8 => Ok(format!("{:o}", this)),
			16 => Ok(format!("{:x}", this)),
			10 => Ok(format!("{}", this)),
			other => todo!("unsupported radix {}", other),
		}
	}

	/// Returns `self` to the power of the `rhs`.
	///
	/// Since Rust doesn't have a "power of" trait, this is is the replacement for it.
	pub fn pow(self, rhs: Self) -> Self {
		if self == Self::ONE || rhs == Self::ZERO {
			return Self::ONE;
		}

		match (self.0, rhs.0) {
			(Inner::Integer(l), Inner::Integer(r)) if 0 <= r && r <= (u32::MAX as IntegerType)
				=> l.wrapping_pow(r as u32).into(),
			(Inner::Integer(l), Inner::Integer(r)) => (l as FloatType).powf(r as FloatType).into(),
			(Inner::Integer(l), Inner::Float(r)) => (l as FloatType).powf(r).into(),
			(Inner::Float(l), Inner::Integer(r)) => l.powf(r as FloatType).into(),
			(Inner::Float(l), Inner::Float(r)) => l.powf(r).into()
		}
	}

	/// Sets `self` to `self` to the power of `rhs`.
	///
	/// Since Rust doesn't have a "power of assign" trait, this is is the replacement for it.
	#[inline]
	pub fn pow_assign(&mut self, rhs: Self) {
		*self = self.pow(rhs);
	}

	#[inline]
	pub fn is_nan(&self) -> bool {
		match self.0 {
			Inner::Integer(..) => false,
			Inner::Float(f) => f.is_nan()
		}
	}
}


impl PartialOrd for Number {
	#[inline]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
		Some(self.cmp(rhs))
	}
}

impl Ord for Number {
	fn cmp(&self, rhs: &Self) -> Ordering {
		use Inner::*;
		// TODO: somehow make an ordering and account for NaN
		match (self.0, rhs.0) {
			(Integer(l), Integer(r)) => l.cmp(&r),
			(Integer(l), Float(r)) => (l as FloatType).partial_cmp(&r).expect("bad cmp (i/f)"),
			(Float(l), Integer(r)) => l.partial_cmp(&(r as FloatType)).expect("bad cmp (f/i)"),
			(Float(l), Float(r)) => l.partial_cmp(&r).expect("bad cmp (f/f)"),
		}
	}
}

impl TryFrom<&str> for Number {
	type Error = FromStrError;

	/// Try to parse a number from a `&str`.
	///
	/// Leading and trailing whitespace is ignored, as is all `_`s.
	fn try_from(inp: &str) -> Result<Self, Self::Error> {
		use std::str::FromStr;

		let inp = inp.trim();

		// if we have underscores, delete them and try again. We don't want to have to convert
		// everything to a string in case a `_` doesn't exist, so we check for `_`'s existance first.
		if inp.find('_') != None {
			let mut inp = inp.to_string();

			while let Some(idx) = inp.rfind('_') {
				inp.remove(idx);
			}

			return Self::try_from(inp.as_str())
		}

		IntegerType::from_str(inp)
			.map(Self::from)
			.or_else(|_| FloatType::from_str(inp).map(Self::from))
			.map_err(FromStrError::BadFloat)
	}
}

/// An error that can occur when trying to parse a [`Number`] out of a string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FromStrError {
	/// If [`IntegerType`] couldn't be parsed.
	BadInteger(std::num::ParseIntError),

	/// If [`FloatType`] couldn't be parsed.
	BadFloat(std::num::ParseFloatError),

	/// An invalid radix was given.
	BadRadix(u32)
}

impl Display for FromStrError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			FromStrError::BadInteger(err) => Display::fmt(err, f),
			FromStrError::BadFloat(err) => Display::fmt(err, f),
			FromStrError::BadRadix(radix) => write!(f, "bad radix: {}", radix)
		}
	}
}

impl std::error::Error for FromStrError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			FromStrError::BadInteger(ref err) => Some(err),
			FromStrError::BadFloat(ref err) => Some(err),
			FromStrError::BadRadix(_) => None
		}
	}
}

/// The error that could occur when trying to [convert a number to a string with a radix](
/// #to_string_radix)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToStringRadixError {
	/// It's a bad radix.
	InvalidRadix(u32),

	/// It's not an integer.
	NotAnInteger(NotAnInteger)
}

impl Display for ToStringRadixError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			ToStringRadixError::InvalidRadix(radix) => write!(f, "invalid radix: {}", radix),
			ToStringRadixError::NotAnInteger(err) => Display::fmt(err, f)
		}		
	}
}

impl std::error::Error for ToStringRadixError {
	// Even though `NotAnInteger` technically references another error, it's not the _cause_ of
	// this exception, so we don't implement `source`.
}

/// The given number wasn't an integer when it should have been.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NotAnInteger(f64);

impl Display for NotAnInteger {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{} is not a whole number", self.0)
	}
}

impl std::error::Error for NotAnInteger {}

impl From<NotAnInteger> for crate::Error {
	fn from(n: NotAnInteger) -> Self {
		// TODO: remove "messaged"...
		ValueError::Messaged(n.to_string()).into()
	}
}

macro_rules! impl_try_from_eq {
	($($int:ty)*; $($float:ty)*) => {
		$(
			impl TryFrom<Number> for $int {
				type Error = NotAnInteger;
				fn try_from(num: Number) -> Result<Self, Self::Error> {
					match num.0 {
						Inner::Integer(n) => Ok(n as Self),
						Inner::Float(f) => Err(NotAnInteger(f))
					}
				}
			}

			impl PartialEq<$int> for Number {
				#[inline]
				fn eq(&self, rhs: &$int) -> bool {
					*self == Self::from(*rhs)
				}
			}
		)*

		$(
			impl PartialEq<$float> for Number {
				#[inline]
				fn eq(&self, rhs: &$float) -> bool {
					*self == Self::from(*rhs)
				}
			}
		)*
	};
}

impl_try_from_eq!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize; f32 f64);

impl From<FloatType> for Number {
	// note that if the given `f` is an integer, we instead construct an `Inner::Integer`.
	#[allow(clippy::float_cmp)]
	fn from(f: FloatType) -> Self {
		if f.is_finite() && f.floor() == f &&
			(IntegerType::MIN..IntegerType::MAX).contains(&(f as _))
		{
			Self(Inner::Integer(f as _))
		} else {
			Self(Inner::Float(f))
		}
	}
}

impl From<IntegerType> for Number {
	#[inline]
	fn from(n: IntegerType) -> Self {
		Self(Inner::Integer(n))
	}
}

impl From<FloatType> for Object {
	#[inline]
	fn from(f: FloatType) -> Self {
		Number::from(f).into()
	}
}

impl From<IntegerType> for Object {
	#[inline]
	fn from(n: IntegerType) -> Self {
		Number::from(n).into()
	}
}

impl From<Number> for FloatType {
	fn from(n: Number) -> Self {
		match n.0 {
			Inner::Integer(n) => n as _,
			Inner::Float(n) => n,
		}
	}
}

macro_rules! impl_from {
	($($int:ty)*; $($float:ty)*) => {
		$(
			impl From<$int> for Number {
				#[inline]
				fn from(num: $int) -> Self {
					Self::from(num as IntegerType)
				}
			}

			impl From<$int> for Object {
				#[inline]
				fn from(num: $int) -> Self {
					Number::from(num).into()
				}
			}
		)*
		$(
			impl From<$float> for Number {
				#[inline]
				fn from(num: $float) -> Self {
					Self::from(num as FloatType)
				}
			}

			impl From<$float> for Object {
				#[inline]
				fn from(num: $float) -> Self {
					Number::from(num).into()
				}
			}
		)*
	};
}

impl_from!{
	i8 i16 i32     i128 isize
	u8 u16 u32 u64 u128 usize;
	f32
}

macro_rules! impl_math_ops {
	($($trait:ident $trait_assign:ident $fn:ident $wrapping_fn:ident $fn_assign:ident)*) => {
		$(
			impl ops::$trait for Number {
				type Output = Self;

				fn $fn(self, rhs: Self) -> Self {
					use Inner::*;
					match (self.0, rhs.0) {
						// todo: make use of a big integer crate.
						(Integer(l), Integer(r)) => Self::from(l.$wrapping_fn(r)),
						(Integer(l), Float(r)) => Self::from((l as FloatType).$fn(r)),
						(Float(l), Integer(r)) => Self::from(l.$fn(r as FloatType)),
						(Float(l), Float(r)) => Self::from(l.$fn(r))
					}
				}
			}

			impl ops::$trait_assign for Number {
				#[inline]
				fn $fn_assign(&mut self, rhs: Self) {
					use ops::$trait;
					*self = (*self).$fn(rhs);
				}
			}
		)*
	};
}

impl_math_ops! {
	Add AddAssign add wrapping_add add_assign
	Sub SubAssign sub wrapping_sub sub_assign
	Mul MulAssign mul wrapping_mul mul_assign
}

impl ops::Div for Number {
	type Output = Self;
	/// Divide `self` by `divisor`.
	///
	/// If `divisor` is [zero](Number::ZERO), then `-INF`, `NAN`, or `INF` are returned based on the
	/// sign of `self`.
	fn div(self, divisor: Self) -> Self {
		if divisor == Self::ZERO {
			match self.cmp(&Self::ZERO) {
				Ordering::Less => -Self::INF,
				Ordering::Equal => Self::NAN,
				Ordering::Greater => Self::INF,
			}
		} else {
			// convert to a float because we want to allow for `1/2 = 0.5`
			Self::from(FloatType::from(self) / FloatType::from(divisor))
		}
	}
}

impl ops::DivAssign for Number {
	/// Divide `self` by `divisor`, in place.
	///
	/// See (Number::div)[#div] for more details on a divisor of [zero](Number::ZERO).
	#[inline]
	fn div_assign(&mut self, divisor: Self) {
		*self = *self / divisor;
	}
}

impl ops::Rem for Number {
	type Output = Self;

	/// Returns `this` modulo `divisor`.
	///
	/// If `divisor` is [zero](Number::ZERO), then `-INF`, `NAN`, or `INF` are returned based on the
	/// sign of `self`.
	fn rem(self, divisor: Self) -> Self {
		if divisor == Self::ZERO {
			match self.cmp(&Self::ZERO) {
				Ordering::Less => -Self::INF,
				Ordering::Equal => Self::NAN,
				Ordering::Greater => Self::INF,
			}
		} else {
			use Inner::*;
			match (self.0, divisor.0) {
				(Integer(l), Integer(r)) => Self::from(l.wrapping_rem(r)),
				(Integer(l), Float(r)) => Self::from(l as FloatType % r),
				(Float(l), Integer(r)) => Self::from(l % r as FloatType),
				(Float(l), Float(r)) => Self::from(l % r)
			}
		}
	}
}

impl ops::RemAssign for Number {
	/// Modulo `self` by `divisor`, in place.
	///
	/// If `divisor` is [zero](Number::ZERO), then `-INF`, `NAN`, or `INF` are used based on the sign
	/// of `self`.
	#[inline]
	fn rem_assign(&mut self, divisor: Self) {
		*self = *self % divisor;
	}
}

impl Number {
	/// If both numbers are integers, simply `&` them. If either isn't an integer, [`NotAnInteger`]
	/// is returned.
	pub fn try_bitand(self, rhs: Self) -> Result<Self, NotAnInteger> {
		Ok(Self::from(IntegerType::try_from(self)? & IntegerType::try_from(rhs)?))
	}

	/// If both numbers are integers, replace `self` with [`try_bitand`]'s result. If either isn't an
	/// integer, [`NotAnInteger`] is returned.
	#[inline]
	pub fn try_bitand_assign(&mut self, rhs: Self) -> Result<(), NotAnInteger> {
		*self = self.try_bitand(rhs)?;
		Ok(())
	}

	/// If both numbers are integers, simply `|` them. If either isn't an integer, [`NotAnInteger`]
	/// is returned.
	pub fn try_bitor(self, rhs: Self) -> Result<Self, NotAnInteger> {
		Ok(Self::from(IntegerType::try_from(self)? | IntegerType::try_from(rhs)?))
	}

	/// If both numbers are integers, replace `self` with [`try_bitor`]'s result. If either isn't an
	/// integer, [`NotAnInteger`] is returned.
	#[inline]
	pub fn try_bitor_assign(&mut self, rhs: Self) -> Result<(), NotAnInteger> {
		*self = self.try_bitor(rhs)?;
		Ok(())
	}

	/// If both numbers are integers, simply `^` them. If either isn't an integer, [`NotAnInteger`]
	/// is returned.
	pub fn try_bitxor(self, rhs: Self) -> Result<Self, NotAnInteger> {
		Ok(Self::from(IntegerType::try_from(self)? ^ IntegerType::try_from(rhs)?))
	}

	/// If both numbers are integers, replace `self` with [`try_bitxor`]'s result. If either isn't an
	/// integer, [`NotAnInteger`] is returned.
	#[inline]
	pub fn try_bitxor_assign(&mut self, rhs: Self) -> Result<(), NotAnInteger> {
		*self = self.try_bitxor(rhs)?;
		Ok(())
	}

	/// If both numbers are integers, simply `<<` them. If either isn't an integer, [`NotAnInteger`]
	/// is returned.
	pub fn try_shl(self, rhs: Self) -> Result<Self, NotAnInteger>{
		Ok(Self::from(IntegerType::try_from(self)?.wrapping_shl(u32::try_from(rhs)?)))
	}

	/// If both numbers are integers, replace `self` with [`try_shl`]'s result. If either isn't an
	/// integer, [`NotAnInteger`] is returned.
	#[inline]
	pub fn try_shl_assign(&mut self, rhs: Self) -> Result<(), NotAnInteger> {
		*self = self.try_shl(rhs)?;
		Ok(())
	}

	/// If both numbers are integers, simply `<<` them. If either isn't an integer, [`NotAnInteger`]
	/// is returned.
	pub fn try_shr(self, rhs: Self) -> Result<Self, NotAnInteger> {
		Ok(Self::from(IntegerType::try_from(self)?.wrapping_shr(u32::try_from(rhs)?)))
	}

	/// If both numbers are integers, replace `self` with [`try_shr`]'s result. If either isn't an
	/// integer, [`NotAnInteger`] is returned.
	#[inline]
	pub fn try_shr_assign(&mut self, rhs: Self) -> Result<(), NotAnInteger> {
		*self = self.try_shr(rhs)?;
		Ok(())
	}

	/// Try to perform `~`, returning [`NotAnInteger`] if `self` isn't an integer
	pub fn try_not(self) -> Result<Self, NotAnInteger> {
		Ok(Self::from(!IntegerType::try_from(self)?))
	}
}

impl ops::Neg for Number {
	type Output = Self;

	fn neg(self) -> Self {
		match self.0 {
			Inner::Integer(i) => Self::from(-i),
			Inner::Float(f) => Self::from(-f)
		}
	}
}

impl From<Number> for Text {
	#[inline]
	fn from(n: Number) -> Self {
		Self::from(n.to_string())
	}
}

impl From<Number> for Boolean {
	/// Convert a [`Number`] to a [`Boolean`]
	///
	/// [zero](Number::ZERO) is considered [`false`](Boolean::FALSE) while everything else is
	/// [`true`](Boolean::TRUE)
	#[inline]
	fn from(n: Number) -> Self {
		if n == Number::ZERO {
			Self::FALSE
		} else {
			Self::TRUE
		}
	}
}

/// Quest methods
impl Number {
	/// Inspects `this`.
	///
	/// This is identical to [`qs_at_text`](#qs_at_text).
	#[instrument(name="Number::inspect", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_inspect(this: &Object, args: Args) -> crate::Result<Object> {
		Self::qs_at_text(this, args)
	}

	/// Convert `this` to a [`Number`].
	///
	/// This simply returns the same object.
	#[instrument(name="Number::@num", level="trace", skip(this), fields(self=?this))]
	pub fn qs_at_num(this: &Object, _: Args) -> crate::Result<Object> {
		Ok(this.clone())
	}

	/// Converts `this` to a [`Text`], with an optional base parameter.
	///
	/// The base must be `2 <= base <= 36`. 
	#[instrument(name="Number::@text", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_at_text(this: &Object, args: Args) -> crate::Result<Object> {
		use std::convert::TryInto;
		let this = this.try_downcast::<Self>()?;

		if let Some(radix) = args.arg(0) {
			this.to_string_radix((*radix.call_downcast::<Self>()?).try_into()?)
				.map_err(|err| TypeError::Messaged(err.to_string()))
				.map_err(crate::Error::from)
				.map(Object::from)
		} else {
			Ok(Text::from(*this).into())
		}
	}

	/// Converts `this` to a [`Boolean`].
	///
	/// All values but [zero](Number::ZERO) are considered true.
	#[instrument(name="Number::@bool", level="trace", skip(this), fields(self=?this))]
	pub fn qs_at_bool(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(Boolean::from(*this).into())
	}

	/// Calling a number is simply an alias for [multiplication](#qs_mul).
	#[instrument(name="Number::()", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_call(this: &Object, args: Args) -> crate::Result<Object> {
		Self::qs_mul(this, args)
	}

	/// Hash a number.
	#[instrument(name="Number::hash", level="trace", skip(this), fields(self=?this))]
	pub fn qs_hash(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(crate::utils::hash(&*this).into())
	}

	/// Invert `this`'s sign.
	#[instrument(name="Number::-@", level="trace", skip(this), fields(self=?this))]
	pub fn qs_neg(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok((-*this).into())
	}

	/// Get the absolute value of `this`.
	#[instrument(name="Number::+@", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_pos(this: &Object, args: Args) -> crate::Result<Object> {
		Self::qs_abs(this, args)
	}

	/// Add `this` and the first argument.
	///
	/// # Arguments
	/// 1. (required, `@num`) The addend.
	#[instrument(name="Number::+", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_add(this: &Object, args: Args) -> crate::Result<Object> {
		let addend = args.try_arg(0)?.call_downcast::<Self>()?;
		let this = this.try_downcast::<Self>()?;

		Ok((*this + *addend).into())
	}

	/// Add `this` and the first argument, in place.
	///
	/// # Arguments
	/// 1. (required, `@num`) The addend.
	#[instrument(name="Number::+=", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_add_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let addend = *args.try_arg(0)?.call_downcast::<Self>()?;

		*this.try_downcast_mut::<Self>()? += addend;
		Ok(this.clone())
	}

	/// Subtract the the first argument from `this`.
	///
	/// # Arguments
	/// 1. (required, `@num`) The subtrahend.
	#[instrument(name="Number::-", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_sub(this: &Object, args: Args) -> crate::Result<Object> {
		let subtrahend = args.try_arg(0)?.call_downcast::<Self>()?;
		let this = this.try_downcast::<Self>()?;

		Ok((*this - *subtrahend).into())
	}

	/// Subtract the the first argument from `this`, in place.
	///
	/// # Arguments
	/// 1. (required, `@num`) The subtrahend.
	#[instrument(name="Number::-=", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_sub_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let subtrahend = *args.try_arg(0)?.call_downcast::<Self>()?;

		*this.try_downcast_mut::<Self>()? -= subtrahend;
		Ok(this.clone())
	}

	/// Multiply `this` and the first argument.
	///
	/// # Arguments
	/// 1. (required, `@num`) The multiplicand.
	#[instrument(name="Number::*", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_mul(this: &Object, args: Args) -> crate::Result<Object> {
		let multiplicand = args.try_arg(0)?.call_downcast::<Self>()?;
		let this = this.try_downcast::<Self>()?;

		Ok((*this * *multiplicand).into())
	}

	/// Multiply `this` and the first argument, in place.
	///
	/// # Arguments
	/// 1. (required, `@num`) The multiplicand.
	#[instrument(name="Number::*=", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_mul_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let multiplicand = *args.try_arg(0)?.call_downcast::<Self>()?;

		*this.try_downcast_mut::<Self>()? *= multiplicand;
		Ok(this.clone())

	}

	/// Divide `this` by the first argument.
	///
	/// See (Number::div)[#div] for more details on a divisor of [zero](Number::ZERO).
	///
	/// # Arguments
	/// 1. (required, `@num`) The divisor.
	#[instrument(name="Number::/", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_div(this: &Object, args: Args) -> crate::Result<Object> {
		let divisor = args.try_arg(0)?.call_downcast::<Self>()?;
		let this = this.try_downcast::<Self>()?;

		Ok((*this / *divisor).into())
	}

	/// Divide `this` by the first argument, in place.
	///
	/// See (Number::div)[#div] for more details on a divisor of [zero](Number::ZERO).
	///
	/// # Arguments
	/// 1. (required, `@num`) The divisor.
	#[instrument(name="Number::/=", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_div_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let divisor = *args.try_arg(0)?.call_downcast::<Self>()?;

		*this.try_downcast_mut::<Self>()? /= divisor;
		Ok(this.clone())

	}

	/// Modulo `this` by `divisor`.
	///
	/// See (Number::rem)[#rem] for more details on a divisor of [zero](Number::ZERO).
	///
	/// # Arguments
	/// 1. (required, `@num`) The divisor.
	#[instrument(name="Number::%", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_mod(this: &Object, args: Args) -> crate::Result<Object> {
		let divisor = args.try_arg(0)?.call_downcast::<Self>()?;
		let this = this.try_downcast::<Self>()?;

		Ok((*this % *divisor).into())
	}

	/// Modulo `this` by `divisor`, in place.
	///
	/// See (Number::rem)[#rem] for more details on a divisor of [zero](Number::ZERO).
	///
	/// # Arguments
	/// 1. (required, `@num`) The divisor.
	#[instrument(name="Number::%=", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_mod_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let divisor = *args.try_arg(0)?.call_downcast::<Self>()?;

		*this.try_downcast_mut::<Self>()? %= divisor;
		Ok(this.clone())
	}

	/// Raises `this` to the power of `exponent`.
	///
	/// # Arguments
	/// 1. (required, `@num`) The exponent.
	#[instrument(name="Number::**", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_pow(this: &Object, args: Args) -> crate::Result<Object> {
		let exponent = args.try_arg(0)?.call_downcast::<Self>()?;
		let this = this.try_downcast::<Self>()?;

		Ok(this.pow(*exponent).into())
	}

	/// Raises `this` to the power of `exponent`, in place.
	///
	/// # Arguments
	/// 1. (required, `@num`) The exponent.
	#[instrument(name="Number::**=", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_pow_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let exponent = *args.try_arg(0)?.call_downcast::<Self>()?;

		this.try_downcast_mut::<Self>()?.pow_assign(exponent);
		Ok(this.clone())
	}

	/// Bitwise NOT of `this`.
	///
	/// If `this` isn't a whole number, a [`ValueError`] is raised.
	#[instrument(name="Number::~", level="trace", skip(this), fields(self=?this))]
	pub fn qs_bitnot(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(this.try_not()?.into())
	}

	/// Bitwise AND of `this` and `other`.
	///
	/// If either `this` or `other` aren't a whole number, a [`ValueError`] is raised.
	///
	/// # Arguments
	/// 1. (required, `@num`) The other value.
	#[instrument(name="Number::&", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_bitand(this: &Object, args: Args) -> crate::Result<Object> {
		let other = args.try_arg(0)?.call_downcast::<Self>()?;
		let this = this.try_downcast::<Self>()?;

		Ok(this.try_bitand(*other)?.into())
	}

	/// Bitwise AND of `this` and `other`, in place.
	///
	/// If either `this` or `other` aren't a whole number, a [`ValueError`] is raised.
	///
	/// # Arguments
	/// 1. (required, `@num`) The other value.
	#[instrument(name="Number::&=", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_bitand_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let other = *args.try_arg(0)?.call_downcast::<Self>()?;

		this.try_downcast_mut::<Self>()?.try_bitand_assign(other)?;
		Ok(this.clone())
	}

	/// Bitwise OR of `this` and `other`.
	///
	/// If either `this` or `other` aren't a whole number, a [`ValueError`] is raised.
	///
	/// # Arguments
	/// 1. (required, `@num`) The other value.
	#[instrument(name="Number::|", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_bitor(this: &Object, args: Args) -> crate::Result<Object> {
		let other = args.try_arg(0)?.call_downcast::<Self>()?;
		let this = this.try_downcast::<Self>()?;

		Ok(this.try_bitor(*other)?.into())
	}

	/// Bitwise OR of `this` and `other`, in place.
	///
	/// If either `this` or `other` aren't a whole number, a [`ValueError`] is raised.
	///
	/// # Arguments
	/// 1. (required, `@num`) The other value.
	#[instrument(name="Number::|=", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_bitor_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let other = *args.try_arg(0)?.call_downcast::<Self>()?;

		this.try_downcast_mut::<Self>()?.try_bitor_assign(other)?;
		Ok(this.clone())
	}

	/// Bitwise XOR of `this` and `other`.
	///
	/// If either `this` or `other` aren't a whole number, a [`ValueError`] is raised.
	///
	/// # Arguments
	/// 1. (required, `@num`) The other value.
	#[instrument(name="Number::^", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_bitxor(this: &Object, args: Args) -> crate::Result<Object> {
		let other = args.try_arg(0)?.call_downcast::<Self>()?;
		let this = this.try_downcast::<Self>()?;

		Ok(this.try_bitxor(*other)?.into())
	}

	/// Bitwise XOR of `this` and `other`, in place.
	///
	/// If either `this` or `other` aren't a whole number, a [`ValueError`] is raised.
	///
	/// # Arguments
	/// 1. (required, `@num`) The other value.
	#[instrument(name="Number::^=", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_bitxor_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let other = *args.try_arg(0)?.call_downcast::<Self>()?;

		this.try_downcast_mut::<Self>()?.try_bitxor_assign(other)?;
		Ok(this.clone())
	}

	/// Shift `this` left by `amnt`.
	///
	/// If either `this` or `amnt` aren't a whole number, a [`ValueError`] is raised.
	///
	/// # Arguments
	/// 1. (required, `@num`) The value to shift by.
	#[instrument(name="Number::<<", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_shl(this: &Object, args: Args) -> crate::Result<Object> {
		let amnt = args.try_arg(0)?.call_downcast::<Self>()?;
		let this = this.try_downcast::<Self>()?;

		Ok(this.try_shl(*amnt)?.into())
	}

	/// Shift `this` left by `amnt`, in place.
	///
	/// If either `this` or `amnt` aren't a whole number, a [`ValueError`] is raised.
	///
	/// # Arguments
	/// 1. (required, `@num`) The value to shift by.
	#[instrument(name="Number::<<=", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_shl_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let amnt = *args.try_arg(0)?.call_downcast::<Self>()?;

		this.try_downcast_mut::<Self>()?.try_shl_assign(amnt)?;
		Ok(this.clone())
	}

	/// Shift `this` right by `amnt`.
	///
	/// If either `this` or `amnt` aren't a whole number, a [`ValueError`] is raised.
	///
	/// # Arguments
	/// 1. (required, `@num`) The value to shift by.
	#[instrument(name="Number::>>", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_shr(this: &Object, args: Args) -> crate::Result<Object> {
		let amnt = args.try_arg(0)?.call_downcast::<Self>()?;
		let this = this.try_downcast::<Self>()?;

		Ok(this.try_shr(*amnt)?.into())
	}

	/// Shift `this` right by `amnt`, in place.
	///
	/// If either `this` or `amnt` aren't a whole number, a [`ValueError`] is raised.
	///
	/// # Arguments
	/// 1. (required, `@num`) The value to shift by.
	#[instrument(name="Number::>>=", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_shr_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let amnt = *args.try_arg(0)?.call_downcast::<Self>()?;

		this.try_downcast_mut::<Self>()?.try_shr_assign(amnt)?;
		Ok(this.clone())
	}

	/// Get the absolute value of `this`.
	#[instrument(name="Number::abs", level="trace", skip(this), fields(self=?this))]
	pub fn qs_abs(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(this.abs().into())
	}

	/// See if a `this` is equal to the first argument.
	///
	/// Unlike most methods, the first argument is not implicitly converted to a [`Number`] first.
	///
	/// # Arguments
	/// 1. (required) The other object to compare against.
	#[instrument(name="Number::==", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_eql(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?.downcast::<Self>();
		let this = this.try_downcast::<Self>()?;

		Ok(rhs.map(|rhs| *rhs == *this).unwrap_or(false).into())
	}

	/// Compares `this` to the first argument.
	///
	/// # Arguments
	/// 1. (required, `@num`) The value to compare against.
	#[instrument(name="Number::<=>", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_cmp(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;
		let rhs = args.try_arg(0)?.downcast::<Self>();

		Ok(rhs.map(|rhs| this.cmp(&*rhs).into()).unwrap_or_default())
	}

	/// Returns `this`, rounded down.
	#[instrument(name="Number::floor", level="trace", skip(this), fields(self=?this))]
	pub fn qs_floor(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(this.floor().into())
	}

	/// Returns `this`, rounded up.
	#[instrument(name="Number::ceil", level="trace", skip(this), fields(self=?this))]
	pub fn qs_ceil(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(this.ceil().into())
	}

	/// Returns `this`, rounded towards the nearest integer. (`##.5` rounds away from zero.)
	#[instrument(name="Number::round", level="trace", skip(this), fields(self=?this))]
	pub fn qs_round(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(this.round().into())
	}

	/// Gets the square root of `this`
	#[instrument(name="Number::sqrt", level="trace", skip(this), fields(self=?this))]
	pub fn qs_sqrt(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(FloatType::from(*this).sqrt().into())
	}
}

impl Convertible for Number {
	const CONVERT_FUNC: crate::Literal = crate::Literal::AT_NUM;
}

impl_object_type!{
	for Number 
{
	fn new_object(self) -> Object {
		use lazy_static::lazy_static;
		use std::collections::HashMap;
		use parking_lot::RwLock;

		lazy_static! {
			static ref OBJECTS: RwLock<HashMap<Number, Object>> = RwLock::new(HashMap::new());
		}

		if let Some(obj) = OBJECTS.read().get(&self) {
			return obj.deep_clone();
		}

		let mut objs = OBJECTS.write();

		objs.entry(self)
			.or_insert_with(|| Object::new_with_parent(self, vec![Number::mapping()]))
			.deep_clone()
	}
}

[(init_parent super::Basic super::Comparable) (parents super::Basic) (no_convert)]:
	"PI" => const Self::PI,
	"E" => const Self::E,
	"NAN" => const Self::NAN,
	"INF" => const Self::INF,

	"@text" => method Self::qs_at_text,
	"inspect" => method Self::qs_inspect,
	"@num" => method Self::qs_at_num,
	"@bool" => method Self::qs_at_bool,
	"hash" => method Self::qs_hash,

	"+"  => method Self::qs_add,    "+="  => method Self::qs_add_assign,
	"-"  => method Self::qs_sub,    "-="  => method Self::qs_sub_assign,
	"*"  => method Self::qs_mul,    "*="  => method Self::qs_mul_assign,
	"/"  => method Self::qs_div,    "/="  => method Self::qs_div_assign,
	"%"  => method Self::qs_mod,    "%="  => method Self::qs_mod_assign,
	"**" => method Self::qs_pow,    "**=" => method Self::qs_pow_assign,
	"&"  => method Self::qs_bitand, "&="  => method Self::qs_bitand_assign,
	"|"  => method Self::qs_bitor,  "|="  => method Self::qs_bitor_assign,
	"^"  => method Self::qs_bitxor, "^="  => method Self::qs_bitxor_assign,
	"<<" => method Self::qs_shl,    "<<=" => method Self::qs_shl_assign,
	">>" => method Self::qs_shr,    ">>=" => method Self::qs_shr_assign,

	"-@"  => method Self::qs_neg,
	"+@"  => method Self::qs_pos,
	"~"   => method Self::qs_bitnot,
	"abs" => method Self::qs_abs,
	"<=>" => method Self::qs_cmp,
	"()"  => method Self::qs_call,
	"=="  => method Self::qs_eql,

	"round" => method Self::qs_round,
	"ceil"  => method Self::qs_ceil,
	"floor" => method Self::qs_floor,
	"sqrt"  => method Self::qs_sqrt,
	"chr" => method |this, _| {
		Ok((u8::try_from(this.try_downcast::<Self>()?.floor()).unwrap() as char)
			.to_string().into())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use rand::random;

	#[test]
	fn clone_and_copy() {
		let x = Number::new(12);
		assert_eq!(x, x.clone());
		assert_eq!(x, x);
	}

	#[allow(clippy::float_cmp)]
	mod qs {
		use super::*;

		#[test]
		fn constants() {
			use crate::types::ObjectType;

			macro_rules! assert_exists_eq {
				($key:literal, $val:expr) => {
					assert_eq!(
						*Number::mapping().get_attr_lit($key).unwrap()
							.downcast::<Number>().unwrap(),
						$val);
				}
			}

			assert_exists_eq!("PI", Number::PI);
			assert_exists_eq!("E", Number::E);
			assert_exists_eq!("INF", Number::INF);
			assert!(Number::mapping().get_attr_lit("NAN").unwrap()
				.downcast::<Number>().unwrap().is_nan());
		}

		#[test]
		fn at_text() {
			assert_call_eq!(Number::qs_at_text(0) -> Text, *"0");
			assert_call_eq!(Number::qs_at_text(0.0) -> Text, *"0");
			assert_call_eq!(Number::qs_at_text(-0.0) -> Text, *"0");
			assert_call_eq!(Number::qs_at_text(1) -> Text, *"1");
			assert_call_eq!(Number::qs_at_text(12.3) -> Text, *"12.3");
			assert_call_eq!(Number::qs_at_text(-1223.129) -> Text, *"-1223.129");
			assert_call_eq!(Number::qs_at_text(Number::INF) -> Text, *"inf");
			assert_call_eq!(Number::qs_at_text(-Number::INF) -> Text, *"-inf");

			for _ in 0..1000 {
				let n = random::<IntegerType>();
				let f = random::<FloatType>();
				assert_call_eq!(Number::qs_at_text(n) -> Text, *n.to_string());
				assert_call_eq!(Number::qs_at_text(f) -> Text, *f.to_string());
			}

			assert_call_err!(Number::qs_at_text(0, 0), crate::Error::TypeError(..));
			assert_call_err!(Number::qs_at_text(0, 1), crate::Error::TypeError(..));
			assert_call_err!(Number::qs_at_text(0, 37), crate::Error::TypeError(..));
			assert_call_err!(Number::qs_at_text(0, 38), crate::Error::TypeError(..));
			assert_call_err!(Number::qs_at_text(12.3, 10), crate::Error::TypeError(..));
			assert_call_err!(Number::qs_at_text(Number::INF, 10), crate::Error::TypeError(..));

			assert_call_eq!(Number::qs_at_text(0b10110110, 2) -> Text, *"10110110");
			assert_call_eq!(Number::qs_at_text(120, 10) -> Text, *"120");
			assert_call_eq!(Number::qs_at_text(0o17214, 8) -> Text, *"17214");
			assert_call_eq!(Number::qs_at_text(0xff1e24, 16) -> Text, *"ff1e24");
				
			for _ in 0..1000 {
				let n = random::<IntegerType>();
				assert_call_eq!(Number::qs_at_text(n, 2) -> Text, *format!("{:b}", n));
				assert_call_eq!(Number::qs_at_text(n, 8) -> Text, *format!("{:o}", n));
				assert_call_eq!(Number::qs_at_text(n, 10) -> Text, *format!("{}", n));
				assert_call_eq!(Number::qs_at_text(n, 16) -> Text, *format!("{:x}", n));
			}

			assert_call_idempotent!(Number::qs_at_text(12));
			assert_call_idempotent!(Number::qs_at_text(12, 8));
		}

		#[test]
		fn inspect() {
			assert_call_eq!(Number::qs_inspect(0) -> Text, *"0");
			assert_call_eq!(Number::qs_inspect(0.0) -> Text, *"0");
			assert_call_eq!(Number::qs_inspect(-0.0) -> Text, *"0");
			assert_call_eq!(Number::qs_inspect(1) -> Text, *"1");
			assert_call_eq!(Number::qs_inspect(12.3) -> Text, *"12.3");
			assert_call_eq!(Number::qs_inspect(-1223.129) -> Text, *"-1223.129");
			assert_call_eq!(Number::qs_inspect(Number::INF) -> Text, *"inf");
			assert_call_eq!(Number::qs_inspect(-Number::INF) -> Text, *"-inf");

			for _ in 0..1000 {
				let n = random::<IntegerType>();
				let f = random::<FloatType>();
				assert_call_eq!(Number::qs_inspect(n) -> Text, *n.to_string());
				assert_call_eq!(Number::qs_inspect(f) -> Text, *f.to_string());
			}

			assert_call_idempotent!(Number::qs_inspect(12));
		}

		#[test]
		fn at_bool() {
			assert_call_eq!(Number::qs_at_bool(0) -> Boolean, false);
			assert_call_eq!(Number::qs_at_bool(1) -> Boolean, true);
			assert_call_eq!(Number::qs_at_bool(123) -> Boolean, true);
			assert_call_eq!(Number::qs_at_bool(Number::INF) -> Boolean, true);

			for _ in 0..1000 {
				let n = random::<IntegerType>();
				let f = random::<FloatType>();
				assert_call_eq!(Number::qs_at_bool(n) -> Boolean, n != 0);
				assert_call_eq!(Number::qs_at_bool(f) -> Boolean, f != 0.0);
			}

			assert_call_idempotent!(Number::qs_at_bool(12));
		}

		#[test]
		fn at_num() {
			assert_call_eq!(Number::qs_at_num(0) -> Number, 0);
			assert_call_eq!(Number::qs_at_num(1) -> Number, 1);
			assert_call_eq!(Number::qs_at_num(123) -> Number, 123);
			assert_call_eq!(Number::qs_at_num(Number::INF) -> Number, Number::INF);

			for _ in 0..1000 {
				let n = random::<IntegerType>();
				let f = random::<FloatType>();
				assert_call_eq!(Number::qs_at_num(n) -> Number, n);
				assert_call_eq!(Number::qs_at_num(f) -> Number, f);
			}

			// ensure that calling `at_num` doesn't modify the underlying type
			let obj = Object::from(194);
			let dup = Number::qs_at_num(&obj, args!()).unwrap();
			obj.downcast_mut::<Number>().map(|mut n| *n = Number::from(123)).unwrap();
			assert_eq!(*dup.downcast::<Number>().unwrap(), 123);

			assert_call_non_idempotent!(Number::qs_at_num(12));
		}

		#[test]
		fn cmp() {
			#[derive(Debug, Clone)]
			struct Dummy;
			impl_object_type! { for Dummy [(parents crate::types::Basic)]: }

			assert_call_eq!(Number::qs_cmp(1, 1) -> Number,  0);
			assert_call_eq!(Number::qs_cmp(1, 0) -> Number,  1);
			assert_call_eq!(Number::qs_cmp(1, 2) -> Number, -1);
			assert_call_eq!(Number::qs_cmp(1, Dummy) -> Null, crate::types::Null);

			assert_call_eq!(Number::qs_cmp(12, Number::INF) -> Number, -1);
			assert_call_eq!(Number::qs_cmp(-5, Number::INF) -> Number, -1);
			assert_call_eq!(Number::qs_cmp(Number::INF, Number::INF) -> Number, 0);
			assert_call_eq!(Number::qs_cmp(-Number::INF, -Number::INF) -> Number, 0);
			assert_call_eq!(Number::qs_cmp(-Number::INF, 12) -> Number, -1);

			for _ in 0..1000 {
				let n1 = random::<IntegerType>();
				let n2 = random::<IntegerType>();
				let f1 = random::<FloatType>();
				let f2 = random::<FloatType>();

				if f1.is_nan() || f2.is_nan() { continue; }

				assert_call_eq!(Number::qs_cmp(n1, n2) -> Number, Number::from(n1.cmp(&n2)));
				assert_call_eq!(Number::qs_cmp(n1, f1) -> Number,
					Number::from((n1 as FloatType).partial_cmp(&f1).unwrap()));

				assert_call_eq!(Number::qs_cmp(f1, n1) -> Number,
					Number::from(f1.partial_cmp(&(n1 as FloatType)).unwrap()));
				assert_call_eq!(Number::qs_cmp(f1, f2) -> Number,
					Number::from(f1.partial_cmp(&f2).unwrap()));
			}

			assert_call_missing_parameter!(Number::qs_cmp(0), 0);
			assert_call_idempotent!(Number::qs_cmp(12, 45));
		}

		#[test]
		fn eql() {
			assert_call_eq!(Number::qs_eql(0, 0) -> Boolean, true);
			assert_call_eq!(Number::qs_eql(0.0, 0) -> Boolean, true);
			assert_call_eq!(Number::qs_eql(0, 0.0) -> Boolean, true);
			assert_call_eq!(Number::qs_eql(0.0, 0.0) -> Boolean, true);
			assert_call_eq!(Number::qs_eql(-0.0, 0.0) -> Boolean, true);

			assert_call_eq!(Number::qs_eql(Number::INF, 0.0) -> Boolean, false);
			assert_call_eq!(Number::qs_eql(Number::INF, -1.0) -> Boolean, false);
			assert_call_eq!(Number::qs_eql(Number::INF, 1.0) -> Boolean, false);
			assert_call_eq!(Number::qs_eql(1.0, Number::INF) -> Boolean, false);
			assert_call_eq!(Number::qs_eql(Number::INF, -Number::INF) -> Boolean, false);
			assert_call_eq!(Number::qs_eql(Number::INF, Number::INF) -> Boolean, true);

			for _ in 0..1000 {
				let n = random::<IntegerType>();
				let f = random::<FloatType>();

				assert_call_eq!(Number::qs_eql(n, n) -> Boolean, true);
				assert_call_eq!(Number::qs_eql(f, f) -> Boolean, true);
				assert_call_eq!(Number::qs_eql(n, f) -> Boolean, n as FloatType == f);
				assert_call_eq!(Number::qs_eql(f, n) -> Boolean, n as FloatType == f);
			}

			assert_call_missing_parameter!(Number::qs_eql(0), 0);
			assert_call_idempotent!(Number::qs_eql(12, 45));
		}

		#[test]
		fn hash() {
			macro_rules! hash {
				($n:expr) => { crate::utils::hash(&Number::from($n)) };
			}

			assert_call_eq!(Number::qs_hash(0.0) -> Number, hash!(0.0));
			assert_call_eq!(Number::qs_hash(0.0) -> Number, hash!(-0.0));

			for _ in 0..1000 {
				let n1 = random::<FloatType>();
				let n2 = random::<FloatType>();
				let f1 = random::<FloatType>();
				let f2 = random::<FloatType>();

				assert_call_eq!(Number::qs_hash(n1) -> Number, hash!(n1));
				assert_eq!(
					call_unwrap!(Number::qs_hash(n1) -> Number; |n| *n) == 
						call_unwrap!(Number::qs_hash(n2) -> Number; |n| *n),
					Number::from(n1) == Number::from(n2)
				);

				assert_call_eq!(Number::qs_hash(f1) -> Number, hash!(f1));
				assert_eq!(
					call_unwrap!(Number::qs_hash(f1) -> Number; |n| *n) == 
						call_unwrap!(Number::qs_hash(f2) -> Number; |n| *n),
					Number::from(f1) == Number::from(f2)
				);
			}

			assert_call_idempotent!(Number::qs_hash(12));
		}

		#[test]
		fn sqrt() {
			assert_call_eq!(Number::qs_sqrt(16) -> Number, 4);
			assert_call_eq!(Number::qs_sqrt(-0.0) -> Number, 0.0);
			assert_call_eq!(Number::qs_sqrt(12.3) -> Number, (12.3 as FloatType).sqrt());
			assert_call_eq!(Number::qs_sqrt(12.7) -> Number, (12.7 as FloatType).sqrt());
			assert_call_eq!(Number::qs_sqrt(Number::INF) -> Number, Number::INF);
			assert_call!(Number::qs_sqrt(-Number::INF) -> Number; |n| Number::is_nan(&n));

			for _ in 0..1000 {
				let n = random::<IntegerType>().abs();
				let f = random::<FloatType>().abs();

				assert_call_eq!(Number::qs_sqrt(f) -> Number, f.sqrt());
				assert_call_eq!(Number::qs_sqrt(n) -> Number, (n as FloatType).sqrt());
			}

			assert_call_idempotent!(Number::qs_sqrt(12));
			assert_call_idempotent!(Number::qs_sqrt(12.3));

			// TODO: negative numbers
			assert_call!(Number::qs_sqrt(-12); |n| Number::is_nan(&n));
		}

		#[test]
		fn ceil() {
			assert_call_eq!(Number::qs_ceil(12) -> Number, 12);
			assert_call_eq!(Number::qs_ceil(-0.0) -> Number, -0.0);
			assert_call_eq!(Number::qs_ceil(12.3) -> Number, 13);
			assert_call_eq!(Number::qs_ceil(12.7) -> Number, 13);
			assert_call_eq!(Number::qs_ceil(-12.3) -> Number, -12);
			assert_call_eq!(Number::qs_ceil(-12.7) -> Number, -12);
			assert_call_eq!(Number::qs_ceil(Number::INF) -> Number, Number::INF);
			assert_call_eq!(Number::qs_ceil(-Number::INF) -> Number, -Number::INF);

			for _ in 0..1000 {
				let n = random::<IntegerType>();
				let f = random::<FloatType>();
				assert_call_eq!(Number::qs_ceil(f) -> Number, f.ceil());
				assert_call_eq!(Number::qs_ceil(n) -> Number, n);
			}

			assert_call_idempotent!(Number::qs_ceil(12));
			assert_call_idempotent!(Number::qs_ceil(12.3));
		}

		#[test]
		fn floor() {
			assert_call_eq!(Number::qs_floor(12) -> Number, 12);
			assert_call_eq!(Number::qs_floor(-0.0) -> Number, -0.0);
			assert_call_eq!(Number::qs_floor(12.3) -> Number, 12);
			assert_call_eq!(Number::qs_floor(12.7) -> Number, 12);
			assert_call_eq!(Number::qs_floor(-12.3) -> Number, -13);
			assert_call_eq!(Number::qs_floor(-12.7) -> Number, -13);
			assert_call_eq!(Number::qs_floor(Number::INF) -> Number, Number::INF);
			assert_call_eq!(Number::qs_floor(-Number::INF) -> Number, -Number::INF);

			for _ in 0..1000 {
				let n = random::<IntegerType>();
				let f = random::<FloatType>();
				assert_call_eq!(Number::qs_floor(f) -> Number, f.floor());
				assert_call_eq!(Number::qs_floor(n) -> Number, n);
			}

			assert_call_idempotent!(Number::qs_floor(12));
			assert_call_idempotent!(Number::qs_floor(12.3));
		}

		#[test]
		fn round() {
			assert_call_eq!(Number::qs_round(12) -> Number, 12);
			assert_call_eq!(Number::qs_round(-0.0) -> Number, -0.0);
			assert_call_eq!(Number::qs_round(12.3) -> Number, 12);
			assert_call_eq!(Number::qs_round(12.7) -> Number, 13);
			assert_call_eq!(Number::qs_round(-12.3) -> Number, -12);
			assert_call_eq!(Number::qs_round(-12.7) -> Number, -13);
			assert_call_eq!(Number::qs_round(Number::INF) -> Number, Number::INF);
			assert_call_eq!(Number::qs_round(-Number::INF) -> Number, -Number::INF);

			for _ in 0..1000 {
				let n = random::<IntegerType>();
				let f = random::<FloatType>();
				assert_call_eq!(Number::qs_round(f) -> Number, f.round());
				assert_call_eq!(Number::qs_round(n) -> Number, n);
			}

			assert_call_idempotent!(Number::qs_round(12));
			assert_call_idempotent!(Number::qs_round(12.3));
		}

		#[test]
		fn abs() {
			assert_call_eq!(Number::qs_abs(12) -> Number, 12);
			assert_call_eq!(Number::qs_abs(-0.0) -> Number, 0.0);
			assert_call_eq!(Number::qs_abs(12.3) -> Number, 12.3);
			assert_call_eq!(Number::qs_abs(12.7) -> Number, 12.7);
			assert_call_eq!(Number::qs_abs(-12.3) -> Number, 12.3);
			assert_call_eq!(Number::qs_abs(-12.7) -> Number, 12.7);
			assert_call_eq!(Number::qs_abs(Number::INF) -> Number, Number::INF);
			assert_call_eq!(Number::qs_abs(-Number::INF) -> Number, Number::INF);

			for _ in 0..1000 {
				let n = random::<IntegerType>();
				let f = random::<FloatType>();
				assert_call_eq!(Number::qs_abs(f) -> Number, f.abs());
				assert_call_eq!(Number::qs_abs(n) -> Number, n.abs());
			}

			assert_call_idempotent!(Number::qs_abs(-12));
			assert_call_idempotent!(Number::qs_abs(12.3));
		}

		#[test]
		fn pos() {
			assert_call_eq!(Number::qs_pos(12) -> Number, 12);
			assert_call_eq!(Number::qs_pos(-0.0) -> Number, 0.0);
			assert_call_eq!(Number::qs_pos(12.3) -> Number, 12.3);
			assert_call_eq!(Number::qs_pos(12.7) -> Number, 12.7);
			assert_call_eq!(Number::qs_pos(-12.3) -> Number, 12.3);
			assert_call_eq!(Number::qs_pos(-12.7) -> Number, 12.7);
			assert_call_eq!(Number::qs_pos(Number::INF) -> Number, Number::INF);
			assert_call_eq!(Number::qs_pos(-Number::INF) -> Number, Number::INF);

			for _ in 0..1000 {
				let n = random::<IntegerType>();
				let f = random::<FloatType>();
				assert_call_eq!(Number::qs_pos(f) -> Number, f.abs());
				assert_call_eq!(Number::qs_pos(n) -> Number, n.abs());
			}

			assert_call_idempotent!(Number::qs_pos(-12));
			assert_call_idempotent!(Number::qs_pos(12.3));
		}

		#[test]
		fn neg() {
			assert_call_eq!(Number::qs_neg(12) -> Number, -12);
			assert_call_eq!(Number::qs_neg(-0.0) -> Number, 0.0);
			assert_call_eq!(Number::qs_neg(12.3) -> Number, -12.3);
			assert_call_eq!(Number::qs_neg(12.7) -> Number, -12.7);
			assert_call_eq!(Number::qs_neg(-12.3) -> Number, 12.3);
			assert_call_eq!(Number::qs_neg(-12.7) -> Number, 12.7);
			assert_call_eq!(Number::qs_neg(Number::INF) -> Number, -Number::INF);
			assert_call_eq!(Number::qs_neg(-Number::INF) -> Number, Number::INF);

			for _ in 0..1000 {
				let n = random::<IntegerType>();
				let f = random::<FloatType>();
				assert_call_eq!(Number::qs_neg(f) -> Number, -f);
				assert_call_eq!(Number::qs_neg(n) -> Number, -n);
			}

			assert_call_idempotent!(Number::qs_neg(-12));
			assert_call_idempotent!(Number::qs_pos(12.3));
		}

		#[test]
		fn add() {
			assert_call_eq!(Number::qs_add(12, 19) -> Number, 12 + 19);
			assert_call_eq!(Number::qs_add(12, -123) -> Number, 12 + -123);
			assert_call_eq!(Number::qs_add(0, -123) -> Number, -123);
			assert_call_eq!(Number::qs_add(Number::INF, 123) -> Number, Number::INF);
			assert_call_eq!(Number::qs_add(-123, Number::INF) -> Number, Number::INF);
			assert_call_eq!(Number::qs_add(Number::INF, Number::INF) -> Number, Number::INF);
			assert_call!(Number::qs_add(Number::INF, -Number::INF) -> Number; |n| Number::is_nan(&n));
			assert_call!(Number::qs_add(-Number::INF, Number::INF) -> Number; |n| Number::is_nan(&n));
			assert_call_eq!(Number::qs_add(-Number::INF, -Number::INF) -> Number, -Number::INF);

			for _ in 0..1000 {
				let n1 = random::<IntegerType>();
				let n2 = random::<IntegerType>();
				let f1 = random::<FloatType>();
				let f2 = random::<FloatType>();

				assert_call_eq!(Number::qs_add(n1, n1) -> Number, n1.wrapping_add(n1));
				assert_call_eq!(Number::qs_add(n1, n2) -> Number, n1.wrapping_add(n2));
				assert_call_eq!(Number::qs_add(n1, f1) -> Number, n1 as FloatType + f1);
				assert_call_eq!(Number::qs_add(n1, f2) -> Number, n1 as FloatType + f2);

				assert_call_eq!(Number::qs_add(n2, n1) -> Number, n2.wrapping_add(n1));
				assert_call_eq!(Number::qs_add(n2, n2) -> Number, n2.wrapping_add(n2));
				assert_call_eq!(Number::qs_add(n2, f1) -> Number, n2 as FloatType + f1);
				assert_call_eq!(Number::qs_add(n2, f2) -> Number, n2 as FloatType + f2);

				assert_call_eq!(Number::qs_add(f1, n1) -> Number, f1 + n1 as FloatType);
				assert_call_eq!(Number::qs_add(f1, n2) -> Number, f1 + n2 as FloatType);
				assert_call_eq!(Number::qs_add(f1, f1) -> Number, f1 + f1);
				assert_call_eq!(Number::qs_add(f1, f2) -> Number, f1 + f2);

				assert_call_eq!(Number::qs_add(f2, n1) -> Number, f2 + n1 as FloatType);
				assert_call_eq!(Number::qs_add(f2, n2) -> Number, f2 + n2 as FloatType);
				assert_call_eq!(Number::qs_add(f2, f1) -> Number, f2 + f1);
				assert_call_eq!(Number::qs_add(f2, f2) -> Number, f2 + f2);
			}

			assert_call_missing_parameter!(Number::qs_add(0), 0);
			assert_call_idempotent!(Number::qs_add(0, 1));
		}

		#[test]
		fn add_assign() {
			assert_call_non_idempotent!(Number::qs_add_assign(0, 1));
			assert_call_missing_parameter!(Number::qs_add_assign(0), 0);
		}

		#[test]
		fn sub() {
			assert_call_eq!(Number::qs_sub(12, 19) -> Number, 12 - 19);
			assert_call_eq!(Number::qs_sub(12, -123) -> Number, 12 - -123);
			assert_call_eq!(Number::qs_sub(0, -123) -> Number, 123);
			assert_call_eq!(Number::qs_sub(Number::INF, 123) -> Number, Number::INF);
			assert_call_eq!(Number::qs_sub(-123, Number::INF) -> Number, -Number::INF);
			assert_call!(Number::qs_sub(Number::INF, Number::INF) -> Number; |n| Number::is_nan(&n));
			assert_call_eq!(Number::qs_sub(Number::INF, -Number::INF) -> Number, Number::INF);
			assert_call_eq!(Number::qs_sub(-Number::INF, Number::INF) -> Number, -Number::INF);
			assert_call!(Number::qs_sub(-Number::INF, -Number::INF) -> Number; |n| Number::is_nan(&n));

			for _ in 0..1000 {
				let n1 = random::<IntegerType>();
				let n2 = random::<IntegerType>();
				let f1 = random::<FloatType>();
				let f2 = random::<FloatType>();

				assert_call_eq!(Number::qs_sub(n1, n1) -> Number, 0);
				assert_call_eq!(Number::qs_sub(n1, n2) -> Number, n1.wrapping_sub(n2));
				assert_call_eq!(Number::qs_sub(n1, f1) -> Number, n1 as FloatType - f1);
				assert_call_eq!(Number::qs_sub(n1, f2) -> Number, n1 as FloatType - f2);

				assert_call_eq!(Number::qs_sub(n2, n1) -> Number, n2.wrapping_sub(n1));
				assert_call_eq!(Number::qs_sub(n2, n2) -> Number, 0);
				assert_call_eq!(Number::qs_sub(n2, f1) -> Number, n2 as FloatType - f1);
				assert_call_eq!(Number::qs_sub(n2, f2) -> Number, n2 as FloatType - f2);

				assert_call_eq!(Number::qs_sub(f1, n1) -> Number, f1 - n1 as FloatType);
				assert_call_eq!(Number::qs_sub(f1, n2) -> Number, f1 - n2 as FloatType);
				assert_call_eq!(Number::qs_sub(f1, f1) -> Number, 0);
				assert_call_eq!(Number::qs_sub(f1, f2) -> Number, f1 - f2);

				assert_call_eq!(Number::qs_sub(f2, n1) -> Number, f2 - n1 as FloatType);
				assert_call_eq!(Number::qs_sub(f2, n2) -> Number, f2 - n2 as FloatType);
				assert_call_eq!(Number::qs_sub(f2, f1) -> Number, f2 - f1);
				assert_call_eq!(Number::qs_sub(f2, f2) -> Number, 0);
			}

			assert_call_missing_parameter!(Number::qs_sub(0), 0);
			assert_call_idempotent!(Number::qs_sub(0, 1));
		}

		#[test]
		fn sub_assign() {
			assert_call_non_idempotent!(Number::qs_sub_assign(0, 1));
			assert_call_missing_parameter!(Number::qs_sub_assign(0), 0);
		}

		#[test]
		fn mul() {
			assert_call_eq!(Number::qs_mul(12, 19) -> Number, 12 * 19);
			assert_call_eq!(Number::qs_mul(12, -123) -> Number, 12 * -123);
			assert_call_eq!(Number::qs_mul(0, -123) -> Number, 0);
			assert_call_eq!(Number::qs_mul(Number::INF, 123) -> Number, Number::INF);
			assert_call_eq!(Number::qs_mul(-123, Number::INF) -> Number, -Number::INF);
			assert_call_eq!(Number::qs_mul(Number::INF, Number::INF) -> Number, Number::INF);
			assert_call_eq!(Number::qs_mul(Number::INF, -Number::INF) -> Number, -Number::INF);
			assert_call_eq!(Number::qs_mul(-Number::INF, Number::INF) -> Number, -Number::INF);
			assert_call_eq!(Number::qs_mul(-Number::INF, -Number::INF) -> Number, Number::INF);

			for _ in 0..1000 {
				let n1 = random::<IntegerType>();
				let n2 = random::<IntegerType>();
				let f1 = random::<FloatType>();
				let f2 = random::<FloatType>();

				assert_call_eq!(Number::qs_mul(n1, n1) -> Number, n1.wrapping_mul(n1));
				assert_call_eq!(Number::qs_mul(n1, n2) -> Number, n1.wrapping_mul(n2));
				assert_call_eq!(Number::qs_mul(n1, f1) -> Number, n1 as FloatType * f1);
				assert_call_eq!(Number::qs_mul(n1, f2) -> Number, n1 as FloatType * f2);

				assert_call_eq!(Number::qs_mul(n2, n1) -> Number, n2.wrapping_mul(n1));
				assert_call_eq!(Number::qs_mul(n2, n2) -> Number, n2.wrapping_mul(n2));
				assert_call_eq!(Number::qs_mul(n2, f1) -> Number, n2 as FloatType * f1);
				assert_call_eq!(Number::qs_mul(n2, f2) -> Number, n2 as FloatType * f2);

				assert_call_eq!(Number::qs_mul(f1, n1) -> Number, f1 * n1 as FloatType);
				assert_call_eq!(Number::qs_mul(f1, n2) -> Number, f1 * n2 as FloatType);
				assert_call_eq!(Number::qs_mul(f1, f1) -> Number, f1 * f1);
				assert_call_eq!(Number::qs_mul(f1, f2) -> Number, f1 * f2);

				assert_call_eq!(Number::qs_mul(f2, n1) -> Number, f2 * n1 as FloatType);
				assert_call_eq!(Number::qs_mul(f2, n2) -> Number, f2 * n2 as FloatType);
				assert_call_eq!(Number::qs_mul(f2, f1) -> Number, f2 * f1);
				assert_call_eq!(Number::qs_mul(f2, f2) -> Number, f2 * f2);
			}

			assert_call_missing_parameter!(Number::qs_mul(0), 0);
			assert_call_idempotent!(Number::qs_mul(0, 1));
		}

		#[test]
		fn mul_assign() {
			assert_call_non_idempotent!(Number::qs_mul_assign(0, 1));
			assert_call_missing_parameter!(Number::qs_mul_assign(0), 0);
		}

		#[test]
		fn call() {
			assert_call_eq!(Number::qs_call(12, 19) -> Number, 12 * 19);
			assert_call_eq!(Number::qs_call(12, -123) -> Number, 12 * -123);
			assert_call_eq!(Number::qs_call(0, -123) -> Number, 0);
			assert_call_eq!(Number::qs_call(Number::INF, 123) -> Number, Number::INF);
			assert_call_eq!(Number::qs_call(-123, Number::INF) -> Number, -Number::INF);
			assert_call_eq!(Number::qs_call(Number::INF, Number::INF) -> Number, Number::INF);
			assert_call_eq!(Number::qs_call(Number::INF, -Number::INF) -> Number, -Number::INF);
			assert_call_eq!(Number::qs_call(-Number::INF, Number::INF) -> Number, -Number::INF);
			assert_call_eq!(Number::qs_call(-Number::INF, -Number::INF) -> Number, Number::INF);

			for _ in 0..1000 {
				let n1 = random::<IntegerType>();
				let n2 = random::<IntegerType>();
				let f1 = random::<FloatType>();
				let f2 = random::<FloatType>();

				assert_call_eq!(Number::qs_call(n1, n1) -> Number, n1.wrapping_mul(n1));
				assert_call_eq!(Number::qs_call(n1, n2) -> Number, n1.wrapping_mul(n2));
				assert_call_eq!(Number::qs_call(n1, f1) -> Number, n1 as FloatType * f1);
				assert_call_eq!(Number::qs_call(n1, f2) -> Number, n1 as FloatType * f2);

				assert_call_eq!(Number::qs_call(n2, n1) -> Number, n2.wrapping_mul(n1));
				assert_call_eq!(Number::qs_call(n2, n2) -> Number, n2.wrapping_mul(n2));
				assert_call_eq!(Number::qs_call(n2, f1) -> Number, n2 as FloatType * f1);
				assert_call_eq!(Number::qs_call(n2, f2) -> Number, n2 as FloatType * f2);

				assert_call_eq!(Number::qs_call(f1, n1) -> Number, f1 * n1 as FloatType);
				assert_call_eq!(Number::qs_call(f1, n2) -> Number, f1 * n2 as FloatType);
				assert_call_eq!(Number::qs_call(f1, f1) -> Number, f1 * f1);
				assert_call_eq!(Number::qs_call(f1, f2) -> Number, f1 * f2);

				assert_call_eq!(Number::qs_call(f2, n1) -> Number, f2 * n1 as FloatType);
				assert_call_eq!(Number::qs_call(f2, n2) -> Number, f2 * n2 as FloatType);
				assert_call_eq!(Number::qs_call(f2, f1) -> Number, f2 * f1);
				assert_call_eq!(Number::qs_call(f2, f2) -> Number, f2 * f2);
			}

			assert_call_missing_parameter!(Number::qs_call(0), 0);
			assert_call_idempotent!(Number::qs_call(0, 1));
		}

		#[test]
		fn div() {
			assert_call_eq!(Number::qs_div(149, 19) -> Number, 149.0 / 19.0);
			assert_call_eq!(Number::qs_div(12, -123) -> Number, 12.0 / -123.0);
			assert_call_eq!(Number::qs_div(0, -123) -> Number, 0.0 / -123.0);
			assert_call_eq!(Number::qs_div(Number::INF, 123) -> Number, Number::INF);
			assert_call_eq!(Number::qs_div(-123, Number::INF) -> Number, 0);

			assert_call!(Number::qs_div(0, 0) -> Number; |n| Number::is_nan(&n));
			assert_call_eq!(Number::qs_div(1, 0) -> Number, Number::INF);
			assert_call_eq!(Number::qs_div(-1, 0) -> Number, -Number::INF);

			assert_call!(Number::qs_div(Number::INF, Number::INF) -> Number; |n| Number::is_nan(&n));
			assert_call!(Number::qs_div(Number::INF, -Number::INF) -> Number; |n| Number::is_nan(&n));
			assert_call!(Number::qs_div(-Number::INF, Number::INF) -> Number; |n| Number::is_nan(&n));
			assert_call!(Number::qs_div(-Number::INF, -Number::INF) -> Number; |n| Number::is_nan(&n));

			for _ in 0..1000 {
				let n1 = random::<IntegerType>();
				let n2 = random::<IntegerType>();
				let f1 = random::<FloatType>();
				let f2 = random::<FloatType>();

				// we already check these cases above.
				if n1 == 0 || n2 == 0 || f1 == 0.0 || f2 == 0.0 {
					continue;
				}

				assert_call_eq!(Number::qs_div(n1, n1) -> Number, 1);
				assert_call_eq!(Number::qs_div(n1, n2) -> Number, n1 as FloatType / n2 as FloatType);
				assert_call_eq!(Number::qs_div(n1, f1) -> Number, n1 as FloatType / f1);
				assert_call_eq!(Number::qs_div(n1, f2) -> Number, n1 as FloatType / f2);

				assert_call_eq!(Number::qs_div(n2, n1) -> Number, n2 as FloatType / n1 as FloatType);
				assert_call_eq!(Number::qs_div(n2, n2) -> Number, 1);
				assert_call_eq!(Number::qs_div(n2, f1) -> Number, n2 as FloatType / f1);
				assert_call_eq!(Number::qs_div(n2, f2) -> Number, n2 as FloatType / f2);

				assert_call_eq!(Number::qs_div(f1, n1) -> Number, f1 / n1 as FloatType);
				assert_call_eq!(Number::qs_div(f1, n2) -> Number, f1 / n2 as FloatType);
				assert_call_eq!(Number::qs_div(f1, f1) -> Number, 1);
				assert_call_eq!(Number::qs_div(f1, f2) -> Number, f1 / f2);

				assert_call_eq!(Number::qs_div(f2, n1) -> Number, f2 / n1 as FloatType);
				assert_call_eq!(Number::qs_div(f2, n2) -> Number, f2 / n2 as FloatType);
				assert_call_eq!(Number::qs_div(f2, f1) -> Number, f2 / f1);
				assert_call_eq!(Number::qs_div(f2, f2) -> Number, 1);
			}

			assert_call_missing_parameter!(Number::qs_div(0), 0);
			assert_call_idempotent!(Number::qs_div(0, 1));
		}

		#[test]
		fn div_assign() {
			assert_call_non_idempotent!(Number::qs_div_assign(0, 1));
			assert_call_missing_parameter!(Number::qs_div_assign(0), 0);
		}

		#[test]
		fn r#mod() {
			assert_call_eq!(Number::qs_mod(149, 19) -> Number, 149.0 % 19.0);
			assert_call_eq!(Number::qs_mod(12, -123) -> Number, 12.0 % 123.0);
			assert_call_eq!(Number::qs_mod(0, -123) -> Number, 0.0 % -123.0);
			assert_call!(Number::qs_mod(Number::INF, 123) -> Number; |n| Number::is_nan(&n));
			assert_call_eq!(Number::qs_mod(-123, Number::INF) -> Number, -123);

			assert_call!(Number::qs_mod(0, 0) -> Number; |n| Number::is_nan(&n));
			assert_call_eq!(Number::qs_mod(1, 0) -> Number, Number::INF);
			assert_call_eq!(Number::qs_mod(-1, 0) -> Number, -Number::INF);

			assert_call!(Number::qs_mod(Number::INF, Number::INF) -> Number; |n| Number::is_nan(&n));
			assert_call!(Number::qs_mod(Number::INF, -Number::INF) -> Number; |n| Number::is_nan(&n));
			assert_call!(Number::qs_mod(-Number::INF, Number::INF) -> Number; |n| Number::is_nan(&n));
			assert_call!(Number::qs_mod(-Number::INF, -Number::INF) -> Number; |n| Number::is_nan(&n));

			for _ in 0..1000 {
				let n1 = random::<IntegerType>();
				let n2 = random::<IntegerType>();
				let f1 = random::<FloatType>();
				let f2 = random::<FloatType>();

				// we already check these cases above.
				if n1 == 0 || n2 == 0 || f1 == 0.0 || f2 == 0.0 {
					continue;
				}

				assert_call_eq!(Number::qs_mod(n1, n1) -> Number, n1.wrapping_rem(n1));
				assert_call_eq!(Number::qs_mod(n1, n2) -> Number, n1.wrapping_rem(n2));
				assert_call_eq!(Number::qs_mod(n1, f1) -> Number, n1 as FloatType % f1);
				assert_call_eq!(Number::qs_mod(n1, f2) -> Number, n1 as FloatType % f2);

				assert_call_eq!(Number::qs_mod(n2, n1) -> Number, n2.wrapping_rem(n1));
				assert_call_eq!(Number::qs_mod(n2, n2) -> Number, n2.wrapping_rem(n2));
				assert_call_eq!(Number::qs_mod(n2, f1) -> Number, n2 as FloatType % f1);
				assert_call_eq!(Number::qs_mod(n2, f2) -> Number, n2 as FloatType % f2);

				assert_call_eq!(Number::qs_mod(f1, n1) -> Number, f1 % n1 as FloatType);
				assert_call_eq!(Number::qs_mod(f1, n2) -> Number, f1 % n2 as FloatType);
				assert_call_eq!(Number::qs_mod(f1, f1) -> Number, f1 % f1);
				assert_call_eq!(Number::qs_mod(f1, f2) -> Number, f1 % f2);

				assert_call_eq!(Number::qs_mod(f2, n1) -> Number, f2 % n1 as FloatType);
				assert_call_eq!(Number::qs_mod(f2, n2) -> Number, f2 % n2 as FloatType);
				assert_call_eq!(Number::qs_mod(f2, f1) -> Number, f2 % f1);
				assert_call_eq!(Number::qs_mod(f2, f2) -> Number, f2 % f2);
			}

			assert_call_missing_parameter!(Number::qs_mod(0), 0);
			assert_call_idempotent!(Number::qs_mod(0xff, 0xee));
		}

		#[test]
		fn mod_assign() {
			assert_call_non_idempotent!(Number::qs_mod_assign(0, 1));
			assert_call_missing_parameter!(Number::qs_mod_assign(0), 0);
		}

		#[test]
		fn pow() {
			assert_call_eq!(Number::qs_pow(149, 19) -> Number, (149 as IntegerType).wrapping_pow(19));
			assert_call_eq!(Number::qs_pow(12, -123) -> Number, (12.0 as FloatType).powf(-123.0));
			assert_call_eq!(Number::qs_pow(0, -123) -> Number, (0.0 as FloatType).powf(-123.0));
			assert_call_eq!(Number::qs_pow(Number::INF, 123) -> Number, Number::INF);
			assert_call_eq!(Number::qs_pow(0.1, Number::INF) -> Number, 0);
			assert_call_eq!(Number::qs_pow(1.1, Number::INF) -> Number, Number::INF);
			assert_call_eq!(Number::qs_pow(1, Number::INF) -> Number, 1);

			assert_call_eq!(Number::qs_pow(0, 0) -> Number, 1);
			assert_call_eq!(Number::qs_pow(1, 0) -> Number, 1);
			assert_call_eq!(Number::qs_pow(-1, 0) -> Number, 1);
			assert_call_eq!(Number::qs_pow(Number::INF, 0) -> Number, 1);
			assert_call_eq!(Number::qs_pow(12, Number::INF) -> Number, Number::INF);
			assert_call_eq!(Number::qs_pow(0, Number::INF) -> Number, 0);

			assert_call_eq!(Number::qs_pow(Number::INF, Number::INF) -> Number, Number::INF);
			assert_call_eq!(Number::qs_pow(Number::INF, -Number::INF) -> Number, 0);
			assert_call_eq!(Number::qs_pow(-Number::INF, Number::INF) -> Number, Number::INF);
			assert_call_eq!(Number::qs_pow(-Number::INF, -Number::INF) -> Number, 0);

			for _ in 0..1000 {
				let n1 = random::<IntegerType>().abs();
				let n2 = random::<u32>();
				let f1 = random::<FloatType>().abs();
				let f2 = random::<FloatType>();

				// we already check these cases above.
				if n1 == 0 || n2 == 0 || f1 == 0.0 || f2 == 0.0 {
					continue;
				}

				assert_call_eq!(Number::qs_pow(n1, n2) -> Number, n1.wrapping_pow(n2));
				assert_call_eq!(Number::qs_pow(n1, f1) -> Number, (n1 as FloatType).powf(f1));
				assert_call_eq!(Number::qs_pow(n1, f2) -> Number, (n1 as FloatType).powf(f2));

				assert_call_eq!(Number::qs_pow(f1, n1) -> Number, f1.powf(n1 as FloatType));
				assert_call_eq!(Number::qs_pow(f1, n2) -> Number, f1.powf(n2 as FloatType));
				assert_call_eq!(Number::qs_pow(f1, f1) -> Number, f1.powf(f1));
				assert_call_eq!(Number::qs_pow(f1, f2) -> Number, f1.powf(f2));
			}

			// TODO: check for imaginary numbers
			assert_call!(Number::qs_pow(-1, 0.5) -> Number; |n| Number::is_nan(&n));

			assert_call_missing_parameter!(Number::qs_pow(0), 0);
			assert_call_idempotent!(Number::qs_pow(12, 4));
		}

		#[test]
		fn pow_assign() {
			assert_call_non_idempotent!(Number::qs_pow_assign(0, 1));
			assert_call_missing_parameter!(Number::qs_pow_assign(0), 0);
		}

		#[test]
		fn bitnot() {
			assert_call_eq!(Number::qs_bitnot(12) -> Number, !12);
			assert_call_eq!(Number::qs_bitnot(-14) -> Number, !-14);

			assert_call_err!(Number::qs_bitnot(12.3), crate::Error::ValueError(..));
			assert_call_err!(Number::qs_bitnot(-12.9), crate::Error::ValueError(..));
			assert_call_err!(Number::qs_bitnot(Number::INF), crate::Error::ValueError(..));

			for _ in 0..1000 {
				let n = random::<IntegerType>();

				assert_call_eq!(Number::qs_bitnot(n) -> Number, !n);
			}

			assert_call_idempotent!(Number::qs_bitnot(12));
		}

		#[test]
		fn bitand() {
			assert_call_eq!(Number::qs_bitand(12, 912) -> Number, (12 as IntegerType) & 912);
			assert_call_eq!(Number::qs_bitand(-512, 14) -> Number, (-512 as IntegerType) & 14);
			assert_call_eq!(Number::qs_bitand(0xff1e24, 0x129fa) -> Number,
				(0xff1e24 as IntegerType) & 0x129fa);

			assert_call_err!(Number::qs_bitand(12.3, 0x12), crate::Error::ValueError(..));
			assert_call_err!(Number::qs_bitand(0x93, -12.9), crate::Error::ValueError(..));
			assert_call_err!(Number::qs_bitand(Number::INF, 0x44), crate::Error::ValueError(..));

			for _ in 0..1000 {
				let n1 = random::<IntegerType>();
				let n2 = random::<IntegerType>();

				assert_call_eq!(Number::qs_bitand(n1, n2) -> Number, n1 & n2);
				assert_call_eq!(Number::qs_bitand(n2, n1) -> Number, n1 & n2);
			}

			assert_call_missing_parameter!(Number::qs_bitand(0), 0);
			assert_call_idempotent!(Number::qs_bitand(12, 14));
		}

		#[test]
		fn bitand_assign() {
			assert_call_non_idempotent!(Number::qs_bitand_assign(0, 1));
			assert_call_missing_parameter!(Number::qs_bitand_assign(0), 0);
		}

		#[test]
		fn bitor() {
			assert_call_eq!(Number::qs_bitor(12, 912) -> Number, (12 as IntegerType) | 912);
			assert_call_eq!(Number::qs_bitor(-512, 14) -> Number, (-512 as IntegerType) | 14);
			assert_call_eq!(Number::qs_bitor(0xff1e24, 0x129fa) -> Number,
				(0xff1e24 as IntegerType) | 0x129fa);

			assert_call_err!(Number::qs_bitor(12.3, 0x54), crate::Error::ValueError(..));
			assert_call_err!(Number::qs_bitor(0x89, -12.9), crate::Error::ValueError(..));
			assert_call_err!(Number::qs_bitor(Number::INF, 0xe2), crate::Error::ValueError(..));

			for _ in 0..1000 {
				let n1 = random::<IntegerType>();
				let n2 = random::<IntegerType>();

				assert_call_eq!(Number::qs_bitor(n1, n2) -> Number, n1 | n2);
				assert_call_eq!(Number::qs_bitor(n2, n1) -> Number, n1 | n2);
			}

			assert_call_missing_parameter!(Number::qs_bitor(0), 0);
			assert_call_idempotent!(Number::qs_bitor(12, 14));
		}

		#[test]
		fn bitor_assign() {
			assert_call_non_idempotent!(Number::qs_bitor_assign(0, 1));
			assert_call_missing_parameter!(Number::qs_bitor_assign(0), 0);
		}

		#[test]
		fn bitxor() {
			assert_call_eq!(Number::qs_bitxor(12, 912) -> Number, (12 as IntegerType) ^ 912);
			assert_call_eq!(Number::qs_bitxor(-512, 14) -> Number, (-512 as IntegerType) ^ 14);
			assert_call_eq!(Number::qs_bitxor(0xff1e24, 0x129fa) -> Number,
				(0xff1e24 as IntegerType) ^ 0x129fa);

			assert_call_err!(Number::qs_bitxor(12.3, 0xfe), crate::Error::ValueError(..));
			assert_call_err!(Number::qs_bitxor(0xed, -12.9), crate::Error::ValueError(..));
			assert_call_err!(Number::qs_bitxor(Number::INF, 0x17), crate::Error::ValueError(..));

			for _ in 0..1000 {
				let n1 = random::<IntegerType>();
				let n2 = random::<IntegerType>();

				assert_call_eq!(Number::qs_bitxor(n1, n2) -> Number, n1 ^ n2);
				assert_call_eq!(Number::qs_bitxor(n2, n1) -> Number, n1 ^ n2);
			}

			assert_call_missing_parameter!(Number::qs_bitxor(0), 0);
			assert_call_idempotent!(Number::qs_bitxor(12, 14));
		}

		#[test]
		fn bitxor_assign() {
			assert_call_non_idempotent!(Number::qs_bitxor_assign(0, 1));
			assert_call_missing_parameter!(Number::qs_bitxor_assign(0), 0);
		}

		#[test]
		fn shl() {
			assert_call_eq!(Number::qs_shl(912, 12) -> Number, (912 as IntegerType).wrapping_shl(12));
			assert_call_eq!(Number::qs_shl(-512, 4) -> Number, (-512 as IntegerType).wrapping_shl(4));
			assert_call_eq!(Number::qs_shl(0xff1e24, 10) -> Number,
				(0xff1e24 as IntegerType).wrapping_shl(10));

			assert_call_err!(Number::qs_shl(12.3, 0xfe), crate::Error::ValueError(..));
			assert_call_err!(Number::qs_shl(0xed, -12.9), crate::Error::ValueError(..));
			assert_call_err!(Number::qs_shl(Number::INF, 0x17), crate::Error::ValueError(..));

			for _ in 0..1000 {
				let n1 = random::<IntegerType>();
				let n2 = random::<u32>() % 100; // to make it more a realistic shift amnt.
				let n3 = random::<u32>();

				assert_call_eq!(Number::qs_shl(n1, n2) -> Number, n1.wrapping_shl(n2));
				assert_call_eq!(Number::qs_shl(n1, n3) -> Number, n1.wrapping_shl(n3));
			}

			assert_call_missing_parameter!(Number::qs_shl(0), 0);
			assert_call_idempotent!(Number::qs_shl(12, 14));
		}

		#[test]
		fn shl_assign() {
			assert_call_non_idempotent!(Number::qs_shl_assign(0, 1));
			assert_call_missing_parameter!(Number::qs_shl_assign(0), 0);
		}

		#[test]
		fn shr() {
			assert_call_eq!(Number::qs_shr(912, 12) -> Number, (912 as IntegerType).wrapping_shr(12));
			assert_call_eq!(Number::qs_shr(-512, 4) -> Number, (-512 as IntegerType).wrapping_shr(4));
			assert_call_eq!(Number::qs_shr(0xff1e24, 10) -> Number,
				(0xff1e24 as IntegerType).wrapping_shr(10));

			assert_call_err!(Number::qs_shr(12.3, 0xfe), crate::Error::ValueError(..));
			assert_call_err!(Number::qs_shr(0xed, -12.9), crate::Error::ValueError(..));
			assert_call_err!(Number::qs_shr(Number::INF, 0x17), crate::Error::ValueError(..));

			for _ in 0..1000 {
				let n1 = random::<IntegerType>();
				let n2 = random::<u32>() % 100; // to make it more a realistic shift amnt.
				let n3 = random::<u32>();

				assert_call_eq!(Number::qs_shr(n1, n2) -> Number, n1.wrapping_shr(n2));
				assert_call_eq!(Number::qs_shr(n1, n3) -> Number, n1.wrapping_shr(n3));
			}

			assert_call_missing_parameter!(Number::qs_shr(0), 0);
			assert_call_idempotent!(Number::qs_shr(12, 14));
		}

		#[test]
		fn shr_assign() {
			assert_call_non_idempotent!(Number::qs_shr_assign(0, 1));
			assert_call_missing_parameter!(Number::qs_shr_assign(0), 0);
		}
	}

	#[test]
	fn default() {
		assert_eq!(Number::default(), Number::ZERO);
	}

	#[test]
	fn from_str_radix() {
		// normal numbers
		assert_eq!(Number::from_str_radix("12", 10).unwrap(), Number(Inner::Integer(12)));
		assert_eq!(Number::from_str_radix("093", 10).unwrap(), Number(Inner::Integer(93)));
		assert_eq!(Number::from_str_radix("000", 10).unwrap(), Number(Inner::Integer(0)));
		assert_eq!(Number::from_str_radix("0110110", 2).unwrap(), Number(Inner::Integer(0b0110110)));
		assert_eq!(Number::from_str_radix("17214", 8).unwrap(), Number(Inner::Integer(0o17214)));
		assert_eq!(Number::from_str_radix("ff1e24", 16).unwrap(), Number(Inner::Integer(0xff1e24)));

		// negative numbers
		assert_eq!(Number::from_str_radix("-134", 10).unwrap(), Number(Inner::Integer(-134)));
		assert_eq!(Number::from_str_radix("-000", 10).unwrap(), Number(Inner::Integer(-0)));
		assert_eq!(Number::from_str_radix("-10110110", 2).unwrap(), -Number(Inner::Integer(0b10110110)));
		assert_eq!(Number::from_str_radix("-17214", 8).unwrap(), Number(Inner::Integer(-0o17214)));
		assert_eq!(Number::from_str_radix("-ff1e24", 16).unwrap(), Number(Inner::Integer(-0xff1e24)));

		// invalid bases
		assert_eq!(Number::from_str_radix("0", 0).unwrap_err(), FromStrError::BadRadix(0));
		assert_eq!(Number::from_str_radix("0", 1).unwrap_err(), FromStrError::BadRadix(1));
		assert_eq!(Number::from_str_radix("0", 37).unwrap_err(), FromStrError::BadRadix(37));
	}

	#[test]
	fn try_from() {
		// integers
		assert_eq!(Number::try_from("0").unwrap(), Number(Inner::Integer(0)));
		assert_eq!(Number::try_from("12").unwrap(), Number(Inner::Integer(12)));
		assert_eq!(Number::try_from("93").unwrap(), Number(Inner::Integer(93)));
		assert_eq!(Number::try_from("-1952").unwrap(), Number(Inner::Integer(-1952)));
		assert_eq!(Number::try_from("1e8").unwrap(), Number(Inner::Integer(1e8 as _)));
		assert_eq!(Number::try_from("1.5e+12").unwrap(), Number(Inner::Integer(1.5e12 as _)));

		// floats
		assert_eq!(Number::try_from("12.3").unwrap(), Number(Inner::Float(12.3)));
		assert_eq!(Number::try_from("-12.3").unwrap(), Number(Inner::Float(-12.3)));
		assert_eq!(Number::try_from("1E-8").unwrap(), Number(Inner::Float(1e-8)));

		// numbers with extra character we can strip
		assert_eq!(Number::try_from("  123\t\n").unwrap(), Number(Inner::Integer(123)));
		assert_eq!(Number::try_from("1_000_000").unwrap(), Number(Inner::Integer(1_000_000)));

		// bad numbers
		assert!(matches!(Number::try_from("invalid").unwrap_err(), FromStrError::BadFloat(..)));
		assert!(matches!(Number::try_from("1.2.3").unwrap_err(), FromStrError::BadFloat(..)));
		assert!(matches!(Number::try_from("12e3e4").unwrap_err(), FromStrError::BadFloat(..)));
		assert!(matches!(Number::try_from("").unwrap_err(), FromStrError::BadFloat(..)));
		assert!(matches!(Number::try_from(" ").unwrap_err(), FromStrError::BadFloat(..)));
	}
}

//! The [`Number`] type in Quest.

use std::convert::TryFrom;
use std::fmt::{self, Debug, Display, Formatter};
use std::cmp::Ordering;
use crate::{Object, Args};
use crate::types::{Text, Boolean};
use std::hash::{Hash, Hasher};
use crate::error::{ValueError};

/// The type used by [`Number`] to keep track of integers.
pub type IntegerType = i64;

/// The tpye used by [`Number`] to keep track of floats.
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
	fn eq(&self, rhs: &Number) -> bool {
		use Inner::*;
		match (self.0, rhs.0) {
			(Integer(l), Integer(r)) => l == r,
			(Float(l), Float(r)) => l == r || (l - r) < FloatType::EPSILON,
			_ => false
		}
	}
}

impl Hash for Number {
	#[inline]
	fn hash<H: Hasher>(&self, h: &mut H) {
		// in the future, we should probably change how floats hash
		match self.0 {
			Inner::Integer(i) => i.hash(h),
			Inner::Float(f) => f.to_bits().hash(h)
		}
	}
}

impl Default for Number {
	/// The default number is [zero](Number::ZERO).
	#[inline]
	fn default() -> Number {
		Number::ZERO
	}
}

impl Debug for Number {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			match self.0 {
				Inner::Integer(n) => write!(f, "Integer({:?})", n),
				Inner::Float(n) => write!(f, "Float({:?})", n),
			}
		} else {
			Display::fmt(self, f)
		}
	}
}

impl Display for Number {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self.0 {
			Inner::Integer(n) => Display::fmt(&n, f),
			Inner::Float(n) => Display::fmt(&n, f),
		}
	}
}

impl Number {
	/// The number zero.
	pub const ZERO: Self = Number(Inner::Integer(0 as IntegerType));

	/// The number one.
	pub const ONE: Self = Number(Inner::Integer(1 as IntegerType));

	/// The mathematical constant π.
	pub const PI: Self = Number(Inner::Float(std::f64::consts::PI));

	/// The mathematical constant e.
	pub const E: Self = Number(Inner::Float(std::f64::consts::E));

	/// The concept of "not a number".
	pub const NAN: Self = Number(Inner::Float(f64::NAN));

	/// Infinity!
	pub const INF: Self = Number(Inner::Float(f64::INFINITY));

	/// Create a new number.
	#[inline]
	pub fn new<N: Into<Self>>(num: N) -> Self {
		num.into()
	}

	/// Rounds `self` to the next highest integer (nothing's done if `self` is an integer).
	#[inline]
	pub fn ceil(self) -> IntegerType {
		match self.0 {
			Inner::Integer(i) => i,
			Inner::Float(f) => f.ceil() as _
		}
	}

	/// Rounds `self` to the next lowest integer (nothing's done if `self` is an integer).
	#[inline]
	pub fn floor(self) -> IntegerType {
		match self.0 {
			Inner::Integer(i) => i,
			Inner::Float(f) => f.floor() as _
		}
	}


	/// Rounds `self` to the nearest integer (nothing's done if `self` is an integer).
	#[inline]
	pub fn round(self) -> IntegerType {
		match self.0 {
			Inner::Integer(i) => i,
			Inner::Float(f) => f.round() as _
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
		let this = IntegerType::try_from(*self).map_err(ToStringRadixError::NotAnInteger)?;

		match radix {
         2 => Ok(format!("{:b}", this)),
         8 => Ok(format!("{:o}", this)),
         16 => Ok(format!("{:x}", this)),
         10 => Ok(format!("{}", this)),
         radix @ 0 | radix @ 1 => Err(ToStringRadixError::InvalidRadix(radix)),
         other => todo!("unsupported radix {}", other),
		}
	}

	/// Returns the absolute value of `self`.
	#[inline]
	pub fn abs(self) -> Self {
		match self.0 {
			Inner::Integer(i) => Self::from(i.abs()),
			Inner::Float(f) => Self::from(f.abs())
		}
	}

	/// Returns `self` to the power of the `rhs`.
	///
	/// Since Rust doesn't have a "power of" trait, this is is the replacement for it.
	#[inline]
	pub fn pow(mut self, rhs: Self) -> Self {
		self.pow_assign(rhs);
		self
	}

	/// Sets `self` to `self` to the power of `rhs`.
	///
	/// Replaces  Rust doesn't have a "power of assign" trait, this is is the replacement for it.
	pub fn pow_assign(&mut self, rhs: Self) {
		match (self.0, rhs.0) {
			(Inner::Integer(l), Inner::Integer(r)) if 0 <= r && r <= (u32::MAX as IntegerType)
				=> *self = l.pow(r as u32).into(),
			(Inner::Integer(l), Inner::Integer(r))
				=> *self = (l as FloatType).powf(r as FloatType).into(),
			(Inner::Integer(l), Inner::Float(r)) => *self = (l as FloatType).powf(r).into(),
			(Inner::Float(l), Inner::Integer(r)) => *self = l.powf(r as FloatType).into(),
			(Inner::Float(l), Inner::Float(r)) => *self = l.powf(r).into()
		}
	}

	pub fn is_nan(self) -> bool {
		match self.0 {
			Inner::Integer(_) => false,
			Inner::Float(f) => f.is_nan()
		}
	}
}


impl PartialOrd for Number {
	#[inline]
	fn partial_cmp(&self, rhs: &Number) -> Option<Ordering> {
		Some(self.cmp(rhs))
	}
}

impl Ord for Number {
	fn cmp(&self, rhs: &Number) -> Ordering {
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

impl TryFrom<&'_ str> for Number {
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
#[derive(Debug, Clone, Copy, PartialEq, Default)]
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

macro_rules! impl_try_from_number {
	($($ty:ty)*) => {
		$(
			impl TryFrom<Number> for $ty {
				type Error = NotAnInteger;
				fn try_from(num: Number) -> Result<Self, Self::Error> {
					match num.0 {
						Inner::Integer(n) => Ok(n as Self),
						Inner::Float(f) => Err(NotAnInteger(f))
					}
				}
			}
		)*
	};
}

impl_try_from_number!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128);


impl From<FloatType> for Number {
	// note that if the given `f` is an integer, we instead construct an `Inner::Integer`.
	fn from(f: FloatType) -> Number {
		#[allow(clippy::float_cmp)]
		if f.is_normal() && f.floor() == f {
			assert!(f.is_normal() && (f as IntegerType as FloatType) == f, "bad f: {}", f);

			Number(Inner::Integer(f as _))
		} else {
			Number(Inner::Float(f))
		}
	}
}

impl From<IntegerType> for Number {
	#[inline]
	fn from(n: IntegerType) -> Number {
		Number(Inner::Integer(n))
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
	#[inline]
	fn from(n: Number) -> Self {
		match n.0 {
			Inner::Integer(n) => n as _,
			Inner::Float(n) => n,
		}
	}
}

macro_rules! impl_from_integer {
	($($ty:ty)*) => {
		$(
			impl From<$ty> for Number {
				#[inline]
				fn from(num: $ty) -> Self {
					Number::from(num as IntegerType)
				}
			}

			impl From<$ty> for Object {
				#[inline]
				fn from(num: $ty) -> Self {
					Number::from(num).into()
				}
			}
		)*
	};
}

impl_from_integer!{
	i8 i16 i32     i128 isize
	u8 u16 u32 u64 u128 usize
}

macro_rules! impl_math_ops {
	($($trait:ident $trait_assign:ident $fn:ident $fn_assign:ident)*) => {
		$(
			impl std::ops::$trait for Number {
				type Output = Self;

				fn $fn(self, rhs: Self) -> Self {
					use Inner::*;
					match (self.0, rhs.0) {
						(Integer(l), Integer(r)) => Self::from(l.$fn(r)),
						(Integer(l), Float(r)) => Self::from((l as FloatType).$fn(r)),
						(Float(l), Integer(r)) => Self::from(l.$fn(r as FloatType)),
						(Float(l), Float(r)) => Self::from(l.$fn(r))
					}
				}
			}

			impl std::ops::$trait_assign for Number {
				#[inline]
				fn $fn_assign(&mut self, rhs: Self) {
					use std::ops::$trait;
					*self = (*self).$fn(rhs);
				}
			}
		)*
	};
}

impl_math_ops! {
	Add AddAssign add add_assign
	Sub SubAssign sub sub_assign
	Mul MulAssign mul mul_assign
}

impl std::ops::Div for Number {
	type Output = Self;
	/// Divide `self` by `divisor`.
	///
	/// If `divisor` is [zero](Number::ZERO), then `-INF`, `NAN`, or `INF` are returned based on the
	/// sign of `self`.
	fn div(self, divisor: Self) -> Self {
		if divisor == Number::ZERO {
			match self.cmp(&Number::ZERO) {
				Ordering::Less => -Number::INF,
				Ordering::Equal => Number::NAN,
				Ordering::Greater => Number::INF,
			}
		} else {
			// convert to a float because we want to allow for `1/2 = 0.5`
			Self::from(FloatType::from(self) / FloatType::from(divisor))
		}
	}
}

impl std::ops::DivAssign for Number {
	/// Divide `self` by `divisor`, in place.
	///
	/// See (Number::div)[#div] for more details on a divisor of [zero](Number::ZERO).
	#[inline]
	fn div_assign(&mut self, divisor: Self) {
		*self = *self / divisor;
	}
}

impl std::ops::Rem for Number {
	type Output = Self;

	/// Returns `this` modulo `divisor`.
	///
	/// If `divisor` is [zero](Number::ZERO), then `-INF`, `NAN`, or `INF` are returned based on the
	/// sign of `self`.
	fn rem(self, divisor: Self) -> Self {
		if divisor == Number::ZERO {
			match self.cmp(&Number::ZERO) {
				Ordering::Less => -Number::INF,
				Ordering::Equal => Number::NAN,
				Ordering::Greater => Number::INF,
			}
		} else {
			use Inner::*;
			match (self.0, divisor.0) {
				(Integer(l), Integer(r)) => Self::from(l % r),
				(Integer(l), Float(r)) => Self::from(l as FloatType % r),
				(Float(l), Integer(r)) => Self::from(l % r as FloatType),
				(Float(l), Float(r)) => Self::from(l % r)
			}
		}
	}
}

impl std::ops::RemAssign for Number {
	/// Modulo `self` by `divisor`, in place.
	///
	/// If `divisor` is [zero](Number::ZERO), then `-INF`, `NAN`, or `INF` are used based on the sign
	/// of `self`.
	#[inline]
	fn rem_assign(&mut self, divisor: Self) {
		*self = *self % divisor;
	}
}

macro_rules! impl_bitwise_ops {
	($($trait:ident $fn:ident $fn_description:literal $fn_assign:ident $fn_num:ident)*) => {
		$(
			impl std::ops::$trait for Number {
				type Output = Result<Self, NotAnInteger>;

				#[doc="If both numbers are integers, simply "]
				#[doc=$fn_description]
				#[doc=" them together. If either isn't an integer, [`NotAnInteger`] is returned."]
				#[inline]
				fn $fn(self, rhs: Number) -> Self::Output {
					Ok(Self::from(IntegerType::try_from(self)?.$fn(IntegerType::try_from(rhs)?)))
				}
			}
		)*

		/// Bitwise "try" assign operators
		///
		/// Because you can't return values from assignment operators, there's no way to indicate
		/// that one of the numbers isn't an integer. So we have to make our own functions and
		/// deal with it :(
		impl Number {
			$(
				#[doc="If both numbers are integers, perform replace `self` with [`"]
				#[doc=$fn_description]
				#[doc="`]'s result. If either isn't an integer, a [`NotAnInteger`] is returned."]
				#[inline]
				pub fn $fn_assign(&mut self, rhs: Self) -> Result<(), NotAnInteger> {
					use std::ops::$trait;
					*self = (*self).$fn(rhs)?;
					Ok(())
				}
			)*
		}
	};
}

impl_bitwise_ops! {
	BitAnd bitand "bitand" try_bitand_assign bitand_assign
	BitOr bitor "bitor" try_bitor_assign bitor_assign
	BitXor bitxor "bitxor" try_bitxor_assign bitxor_assign
	Shl shl "shl" try_shl_assign shl_assign
	Shr shr "shr" try_shr_assign shr_assign
}

impl std::ops::Neg for Number {
	type Output = Self;
	#[inline]
	fn neg(self) -> Self {
		match self.0 {
			Inner::Integer(i) => Number::from(-i),
			Inner::Float(f) => Number::from(-f)
		}
	}
}

impl std::ops::Not for Number {
	type Output = Result<Self, NotAnInteger>;
	#[inline]
	fn not(self) -> Self::Output {
		Ok(Number::from(!IntegerType::try_from(self)?))
	}

}

impl From<Number> for Text {
	#[inline]
	fn from(n: Number) -> Self {
		Text::new(n.to_string())
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
			Boolean::FALSE
		} else {
			Boolean::TRUE
		}
	}
}

/// Quest methods
impl Number {
	/// Inspects `this`.
	///
	/// This is identical to [`qs_at_text`](#qs_at_text).
	#[inline]
	pub fn qs_inspect(this: &Object, args: Args) -> crate::Result<Object> {
		Self::qs_at_text(this, args)
	}

	/// Convert `this` to a [`Number`].
	///
	/// This simply returns the same object.
	#[inline]
	pub fn qs_at_num(this: &Object, _: Args) -> Result<Object, !> {
		Ok(this.clone())
	}

	/// Converts `this` to a [`Text`], with an optional base parameter.
	///
	/// The base must be `2 <= base <= 36`. 
	pub fn qs_at_text(this: &Object, args: Args) -> crate::Result<Object> {
		use std::convert::TryInto;
		this.try_downcast_and_then(|this: &Self| {
			if let Ok(radix) = args.arg(0) {
				this.to_string_radix(radix.call_downcast_map(Self::clone)?.try_into()?)
					.map_err(|err| crate::Error::from(err.to_string()))
					.map(Object::from)
			} else {
				Ok(Text::from(*this).into())
			}
		})
	}

	/// Converts `this` to a [`Boolean`].
	///
	/// All values but [zero](Number::ZERO) are considered true.
	#[inline]
	pub fn qs_at_bool(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_map(|this: &Self| Boolean::from(*this).into())
	}

	/// Calling a number is simply an alias for [multiplication](#qs_mul).
	#[inline]
	pub fn qs_call(this: &Object, args: Args) -> crate::Result<Object> {
		Self::qs_mul(this, args)
	}

	/// Invert `this`'s sign.
	#[inline]
	pub fn qs_neg(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_map(|this: &Self| (-*this).into())
	}

	/// Get the absolute value of `this`.
	#[inline]
	pub fn qs_pos(this: &Object, args: Args) -> crate::Result<Object> {
		Self::qs_abs(this, args)
	}

	/// Add `this` and the first argument.
	///
	/// # Arguments
	/// 1. (required, `@num`) The addend.
	pub fn qs_add(this: &Object, args: Args) -> crate::Result<Object> {
		let addend = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_map(|this: &Self| (*this + addend).into())
	}

	/// Add `this` and the first argument, in place.
	///
	/// # Arguments
	/// 1. (required, `@num`) The addend.
	pub fn qs_add_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let addend = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_mut_map(|this: &mut Self| *this += addend)
			.map(|_| this.clone())
	}

	/// Subtract the the first argument from `this`.
	///
	/// # Arguments
	/// 1. (required, `@num`) The subtrahend.
	pub fn qs_sub(this: &Object, args: Args) -> crate::Result<Object> {
		let subtrahend = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_map(|this: &Self| (*this - subtrahend).into())
	}

	/// Subtract the the first argument from `this`, in place.
	///
	/// # Arguments
	/// 1. (required, `@num`) The subtrahend.
	pub fn qs_sub_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let subtrahend = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_mut_map(|this: &mut Self| *this -= subtrahend)
			.map(|_| this.clone())
	}

	/// Multiply `this` and the first argument.
	///
	/// # Arguments
	/// 1. (required, `@num`) The multiplicand.
	pub fn qs_mul(this: &Object, args: Args) -> crate::Result<Object> {
		let multiplicand = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_map(|this: &Self| (*this * multiplicand).into())
	}

	/// Multiply `this` and the first argument, in place.
	///
	/// # Arguments
	/// 1. (required, `@num`) The multiplicand.
	pub fn qs_mul_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let multiplicand = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_mut_map(|this: &mut Self| *this *= multiplicand)
			.map(|_| this.clone())
	}

	/// Divide `this` by the first argument.
	///
	/// See (Number::div)[#div] for more details on a divisor of [zero](Number::ZERO).
	///
	/// # Arguments
	/// 1. (required, `@num`) The divisor.
	pub fn qs_div(this: &Object, args: Args) -> crate::Result<Object> {
		let divisor = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_map(|this: &Self| (*this / divisor).into())
	}

	/// Divide `this` by the first argument, in place.
	///
	/// See (Number::div)[#div] for more details on a divisor of [zero](Number::ZERO).
	///
	/// # Arguments
	/// 1. (required, `@num`) The divisor.
	pub fn qs_div_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_mut_map(|this: &mut Self| *this /= rhs)
			.map(|_| this.clone())
	}

	/// Modulo `this` by `divisor`.
	///
	/// See (Number::rem)[#rem] for more details on a divisor of [zero](Number::ZERO).
	///
	/// # Arguments
	/// 1. (required, `@num`) The divisor.
	pub fn qs_mod(this: &Object, args: Args) -> crate::Result<Object> {
		let divisor = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_map(|this: &Self| (*this % divisor).into())
	}

	/// Modulo `this` by `divisor`, in place.
	///
	/// See (Number::rem)[#rem] for more details on a divisor of [zero](Number::ZERO).
	///
	/// # Arguments
	/// 1. (required, `@num`) The divisor.
	pub fn qs_mod_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let divisor = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_mut_map(|this: &mut Self| *this %= divisor)
			.map(|_| this.clone())
	}

	/// Raises `this` to the power of `exponent`.
	///
	/// # Arguments
	/// 1. (required, `@num`) The exponent.
	pub fn qs_pow(this: &Object, args: Args) -> crate::Result<Object> {
		let exponent = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_map(|this: &Self| this.pow(exponent).into())
	}

	/// Raises `this` to the power of `exponent`, in place.
	///
	/// # Arguments
	/// 1. (required, `@num`) The exponent.
	pub fn qs_pow_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let exponent = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_mut_map(|this: &mut Self| this.pow_assign(exponent))
			.map(|_| this.clone())
	}

	/// Bitwise NOT of `this`.
	///
	/// If `this` isn't a whole number, a [`ValueError`] is raised.
	pub fn qs_bitnot(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_and_then(|this: &Self| (!*this).map(Object::from))
	}

	/// Bitwise AND of `this` and `other`.
	///
	/// If either `this` or `other` aren't a whole number, a [`ValueError`] is raised.
	///
	/// # Arguments
	/// 1. (required, `@num`) The other value.
	pub fn qs_bitand(this: &Object, args: Args) -> crate::Result<Object> {
		let other = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_and_then(|this: &Self| (*this & other).map(Object::from))
	}

	/// Bitwise AND of `this` and `other`, in place.
	///
	/// If either `this` or `other` aren't a whole number, a [`ValueError`] is raised.
	///
	/// # Arguments
	/// 1. (required, `@num`) The other value.
	pub fn qs_bitand_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let other = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_mut_and_then(|this: &mut Self| this.try_bitand_assign(other))
			.map(|_| this.clone())
	}

	/// Bitwise OR of `this` and `other`.
	///
	/// If either `this` or `other` aren't a whole number, a [`ValueError`] is raised.
	///
	/// # Arguments
	/// 1. (required, `@num`) The other value.
	pub fn qs_bitor(this: &Object, args: Args) -> crate::Result<Object> {
		let other = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_and_then(|this: &Self| (*this | other).map(Object::from))
	}

	/// Bitwise OR of `this` and `other`, in place.
	///
	/// If either `this` or `other` aren't a whole number, a [`ValueError`] is raised.
	///
	/// # Arguments
	/// 1. (required, `@num`) The other value.
	pub fn qs_bitor_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let other = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_mut_and_then(|this: &mut Self| this.try_bitor_assign(other))
			.map(|_| this.clone())
	}

	/// Bitwise XOR of `this` and `other`.
	///
	/// If either `this` or `other` aren't a whole number, a [`ValueError`] is raised.
	///
	/// # Arguments
	/// 1. (required, `@num`) The other value.
	pub fn qs_bitxor(this: &Object, args: Args) -> crate::Result<Object> {
		let other = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_and_then(|this: &Self| (*this ^ other).map(Object::from))
	}

	/// Bitwise XOR of `this` and `other`, in place.
	///
	/// If either `this` or `other` aren't a whole number, a [`ValueError`] is raised.
	///
	/// # Arguments
	/// 1. (required, `@num`) The other value.
	pub fn qs_bitxor_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let other = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_mut_and_then(|this: &mut Self| this.try_bitxor_assign(other))
			.map(|_| this.clone())
	}

	/// Shift `this` left by `amnt`.
	///
	/// If either `this` or `amnt` aren't a whole number, a [`ValueError`] is raised.
	///
	/// # Arguments
	/// 1. (required, `@num`) The value to shift by.
	pub fn qs_shl(this: &Object, args: Args) -> crate::Result<Object> {
		let amnt = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_and_then(|this: &Self| (*this << amnt).map(Object::from))
	}

	/// Shift `this` left by `amnt`, in place.
	///
	/// If either `this` or `amnt` aren't a whole number, a [`ValueError`] is raised.
	///
	/// # Arguments
	/// 1. (required, `@num`) The value to shift by.
	pub fn qs_shl_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let amnt = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_mut_and_then(|this: &mut Self| this.try_shl_assign(amnt))
			.map(|_| this.clone())
	}

	/// Shift `this` right by `amnt`.
	///
	/// If either `this` or `amnt` aren't a whole number, a [`ValueError`] is raised.
	///
	/// # Arguments
	/// 1. (required, `@num`) The value to shift by.
	pub fn qs_shr(this: &Object, args: Args) -> crate::Result<Object> {
		let amnt = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_and_then(|this: &Self| (*this >> amnt).map(Object::from))
	}

	/// Shift `this` right by `amnt`, in place.
	///
	/// If either `this` or `amnt` aren't a whole number, a [`ValueError`] is raised.
	///
	/// # Arguments
	/// 1. (required, `@num`) The value to shift by.
	pub fn qs_shr_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let amnt = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_mut_and_then(|this: &mut Self| this.try_shr_assign(amnt))
			.map(|_| this.clone())
	}

	/// Get the absolute value of `this`.
	#[inline]
	pub fn qs_abs(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_map(|this: &Self| this.abs().into())
	}

	/// See if a `this` is equal to the first argument.
	///
	/// Unlike most methods, the first argument is not implicitly converted to a [`Number`] first.
	///
	/// # Arguments
	/// 1. (required) The other object to compare against.
	pub fn qs_eql(this: &Object, args: Args) -> crate::Result<Object> {
		let other = args.arg(0)?.downcast_and_then(Number::clone);

		this.try_downcast_map(|this: &Self| other.map(|other| *this == other).unwrap_or(false).into())
	}

	/// Compares `this` to the first argument.
	///
	/// # Arguments
	/// 1. (required, `@num`) The value to compare against.
	pub fn qs_cmp(this: &Object, args: Args) -> crate::Result<Object> {
		let other = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_map(|this: &Self| this.cmp(&other).into())
	}

	/// Returns `this`, rounded down.
	#[inline]
	pub fn qs_floor(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_map(|this: &Self| this.floor().into())
	}

	/// Returns `this`, rounded up.
	#[inline]
	pub fn qs_ceil(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_map(|this: &Self| this.ceil().into())
	}

	/// Returns `this`, rounded towards the nearest integer. (`##.5` rounds away from zero.)
	#[inline]
	pub fn qs_round(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_map(|this: &Self| this.round().into())
	}

	/// Gets the square root of `this`
	#[inline]
	pub fn qs_sqrt(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_map(|this: &Self| Self::from(FloatType::from(*this).sqrt()).into())
	}
}

impl_object_type!{
	for Number 
{
	fn new_object(self) -> Object where Self: Sized {
		use lazy_static::lazy_static;
		use std::collections::HashMap;
		use std::sync::RwLock;

		lazy_static! {
			static ref OBJECTS: RwLock<HashMap<Number, Object>> = RwLock::new(HashMap::new());
		}

		if let Some(obj) = OBJECTS.read().unwrap().get(&self) {
			return obj.deep_clone();
		}

		let mut objs = OBJECTS.write().unwrap();

		objs.entry(self)
			.or_insert_with(|| Object::new_with_parent(self, vec![Number::mapping()]))
			.deep_clone()
	}
}

[(init_parent super::Basic super::Comparable) (parents super::Basic) (convert "@num")]:
	"PI" => const Number::PI,
	"E" => const Number::E,
	"NAN" => const Number::NAN,
	"INF" => const Number::INF,

	"@text" => function Number::qs_at_text,
	"inspect" => function Number::qs_inspect,
	"@num" => function Number::qs_at_num,
	"@bool" => function Number::qs_at_bool,

	"+"  => function Number::qs_add,    "+="  => function Number::qs_add_assign,
	"-"  => function Number::qs_sub,    "-="  => function Number::qs_sub_assign,
	"*"  => function Number::qs_mul,    "*="  => function Number::qs_mul_assign,
	"/"  => function Number::qs_div,    "/="  => function Number::qs_div_assign,
	"%"  => function Number::qs_mod,    "%="  => function Number::qs_mod_assign,
	"**" => function Number::qs_pow,    "**=" => function Number::qs_pow_assign,
	"&"  => function Number::qs_bitand, "&="  => function Number::qs_bitand_assign,
	"|"  => function Number::qs_bitor,  "|="  => function Number::qs_bitor_assign,
	"^"  => function Number::qs_bitxor, "^="  => function Number::qs_bitxor_assign,
	"<<" => function Number::qs_shl,    "<<=" => function Number::qs_shl_assign,
	">>" => function Number::qs_shr,    ">>=" => function Number::qs_shr_assign,

	"-@"  => function Number::qs_neg,
	"+@"  => function Number::qs_pos,
	"~"   => function Number::qs_bitnot,
	"abs" => function Number::qs_abs,
	"<=>" => function Number::qs_cmp,
	"()"  => function Number::qs_call,
	"=="  => function Number::qs_eql,

	"round" => function Number::qs_round,
	"ceil"  => function Number::qs_ceil,
	"floor" => function Number::qs_floor,
	"sqrt"  => function Number::qs_sqrt,
}

#[cfg(test)]
mod tests {
	use super::*;
	mod qs {
		use super::*;

		#[test]
		fn constants() {
			use crate::types::ObjectType;

			macro_rules! assert_exists_eq {
				($key:literal, $val:expr) => {
					assert_eq!(Number::mapping().get_attr_lit($key).unwrap()
						.downcast_and_then(Number::clone).unwrap(), $val);
				}
			}

			<Number as crate::types::ObjectType>::_wait_for_setup_to_finish();

			assert_exists_eq!("PI", Number::PI);
			assert_exists_eq!("E", Number::E);
			assert_exists_eq!("INF", Number::INF);
			assert!(Number::mapping().get_attr_lit("NAN").unwrap()
				.downcast_and_then(Number::clone).unwrap().is_nan());
		}

		#[test]
		fn hash() {
			<Number as crate::types::ObjectType>::_wait_for_setup_to_finish();
		}
	}

	#[test]
	fn constants() {
		assert_eq!(Number::ZERO, Number(Inner::Integer(0 as IntegerType)));
		assert_eq!(Number::ONE, Number(Inner::Integer(1 as IntegerType)));
		assert_eq!(Number::PI, Number(Inner::Float(std::f64::consts::PI)));
		assert_eq!(Number::E, Number(Inner::Float(std::f64::consts::E)));
		assert!(Number(Inner::Float(f64::NAN)).is_nan());
		assert_eq!(Number::INF, Number(Inner::Float(f64::INFINITY)));
	}


	#[test]
	fn default() {
		assert_eq!(Number::default(), Number::ZERO);
	}

	#[test]
	fn to_string() {
		assert_eq!(Number::ONE.to_string(), "1".to_string());
		assert_eq!(Number::ZERO.to_string(), "0".to_string());
		assert_eq!(Number::from(12.3).to_string(), "12.3".to_string());
		assert_eq!(Number::from(-1223.129).to_string(), "-1223.129".to_string());
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


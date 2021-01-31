use crate::value::{Value, SmallInt, SmallFloat};
use std::cmp::Ordering;
use std::borrow::Cow;
use num::ToPrimitive;

/// A heap-allocated number that can represent any numeric value Quest supports.
#[derive(Debug, Clone, PartialEq, PartialOrd, Named)]
#[quest(crate_name="crate", name="Number")]
pub struct BigNum(Internal);

type BigInt = num::BigInt;
type BigFloat = f64;
type Complex = num::Complex<BigFloat>;

#[derive(Debug, Clone)]
enum Internal {
	BigInt(BigInt),
	BigFloat(BigFloat),
	Complex(Complex),
}

#[inline]
fn is_smallint(bigint: &BigInt) -> bool {
	bigint.to_i64().and_then(SmallInt::new).is_some()
}

impl BigNum {
	/// Creates a new `BigNum`, without checking to see if the number is out of the range of a [`SmallInt`].
	///
	/// # Safety
	/// The caller must ensure that the number is either larger than [`SmallInt::MAX`] or smaller than [`SmallInt::MIN`].
	#[inline]
	pub unsafe fn from_bigint_unchecked(bigint: BigInt) -> Self {
		debug_assert!(is_smallint(&bigint), "BigInt '{:?}' shoudl have been made into an SmallInt", bigint);
		Self(Internal::BigInt(bigint))
	}

	/// Creates a new `BigNum`, without checking to see if the float is reducible to a [`SmallFloat`], [`SmallInt`], or
	/// [`BigInt`].
	///
	/// # Safety
	/// The caller must ensure that the number isn't a valid [`SmallFloat`] (ie `number as f32 as f64 != number`)
	#[inline]
	pub unsafe fn from_bigfloat_unchecked(float: BigFloat) -> Self {
		debug_assert_ne!(float as f32 as f64, float, "BigFloat '{:?}' should have been made into a SmallFloat", float);
		debug_assert!(
			!float.is_normal() ||
			float <= SmallInt::MAX.into_inner() as f64 ||
			float <= SmallInt::MAX.into_inner() as f64,
			"BigFloat '{:?}' should have been made into a SmallInt", float);
		// TODO: this check might be buggy.
		debug_assert!(!float.is_normal() || float.fract().abs() > f64::EPSILON,
			"BigFloat '{:?}' should have been made into a BigInt", float);

		Self(Internal::BigFloat(float))
	}

	/// Creates a new `BigNum`, without checking to see if the number is a 
	///
	/// # Safety
	/// The caller must ensure that the number isn't a valid f32 (ie `number as f32 as f64 != number`)
	#[inline]
	pub unsafe fn from_complex_unchecked(complex: Complex) -> Self {
		// As long as there's an imaginary part, we don't care what the real part is. If imaginary is zero, it should be
		// an appropriate type, however.
		debug_assert_ne!(complex.im, 0.0, "Complex '{:?}' should have been made into a non-complex value", complex);

		Self(Internal::Complex(complex))
	}
}

impl From<i64> for Value {
	fn from(val: i64) -> Self {
		if let Some(smallint) = SmallInt::new(val) {
			smallint.into()
		} else {
			// SAFETY: we just verified this in the `if` statement.
			unsafe {
				BigNum::from_bigint_unchecked(val.into()).into()
			}
		}
	}
}

impl From<u64> for Value {
	#[inline]
	fn from(val: u64) -> Self {
		if val <= SmallInt::MAX.into_inner() as u64  {
			(val as i64).into()
		} else {
			// SAFETY: we just verified we were not a valid SmallInt, as no u64 can be smaller than 0.
			unsafe {
				BigNum::from_bigint_unchecked(val.into()).into()
			}
		}
	}
}

impl From<BigInt> for Value {
	#[inline]
	fn from(bigint: BigInt) -> Self {
		if is_smallint(&bigint) {
			// SAFETY: we just verified it is a valid smallint.
			unsafe {
				if let Some(smallint) = bigint.to_i64() {
					SmallInt::new_unchecked(smallint).into()
				} else if cfg!(debug_assertions) {
					unreachable!("bigint.to_i64() failed but is_smallint passed?: {:?}", bigint);
				} else {
					std::hint::unreachable_unchecked()
				}
			}
		} else {
			// SAFETY: we just verified it's not a valid smallint.
			unsafe {
				BigNum::from_bigint_unchecked(bigint).into()
			}
		}
	}
}

// impl From<BigFloat> for BigNum {
// 	fn from(float: BigFloat) -> Self {
// 		// yes, we are using 
// 		if float.is_normal() && (float.round() - float).abs() < f64::EPSILON {
// 			if float <= create::value::SmallInt::MAX.into_inner() as f64 &&
// 			   float >= create::value::SmallInt::MIN.into_inner() as f64 {

// 			}
// 		}  else {
// 			Self(Internal::BigFloat(float))
// 		}
// 		// const FLOAT_EPSILON: f64 = 1e-20;

// 		// if float.frac().abs() <= FLOAT_EPSILON {
// 		// 	Self(Internal::BigFloat())
// 		// }

// 		// Self(Internal::BigFloat(flaot))
// 	}
// }

impl From<Complex> for BigNum {
	#[inline]
	fn from(complex: Complex) -> Self {
		if complex.im == 0.0 {
			// complex.re.into()
			todo!()
		} else {
			Self(Internal::Complex(complex))
		}
	}
}

impl PartialEq for Internal {
	fn eq(&self, rhs: &Self) -> bool {
		match (self, rhs) {
			(Self::BigInt(lhs), Self::BigInt(rhs)) => lhs == rhs,
			(Self::BigFloat(lhs), Self::BigFloat(rhs)) => lhs == rhs,
			(Self::Complex(lhs), Self::Complex(rhs)) => lhs == rhs,
			_ => false
		}
	}
}

impl PartialOrd for Internal {
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> { todo!() }
}

impl crate::ShallowClone for BigNum {
	fn shallow_clone(&self) -> crate::Result<Self> {
		Ok(self.clone())
	}
}

impl crate::DeepClone for BigNum {
	fn deep_clone(&self) -> crate::Result<Self> {
		Ok(self.clone())
	}
}

impl_allocated_type!(for BigNum);
impl_allocated_value_type_ref!(for BigNum);

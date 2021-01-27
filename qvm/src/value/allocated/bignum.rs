use crate::value::NamedType;
use std::cmp::Ordering;
use std::borrow::Cow;

/// A heap-allocated number that can represent any numeric value Quest supports.
#[derive(Debug, Clone, PartialEq, PartialOrd, Named)]
#[quest(crate_name="crate", name="Number")]
pub struct BigNum(Internal);

type BigInt = num::BigInt;
type Float = f64;
type Complex = num::Complex<Float>;

#[derive(Debug, Clone)]
enum Internal {
	BigInt(BigInt),
	Float(Float),
	Complex(Complex),
}

impl BigNum {
	#[inline]
	pub fn new<T: Into<Self>>(data: T) -> Self {
		data.into()
	}
}

// all smaller numbers should go through `smallint`.
// i64 is included here because smallint is `i63`.
impl From<i64> for BigNum {
	#[inline]
	fn from(val: i64) -> Self {
		BigInt::from(val).into()
	}
}

impl From<u64> for BigNum {
	#[inline]
	fn from(val: u64) -> Self {
		BigInt::from(val).into()

	}
}

impl From<i128> for BigNum {
	#[inline]
	fn from(val: i128) -> Self {
		BigInt::from(val).into()
	}
}

impl From<u128> for BigNum {
	#[inline]
	fn from(val: u128) -> Self {
		BigInt::from(val).into()
	}
}

impl From<BigInt> for BigNum {
	#[inline]
	fn from(bigint: BigInt) -> Self {
		Self(Internal::BigInt(bigint))
	}
}

// impl From<Float> for BigNum {
// 	fn from(float: Float) -> Self {
// 		// yes, we are using 
// 		if float.is_normal() && (float.round() - float).abs() < f64::EPSILON {
// 			if float <= create::value::SmallInt::MAX.into_inner() as f64 &&
// 			   float >= create::value::SmallInt::MIN.into_inner() as f64 {

// 			}
// 		}  else {
// 			Self(Internal::Float(float))
// 		}
// 		// const FLOAT_EPSILON: f64 = 1e-20;

// 		// if float.frac().abs() <= FLOAT_EPSILON {
// 		// 	Self(Internal::Float())
// 		// }

// 		// Self(Internal::Float(flaot))
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
			(Self::Float(lhs), Self::Float(rhs)) => lhs == rhs,
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

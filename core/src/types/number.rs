use std::convert::TryFrom;
use std::fmt::{self, Debug, Display, Formatter};
use std::cmp::Ordering;
use crate::{Object, Args};
use crate::types::{Text, Boolean};
use std::hash::{Hash, Hasher};

pub type IntegerType = i64;
pub type FloatType = f64;

#[derive(Clone, Copy)]
pub struct Number(Inner);

#[derive(Clone, Copy)]
enum Inner {
	Integer(IntegerType),
	Float(FloatType),
}

impl Eq for Number {}

impl PartialEq for Number {
	fn eq(&self, rhs: &Number) -> bool {
		use Inner::*;
		match (self.0, rhs.0) {
			(Integer(l), Integer(r)) => l == r,
			(Float(l), Float(r)) => l == r,
			(Integer(n), Float(f))
				| (Float(f), Integer(n)) => f == (n as FloatType),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FromStrError {
	BadInteger(std::num::ParseIntError),
	BadFloat(std::num::ParseFloatError),
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

impl Number {
	pub const ZERO: Self = Number(Inner::Integer(0 as IntegerType));
	pub const  ONE: Self = Number(Inner::Integer(1 as IntegerType));
	pub const   PI: Self = Number(Inner::Float(std::f64::consts::PI));
	pub const    E: Self = Number(Inner::Float(std::f64::consts::E));
	pub const  NAN: Self = Number(Inner::Float(f64::NAN));
	pub const  INF: Self = Number(Inner::Float(f64::INFINITY));


	#[inline]
	pub fn ceil(self) -> IntegerType {
		match self.0 {
			Inner::Integer(i) => i,
			Inner::Float(f) => f.ceil() as _
		}
	}

	#[inline]
	pub fn floor(self) -> IntegerType {
		match self.0 {
			Inner::Integer(i) => i,
			Inner::Float(f) => f.floor() as _
		}
	}

	pub fn from_str_radix(inp: &str, radix: u32) -> Result<Self, FromStrError> {
		if radix < 2 || radix > 36 {
			return Err(FromStrError::BadRadix(radix))
		}

		IntegerType::from_str_radix(inp.trim(), radix)
			.map(Number::from)
			.map_err(FromStrError::BadInteger)
	}
}

impl TryFrom<&'_ str> for Number {
	type Error = FromStrError;
	fn try_from(inp: &str) -> Result<Self, Self::Error> {
		use std::str::FromStr;

		let inp = inp.trim();

		// if we have underscores, delete them and try again.
		if inp.find('_') != None {
			// we don't want to have to convert everything to a string in case a `_` doesn't exist, so
			// we check for `_`'s existance first.
			let mut inp = inp.to_string();

			while let Some(idx) = inp.rfind('_') {
				inp.remove(idx);
			}

			return Number::try_from(inp.as_str())
		}

		IntegerType::from_str(inp)
			.map(Number::from)
			.or_else(|_| FloatType::from_str(inp).map(Number::from))
			.map_err(FromStrError::BadFloat)
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToStringRadixError {
	InvalidRadix(u32),
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
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			ToStringRadixError::InvalidRadix(_) => None,
			ToStringRadixError::NotAnInteger(ref err) => Some(err)
		}
	}
}

impl Number {
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

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct NotAnInteger(f64);

impl Display for NotAnInteger {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{} is not a whole number", self.0)
	}
}

impl std::error::Error for NotAnInteger {}

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

macro_rules! impl_from_integer {
	($($ty:ty)*) => {
		$(
			impl From<$ty> for Number {
				fn from(num: $ty) -> Self {
					Number(Inner::Integer(num as IntegerType))
				}
			}

			impl From<$ty> for Object {
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
				#[inline]
				fn $fn(mut self, rhs: Self) -> Self {
					use std::ops::$trait_assign;
					self.$fn_assign(rhs);
					self
				}
			}

			impl std::ops::$trait_assign for Number {
				fn $fn_assign(&mut self, rhs: Self) {
					use Inner::*;
					use std::ops::$trait;
					match (self.0, rhs.0) {
						(Integer(l), Integer(r)) => self.0 = Integer(l.$fn(r)),
						(Integer(l), Float(r)) => self.0 = Float((l as FloatType).$fn(r)),
						(Float(l), Integer(r)) => self.0 = Float(l.$fn(r as FloatType)),
						(Float(l), Float(r)) => self.0 = Float(l.$fn(r))
					}
				}
			}
		)*
	};
}

impl_math_ops! {
	Add AddAssign add add_assign
	Sub SubAssign sub sub_assign
	Mul MulAssign mul mul_assign
	Rem RemAssign rem rem_assign
}

impl std::ops::Div for Number {
	type Output = Self;
	#[inline]
	fn div(mut self, rhs: Self) -> Self {
		self /= rhs;
		self
	}
}

impl std::ops::DivAssign for Number {
	fn div_assign(&mut self, rhs: Self) {
		use Inner::*;

		match (self.0, rhs.0) {
			(Integer(_), Integer(r)) if r == 0 => *self = Number::NAN,
			(Integer(l), Integer(r)) => *self = Self::from((l as FloatType) / (r as FloatType)),
			(Integer(l), Float(r)) => self.0 = Inner::Float((l as FloatType) / (r)),
			(Float(l), Integer(r)) => self.0 = Inner::Float(l / (r as FloatType)),
			(Float(l), Float(r)) => self.0 = Inner::Float(l / r)
		}
	}
}

macro_rules! impl_bitwise_ops {
	($($trait:ident $fn:ident $fn_assign:ident)*) => {
		$(
			impl std::ops::$trait for Number {
				type Output = Result<Self, NotAnInteger>;
				#[inline]
				fn $fn(mut self, rhs: Number) -> Self::Output {
					self.$fn_assign(rhs)?;
					Ok(self)
				}
			}

			impl Number {
				pub fn $fn_assign(&mut self, rhs: Self) -> Result<(), NotAnInteger> {
					#[allow(unused_imports)]
					use std::ops::*;

					match self.0 {
						Inner::Float(n) => Err(NotAnInteger(n)),
						Inner::Integer(ref mut n) => Ok(n.$fn_assign(IntegerType::try_from(rhs)?))
					}
				}
			}
		)*
	};
}

impl_bitwise_ops! {
	BitAnd bitand bitand_assign
	BitOr bitor bitor_assign
	BitXor bitxor bitxor_assign
	Shl shl shl_assign
	Shr shr shr_assign
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

impl Number {
	#[inline]
	pub fn abs(self) -> Number {
		match self.0 {
			Inner::Integer(i) => Number::from(i.abs()),
			Inner::Float(f) => Number::from(f.abs())
		}
	}

	#[inline]
	pub fn pow(mut self, rhs: Number) -> Number {
		self.pow_assign(rhs);
		self
	}

	pub fn pow_assign(&mut self, rhs: Self) {
		use Inner::*;
		match (self.0, rhs.0) {
			(Integer(l), Integer(r)) if 0 <= r && r <= (u32::MAX as IntegerType)
				=> *self = l.pow(r as u32).into(),
			(Integer(l), Integer(r)) => *self = (l as FloatType).powf(r as FloatType).into(),
			(Integer(l), Float(r)) => *self = (l as FloatType).powf(r).into(),
			(Float(l), Integer(r)) => *self = l.powf(r as FloatType).into(),
			(Float(l), Float(r)) => *self = l.powf(r).into()
		}
	}
}

impl From<Number> for Text {
	#[inline]
	fn from(n: Number) -> Self {
		Text::new(n.to_string())
	}
}

impl From<Number> for Boolean {
	#[inline]
	fn from(n: Number) -> Self {
		if n == Number::ZERO {
			Boolean::FALSE
		} else {
			Boolean::TRUE
		}
	}
}

impl Number {
	#[inline]
	#[allow(non_snake_case)]
	pub fn qs___inspect__(&self, _: Args) -> Result<Text, !> {
		Ok(format!("{:?}", self).into())
	}

	#[inline]
	pub fn qs_at_num(this: &Object, _: Args) -> Result<Object, !> {
		Ok(this.clone())
	}

	pub fn qs_at_text(&self, args: Args) -> crate::Result<Text> {
		if let Ok(radix) = args.arg(0) {
			let radix = radix.downcast_call::<Number>()?.floor();
			self.to_string_radix(radix as _)
				.map_err(|err| err.to_string().into())
				.map(Text::from)
		} else {
			Ok(Text::from(*self))
		}
	}

	#[inline]
	pub fn qs_at_bool(&self, _: Args) -> Result<Boolean, !> {
		Ok(Boolean::from(*self))
	}

	#[inline]
	pub fn qs_call(&self, args: Args) -> crate::Result<Self> {
		self.qs_mul(args)
	}

	#[inline]
	pub fn qs_add(&self, args: Args) -> crate::Result<Self> {
		let rhs = args.arg(0)?.downcast_call::<Self>()?;

		Ok(*self + rhs)
	}

	pub fn qs_add_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.downcast_call::<Self>()?;

		this.try_with_mut(|num: &mut Self| Ok(*num += rhs))?;

		Ok(this.clone())
	}

	#[inline]
	pub fn qs_sub(&self, args: Args) -> crate::Result<Self> {
		let rhs = args.arg(0)?.downcast_call::<Self>()?;

		Ok(*self - rhs)
	}

	pub fn qs_sub_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.downcast_call::<Self>()?;

		this.try_with_mut(|num: &mut Self| Ok(*num -= rhs))?;

		Ok(this.clone())
	}

	#[inline]
	pub fn qs_mul(&self, args: Args) -> crate::Result<Self> {
		let rhs = args.arg(0)?.downcast_call::<Self>()?;

		Ok(*self * rhs)
	}

	pub fn qs_mul_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.downcast_call::<Self>()?;

		this.try_with_mut(|num: &mut Self| Ok(*num *= rhs))?;

		Ok(this.clone())
	}

	#[inline]
	pub fn qs_div(&self, args: Args) -> crate::Result<Self> {
		let rhs = args.arg(0)?.downcast_call::<Self>()?;

		Ok(*self / rhs)
	}

	pub fn qs_div_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.downcast_call::<Self>()?;

		this.try_with_mut(|num: &mut Self| Ok(*num /= rhs))?;

		Ok(this.clone())
	}

	#[inline]
	pub fn qs_mod(&self, args: Args) -> crate::Result<Self> {
		let rhs = args.arg(0)?.downcast_call::<Self>()?;

		Ok(*self % rhs)
	}

	pub fn qs_mod_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.downcast_call::<Self>()?;

		this.try_with_mut(|num: &mut Self| Ok(*num %= rhs))?;

		Ok(this.clone())
	}

	#[inline]
	pub fn qs_pow(&self, args: Args) -> crate::Result<Self> {
		let rhs = args.arg(0)?.downcast_call::<Self>()?;

		Ok(self.pow(rhs))
	}

	pub fn qs_pow_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.downcast_call::<Self>()?;

		this.try_with_mut(|num: &mut Self| Ok(*num = num.pow(rhs)))?;

		Ok(this.clone())
	}

	#[inline]
	pub fn qs_bitand(&self, args: Args) -> crate::Result<Self> {
		let rhs = args.arg(0)?.downcast_call::<Number>()?;

		(*self & rhs).map_err(|err| err.to_string().into())
	}

	pub fn qs_bitand_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.downcast_call::<Number>()?;

		this.try_with_mut(|num: &mut Self| num.bitand_assign(rhs)
			.map_err(|err| err.to_string().into()))?;

		Ok(this.clone())
	}

	#[inline]
	pub fn qs_bitor(&self, args: Args) -> crate::Result<Self> {
		let rhs = args.arg(0)?.downcast_call::<Number>()?;

		(*self | rhs).map_err(|err| err.to_string().into())
	}

	pub fn qs_bitor_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.downcast_call::<Number>()?;

		this.try_with_mut(|num: &mut Self| num.bitor_assign(rhs)
			.map_err(|err| err.to_string().into()))?;

		Ok(this.clone())
	}

	#[inline]
	pub fn qs_bitxor(&self, args: Args) -> crate::Result<Self> {
		let rhs = args.arg(0)?.downcast_call::<Number>()?;

		(*self ^ rhs).map_err(|err| err.to_string().into())
	}

	pub fn qs_bitxor_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.downcast_call::<Number>()?;

		this.try_with_mut(|num: &mut Self| num.bitxor_assign(rhs)
			.map_err(|err| err.to_string().into()))?;

		Ok(this.clone())
	}

	#[inline]
	pub fn qs_shl(&self, args: Args) -> crate::Result<Self> {
		let rhs = args.arg(0)?.downcast_call::<Number>()?;

		(*self << rhs).map_err(|err| err.to_string().into())
	}

	pub fn qs_shl_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.downcast_call::<Number>()?;

		this.try_with_mut(|num: &mut Self| num.shl_assign(rhs)
			.map_err(|err| err.to_string().into()))?;

		Ok(this.clone())
	}

	#[inline]
	pub fn qs_shr(&self, args: Args) -> crate::Result<Self> {
		let rhs = args.arg(0)?.downcast_call::<Number>()?;

		(*self >> rhs).map_err(|err| err.to_string().into())
	}

	pub fn qs_shr_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.downcast_call::<Number>()?;

		this.try_with_mut(|num: &mut Self| num.shr_assign(rhs)
			.map_err(|err| err.to_string().into()))?;

		Ok(this.clone())
	}


	#[inline]
	pub fn qs_neg(&self, _: Args) -> Result<Self, !> {
		Ok(-*self)
	}

	#[inline]
	pub fn qs_pos(&self, args: Args) -> Result<Self, !> {
		self.qs_abs(args)
	}

	#[inline]
	pub fn qs_bitnot(&self, _: Args) -> crate::Result<Self> {
		Ok((!*self).map_err(|x| x.to_string())?)
	}

	#[inline]
	pub fn qs_abs(&self, _: Args) -> Result<Self, !> {
		Ok(self.abs())
	}

	#[inline]
	pub fn qs_eql(&self, args: Args) -> Result<bool, crate::error::KeyError> {
		let rhs = args.arg(0)?.downcast_ref::<Number>();
		Ok(rhs.map(|rhs| *self == *rhs).unwrap_or(false))
	}

	#[inline]
	pub fn qs_cmp(&self, args: Args) -> crate::Result<std::cmp::Ordering> {
		let rhs = args.arg(0)?.downcast_call::<Number>()?;

		Ok(self.cmp(&rhs))
	}

	#[inline]
	pub fn qs_floor(&self, _: Args) -> Result<Self, !> {
		Ok(Number::from(self.floor()))
	}

	#[inline]
	pub fn qs_ceil(&self, _: Args) -> Result<Self, !> {
		Ok(Number::from(self.ceil()))
	}

	#[inline]
	pub fn qs_round(&self, _: Args) -> Result<Self, !> {
		unimplemented!("round");
	}

	#[inline]
	pub fn qs_sqrt(&self, _: Args) -> Result<Self, !> {
		unimplemented!("sqrt")
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

	"@text" => method Number::qs_at_text,
	"__inspect__" => method Number::qs___inspect__,
	"@num" => function Number::qs_at_num,
	"@bool" => method Number::qs_at_bool,

	"+"  => method Number::qs_add,    "+="  => function Number::qs_add_assign,
	"-"  => method Number::qs_sub,    "-="  => function Number::qs_sub_assign,
	"*"  => method Number::qs_mul,    "*="  => function Number::qs_mul_assign,
	"/"  => method Number::qs_div,    "/="  => function Number::qs_div_assign,
	"%"  => method Number::qs_mod,    "%="  => function Number::qs_mod_assign,
	"**" => method Number::qs_pow,    "**=" => function Number::qs_pow_assign,
	"&"  => method Number::qs_bitand, "&="  => function Number::qs_bitand_assign,
	"|"  => method Number::qs_bitor,  "|="  => function Number::qs_bitor_assign,
	"^"  => method Number::qs_bitxor, "^="  => function Number::qs_bitxor_assign,
	"<<" => method Number::qs_shl,    "<<=" => function Number::qs_shl_assign,
	">>" => method Number::qs_shr,    ">>=" => function Number::qs_shr_assign,

	"-@"  => method Number::qs_neg,
	"+@"  => method Number::qs_pos,
	"~"   => method Number::qs_bitnot,
	"abs" => method Number::qs_abs,
	"<=>" => method Number::qs_cmp,
	"()"  => method Number::qs_call,
	"=="  => method Number::qs_eql,

	"round" => method Number::qs_round,
	"ceil"  => method Number::qs_ceil,
	"floor" => method Number::qs_floor,
	"sqrt"  => method Number::qs_sqrt,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn constants() {
		assert_eq!(Number::ZERO, Number(Inner::Integer(0)));
		assert_eq!(Number::ONE, Number(Inner::Integer(1)));
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


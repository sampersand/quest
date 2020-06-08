use std::convert::TryFrom;
use std::fmt::{self, Debug, Display, Formatter};
use std::cmp::Ordering;
use crate::{Object, types};

type IntegerType = i64;
type FloatType = f64;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Number {
	Integer(IntegerType),
	Float(FloatType)
}

impl Eq for Number {}

impl Default for Number {
	fn default() -> Number {
		Number::ZERO
	}
}

impl Debug for Number {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			match self {
				Number::Integer(n) => write!(f, "Integer({:?})", n),
				Number::Float(n) => write!(f, "Float({:?})", n),
			}
		} else {
			Display::fmt(self, f)
		}
	}
}

impl Display for Number {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Number::Integer(n) => Display::fmt(n, f),
			Number::Float(n) => Display::fmt(n, f),
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
	pub const ZERO: Number = Number::Integer(0);
	pub const  ONE: Number = Number::Integer(1);
	pub const   PI: Number = Number::Float(std::f64::consts::PI);
	pub const    E: Number = Number::Float(std::f64::consts::E);
	pub const  NAN: Number = Number::Float(f64::NAN);
	pub const  INF: Number = Number::Float(f64::INFINITY);


	pub fn truncate(self) -> IntegerType {
		match self {
			Number::Integer(i) => i,
			Number::Float(f) => f as _
		}
	}

	#[allow(clippy::float_cmp)]
	fn from_float_should_be_int(f: FloatType) -> Number {
		assert!(f.is_normal() && (f as IntegerType as FloatType) == f, "bad f: {}", f);

		Number::from(f as IntegerType)
	}

	pub fn from_str_radix(inp: &str, radix: u32) -> Result<Number, FromStrError> {
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
			let mut inp = inp.to_string();

			while let Some(idx) = inp.rfind('_') {
				inp.remove(idx);
			}

			return Number::try_from(inp.as_ref())
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

impl Ord for Number {
	fn cmp(&self, rhs: &Number) -> Ordering {
		// this needs to be fixed, but it's not necessary currently.
		self.partial_cmp(rhs).expect("this should be fixed")
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct NotAnInteger(f64);

impl Display for NotAnInteger {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{} is not a whole number", self.0)
	}
}

impl std::error::Error for NotAnInteger {}

impl TryFrom<Number> for IntegerType {
	type Error = NotAnInteger;
	fn try_from(num: Number) -> Result<IntegerType, Self::Error> {
		match num {
			Number::Integer(i) => Ok(i),
			Number::Float(f) => Err(NotAnInteger(f))
		}
	}
}

impl From<FloatType> for Number {
	#[allow(clippy::float_cmp)]
	fn from(f: FloatType) -> Number {
		if f.is_normal() && f.floor() == f {
			Number::from_float_should_be_int(f)
		} else {
			Number::Float(f)
		}
	}
}

impl From<IntegerType> for Number {
	fn from(n: IntegerType) -> Number {
		Number::Integer(n)
	}
}

impl From<FloatType> for Object {
	fn from(f: FloatType) -> Self {
		Number::from(f).into()
	}
}

impl From<IntegerType> for Object {
	fn from(n: IntegerType) -> Self {
		Number::from(n).into()
	}
}

macro_rules! impl_from_integer {
	($($ty:ty)*) => {
		$(
			impl From<$ty> for Number {
				fn from(num: $ty) -> Self {
					Number::Integer(num as IntegerType)
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

impl AsRef<f64> for Number {
	fn as_ref(&self) -> &f64 {
		unimplemented!()
		// &self.0
	}
}

macro_rules! impl_operators {
	(BINARY $($op:ident $fn:ident)*) => {
		$(
			impl std::ops::$op for Number {
				type Output = Self;
				fn $fn(self, rhs: Number) -> Self::Output {
					match (self, rhs) {
						(Number::Integer(lhs), Number::Integer(rhs)) => Number::from(lhs.$fn(rhs)),
						(Number::Integer(lhs),   Number::Float(rhs)) => Number::from((lhs as FloatType).$fn(rhs)),
						(  Number::Float(lhs), Number::Integer(rhs)) => Number::from(lhs.$fn((rhs as FloatType))),
						(  Number::Float(lhs),   Number::Float(rhs)) => Number::from(lhs.$fn(rhs)),
					}
				}
			}
		)*
	};

	(UNARY $($op:ident $fn:ident)*) => {
		$(
			impl std::ops::$op for Number {
				type Output = Self;
				fn $fn(self) -> Self::Output {
					match self {
						Number::Integer(i) => Number::from(i.$fn()),
						Number::Float(f) => Number::from(f.$fn())
					}
				}
			}
		)*
	};

	(BITWISE BINARY $($op:ident $fn:ident)*) => {
		$(
			impl std::ops::$op for Number {
				type Output = Result<Self, NotAnInteger>;
				fn $fn(self, rhs: Number) -> Self::Output {
					Ok(Number::from(IntegerType::try_from(self)?.$fn(IntegerType::try_from(rhs)?)))
				}
			}
		)*
	};

	(BITWISE UNARY $($op:ident $fn:ident)*) => {
		$(
			impl std::ops::$op for Number {
				type Output = Result<Self, NotAnInteger>;
				fn $fn(self) -> Self::Output {
					Ok(Number::from(IntegerType::try_from(self)?.$fn()))
				}
			}
		)*
	};
}

impl_operators!(BINARY Add add Sub sub Mul mul Div div Rem rem);
impl_operators!(UNARY Neg neg);
impl_operators!(BITWISE BINARY BitAnd bitand BitOr bitor BitXor bitxor Shl shl Shr shr);
impl_operators!(BITWISE UNARY Not not);

impl Number {
	pub fn abs(self) -> Number {
		match self {
			Number::Integer(i) => Number::from(i.abs()),
			Number::Float(f) => Number::from(f.abs())
		}
	}

	pub fn pow(self, rhs: Number) -> Number {
		match (self, rhs) {
			(Number::Integer(lhs), Number::Integer(rhs))if 0 <= rhs && rhs <= (u32::MAX as i64)
				=> Number::from(lhs.pow(rhs as u32)),
			(Number::Integer(lhs), Number::Integer(rhs)) => Number::from((lhs as f64).powf(rhs as f64)),
			(Number::Integer(lhs),   Number::Float(rhs)) => Number::from((lhs as f64).powf(rhs)),
			(  Number::Float(lhs), Number::Integer(rhs)) => Number::from(lhs.powf(rhs as f64)),
			(  Number::Float(lhs),   Number::Float(rhs)) => Number::from(lhs.powf(rhs))
		}
	}
}

impl From<Number> for types::Text {
	fn from(n: Number) -> Self {
		types::Text::new(n.to_string())
	}
}

impl From<Number> for types::Boolean {
	fn from(n: Number) -> Self {
		if n == Number::ZERO {
			types::Boolean::FALSE
		} else {
			types::Boolean::TRUE
		}
	}
}

mod impls {
	use super::*;
	use crate::{Object, Result, Args, types::{Text, Boolean}};

	#[derive(Debug, Clone)]
	pub enum NumberError {
		#[allow(unused)]
		BitwiseNotInteger(&'static str, Number, Number, NotAnInteger)
	}

	impl Display for NumberError {
		fn fmt(&self, f: &mut Formatter) -> fmt::Result {
			match self {
				NumberError::BitwiseNotInteger(func, lhs, rhs, err) =>
					write!(f, "bad args for `{}`: ({}, {}): {}", func, lhs, rhs, err)
			}
		}
	}

	impl std::error::Error for NumberError {
		fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
			match self {
				NumberError::BitwiseNotInteger(.., ref err) => Some(err)
			}
		}
	}


	pub fn at_num(args: Args) -> Result<Object> {
		let this = args.this()?;

		this.call_attr("clone", args.clone())
	}
	
	pub fn at_text(args: Args) -> Result<Object> {
		let this = args.this()?.try_downcast_ref::<Number>()?;
		if let Ok(radix) = args.arg(0) {
			let radix = radix.downcast_call::<Number>()?.truncate();
			this.to_string_radix(radix as _)
				.map_err(|err| err.to_string().into())
				.map(Object::from)
		} else {
			Ok(Text::from(*this).into())
		}
	}
	// pub fn at_text(args: Args) -> Result<Object> {
	// 	let this = args.this()?.try_downcast_ref::<Number>()?;
	// 	if let Some(radix) = args.arg(0).ok() {
	// 		let radix = radix.downcast_call::<Number>()?.truncate();
	// 		this.to_string_radix(radix as _)
	// 			.map_err(|err| err.to_string().into())
	// 			.map(Object::from)
	// 	} else {
	// 		Ok(Text::from(*this).into())
	// 	}
	// }

	pub fn at_bool(args: Args) -> Result<Object> {
		let this = args.this()?.try_downcast_ref::<Number>()?;

		Ok(Boolean::from(*this).into())
	}

	pub fn clone(args: Args) -> Result<Object> {
		let this = args.this()?.try_downcast_ref::<Number>()?;

		Ok(this.clone().into())
	}


	pub fn call(args: Args) -> Result<Object> {
		let this = args.this()?;

		this.call_attr("*", args.clone())
	}

	macro_rules! define_operators {
		(BINARY $(($fn:ident $op:ident))*) => {
			$(
				pub fn $fn(args: Args) -> Result<Object> {
					let this = *args.this()?.try_downcast_ref::<Number>()?;
					let rhs = args.arg(0)?.downcast_call::<Number>()?;

					#[allow(unused)]
					use std::ops::*;
					Ok(this.$op(rhs).into())
				}
			)*
		};

		(UNARY $(($fn:ident $op:ident))*) => {
			$(
				pub fn $fn(args: Args) -> Result<Object> {
					let this = *args.this()?.try_downcast_ref::<Number>()?;

					use std::ops::*;
					Ok(this.$op().into())
				}
			)*
		};

		(BITWISE BINARY $(($fn:ident $op:ident))*) => {
			$(
				pub fn $fn(args: Args) -> Result<Object> {
					let this = *args.this()?.try_downcast_ref::<Number>()?;
					let rhs = args.arg(0)?.downcast_call::<Number>()?;

					use std::ops::*;
					this.$op(rhs)
						.map_err(|err| err.to_string().into())
						.map(Object::from)
				}
			)*
		};

		(BITWISE UNARY $(($fn:ident $op:ident))*) => {
			$(
				pub fn $fn(args: Args) -> Result<Object> {
					let this = *args.this()?.try_downcast_ref::<Number>()?;

					use std::ops::*;
					this.$op()
						.map_err(|err| err.to_string().into())
						.map(Object::from)
				}
			)*
		};
	}

	define_operators!(BINARY (add add) (sub sub) (mul mul) (div div) (r#mod rem) (pow pow));
	define_operators!(UNARY  (neg neg));
	define_operators!(BITWISE BINARY (bitand bitand) (bitor bitor) (bitxor bitxor) (shl shl) (shr shr));
	define_operators!(BITWISE UNARY (bitnot not));

	pub fn pos(args: Args) -> Result<Object> {
		let this = args.this()?;

		this.call_attr("abs", args.clone())
	}

	pub fn abs(args: Args) -> Result<Object> {
		let this = *args.this()?.try_downcast_ref::<Number>()?;

		Ok(this.abs().into())
	}


	pub fn eql(args: Args) -> Result<Object> {
		let this = args.this()?.try_downcast_ref::<Number>()?;
		let rhs = args.arg(0)?.downcast_ref::<Number>();

		match rhs {
			Some(rhs) => Ok((*this == *rhs).into()),
			None => Ok(false.into())
		}
	}

	pub fn cmp(args: Args) -> Result<Object> {
		let this = args.this()?.try_downcast_ref::<Number>()?;
		let rhs = args.arg(0)?.downcast_call::<Number>()?;

		match this.cmp(&rhs) {
			std::cmp::Ordering::Greater => Ok(1.into()),
			std::cmp::Ordering::Equal => Ok(0.into()),
			std::cmp::Ordering::Less => Ok((-1).into())
		}
	}

	pub fn floor(args: Args) -> Result<Object> {
		let this = *args.this()?.try_downcast_ref::<Number>()?;

		Ok(this.truncate().into())
	}

	pub fn round(args: Args) -> Result<Object> {
		let this = *args.this()?.try_downcast_ref::<Number>()?;

		match this {
			Number::Integer(i) => Ok(Number::Integer(i).into()),
			Number::Float(f) => Ok(Number::from_float_should_be_int(f.round()).into())
		}
	}

	pub fn ceil(args: Args) -> Result<Object> {
		let this = *args.this()?.try_downcast_ref::<Number>()?;

		match this {
			Number::Integer(i) => Ok(Number::Integer(i).into()),
			Number::Float(f) => Ok(Number::from_float_should_be_int(f.ceil()).into())
		}
	}

	pub fn sqrt(_args: Args) -> Result<Object> {
		unimplemented!()
		// Ok(Number::from(args.this_downcast_ref::<Number>()?.0.sqrt()).into())
	}
}

impl_object_type!{
for Number [(parents super::Basic) (convert "@num")]:
	"PI" => const Number::PI,
	"E" => const Number::E,
	"NAN" => const Number::NAN,
	"INF" => const Number::INF,

	"@num" => impls::at_num,
	"@text" => impls::at_text,
	"@bool" => impls::at_bool,
	"clone" => impls::clone,
	"()" => impls::call,
	"+" => impls::add,
	"-" => impls::sub,
	"*" => impls::mul,
	"/" => impls::div,
	"%" => impls::r#mod,
	"**" => impls::pow,
	"&" => impls::bitand,
	"|" => impls::bitor,
	"^" => impls::bitxor,
	"<<" => impls::shl,
	">>" => impls::shr,
	"-@" => impls::neg,
	"+@" => impls::pos,
	"~" => impls::bitnot,
	"==" => impls::eql,
	"<=>" => impls::cmp,
	"abs" => impls::abs,
	"<" => (|args| args.this()?
			.call_attr("<=>", args.args(..)?)?
			.call_attr("==", vec![(-1).into()])),
	"round" => impls::round,
	"ceil" => impls::ceil,
	"floor" => impls::floor,
	"sqrt" => impls::sqrt,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn constants() {
		assert_eq!(Number::ZERO, Number::Integer(0));
		assert_eq!(Number::ONE, Number::Integer(1));
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
		assert_eq!(Number::from_str_radix("12", 10).unwrap(), Number::Integer(12));
		assert_eq!(Number::from_str_radix("093", 10).unwrap(), Number::Integer(93));
		assert_eq!(Number::from_str_radix("000", 10).unwrap(), Number::Integer(0));
		assert_eq!(Number::from_str_radix("0110110", 2).unwrap(), Number::Integer(0b0110110));
		assert_eq!(Number::from_str_radix("17214", 8).unwrap(), Number::Integer(0o17214));
		assert_eq!(Number::from_str_radix("ff1e24", 16).unwrap(), Number::Integer(0xff1e24));

		// negative numbers
		assert_eq!(Number::from_str_radix("-134", 10).unwrap(), Number::Integer(-134));
		assert_eq!(Number::from_str_radix("-000", 10).unwrap(), Number::Integer(-0));
		assert_eq!(Number::from_str_radix("-10110110", 2).unwrap(), -Number::Integer(0b10110110));
		assert_eq!(Number::from_str_radix("-17214", 8).unwrap(), Number::Integer(-0o17214));
		assert_eq!(Number::from_str_radix("-ff1e24", 16).unwrap(), Number::Integer(-0xff1e24));

		// invalid bases
		assert_eq!(Number::from_str_radix("0", 0).unwrap_err(), FromStrError::BadRadix(0));
		assert_eq!(Number::from_str_radix("0", 1).unwrap_err(), FromStrError::BadRadix(1));
		assert_eq!(Number::from_str_radix("0", 37).unwrap_err(), FromStrError::BadRadix(37));
	}

	#[test]
	fn try_from() {
		// integers
		assert_eq!(Number::try_from("0").unwrap(), Number::Integer(0));
		assert_eq!(Number::try_from("12").unwrap(), Number::Integer(12));
		assert_eq!(Number::try_from("93").unwrap(), Number::Integer(93));
		assert_eq!(Number::try_from("-1952").unwrap(), Number::Integer(-1952));
		assert_eq!(Number::try_from("1e8").unwrap(), Number::Integer(1e8 as _));
		assert_eq!(Number::try_from("1.5e+12").unwrap(), Number::Integer(1.5e12 as _));

		// floats
		assert_eq!(Number::try_from("12.3").unwrap(), Number::Float(12.3));
		assert_eq!(Number::try_from("-12.3").unwrap(), Number::Float(-12.3));
		assert_eq!(Number::try_from("1E-8").unwrap(), Number::Float(1e-8));

		// numbers with extra character we can strip
		assert_eq!(Number::try_from("  123\t\n").unwrap(), Number::Integer(123));
		assert_eq!(Number::try_from("1_000_000").unwrap(), Number::Integer(1_000_000));

		// bad numbers
		assert!(matches!(Number::try_from("invalid").unwrap_err(), FromStrError::BadFloat(..)));
		assert!(matches!(Number::try_from("1.2.3").unwrap_err(), FromStrError::BadFloat(..)));
		assert!(matches!(Number::try_from("12e3e4").unwrap_err(), FromStrError::BadFloat(..)));
		assert!(matches!(Number::try_from("").unwrap_err(), FromStrError::BadFloat(..)));
		assert!(matches!(Number::try_from(" ").unwrap_err(), FromStrError::BadFloat(..)));
	}

}














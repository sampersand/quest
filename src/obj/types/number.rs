mod numbertype;
use self::numbertype::{NumberType, Integer, Rational};

use crate::obj::{self, Object, Mapping, types};
use std::sync::{Arc, RwLock};
use std::convert::TryFrom;
use std::fmt::{self, Debug, Display, Formatter};

impl Eq for Number {}
#[derive(Clone, Copy, PartialEq)]
pub struct Number(f64);

impl Number {
	pub const fn new_integer(n: Integer) -> Self {
		Number(n as _)
		// Number(NumberType::Integer(n))
	}

	pub const fn new_rational(n: Rational) -> Self {
		Number(n as _)
		// Number(NumberType::Rational(n))
	}


	pub const ZERO: Number = Number::new_integer(0);
	pub const  ONE: Number = Number::new_integer(1);
	pub const   PI: Number = Number::new_rational(std::f64::consts::PI);
	pub const    E: Number = Number::new_rational(std::f64::consts::E);
	pub const  NAN: Number = Number::new_rational(f64::NAN);
	pub const  INF: Number = Number::new_rational(f64::INFINITY);

	pub fn try_to_int(&self) -> obj::Result<Integer> {
		Ok(self.0 as _)
		// self.0.try_into()
			// .ok_or_else(|| format!("non-integer number {}", self.0).into())
	}

	pub fn to_int(&self) -> i64 {
		// unimplemented!()
		self.0 as i64
	}

	pub fn from_str_radix(inp: &str, radix: u32) -> Result<Number, std::num::ParseIntError> {
		println!("{:?} {:?}", inp, radix);
		// i64::from_str_radix(inp, radix).map(Number::from)
		unimplemented!()

	}

	pub fn abs(self) -> Number {
		Number::from(self.0.abs())
		// unimplemented!()

	}

	pub fn pow(self, rhs: Number) -> Number {
		self.0.powf(rhs.0).into()
	}

	pub fn from_str(inp: &str) -> Result<Number, std::num::ParseFloatError> {
		use std::str::FromStr;
		// unimplemented!()
		f64::from_str(inp).map(Number::from)
	}

	pub fn to_string_radix(&self, radix: Option<u32>) -> obj::Result<String> {
		match radix {
         Some(2) => Ok(format!("{:b}", self.try_to_int()?).into()),
         Some(8) => Ok(format!("{:o}", self.try_to_int()?).into()),
         Some(16) => Ok(format!("{:x}", self.try_to_int()?).into()),
         Some(10) => Ok(format!("{}", self.try_to_int()?).into()),
         Some(radix @ 0) | Some(radix @ 1) => Err(format!("invalid radix {}", radix).into()),
         Some(other) => todo!("unsupported radix {}", other),
         None => Ok(self.0.to_string().into()),
		}
	}
}

impl Debug for Number {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			write!(f, "Number({:?})", self.0)
		} else {
			Debug::fmt(&self.0, f)
		}
	}
}

impl Display for Number {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl From<Number> for f64 {
	fn from(num: Number) -> Self {
		// unimplemented!()
		num.0
	}
}

macro_rules! impl_from_integer {
	($($ty:ty)*) => {
		$(
			impl From<$ty> for Number {
				fn from(num: $ty) -> Self {
					// unimplemented!()
					Number(num as _)
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
	i8 i16 i32 i64 i128 isize
	u8 u16 u32 u64 u128 usize
	f32 f64
}

impl AsRef<f64> for Number {
	fn as_ref(&self) -> &f64 {
		unimplemented!()
		// &self.0
	}
}

macro_rules! impl_binary_op {
	($($binary:ident $binary_method:ident)* ; $($bitwise:ident $bitwise_method:ident)*) => {
		$(
			impl std::ops::$binary for Number {
				type Output = Self;
				fn $binary_method(self, rhs: Self) -> Self::Output {
		// unimplemented!()
					((self.0).$binary_method(rhs.0)).into()
				}
			}
		)*

		$(
			impl std::ops::$bitwise for Number {
				type Output = obj::Result<Self>;
				fn $bitwise_method(self, rhs: Self) -> Self::Output {
					// unimplemented!()
					Ok(self.try_to_int()?.$bitwise_method(rhs.try_to_int()?).into())
				}
			}
		)*
	};
}

impl_binary_op!(
	Add add Sub sub Mul mul Div div Rem rem;
	Shl shl Shr shr BitAnd bitand BitOr bitor BitXor bitxor
);

impl PartialOrd for Number {
	fn partial_cmp(&self, rhs: &Number) -> Option<std::cmp::Ordering> {
		self.0.partial_cmp(&rhs.0)
	}
}

impl std::ops::Neg for Number {
	type Output = Self;
	fn neg(self) -> Self::Output {
		// unimplemented!()
		Self::from(-f64::from(self))
	}
}


mod impls {
	use super::*;
	use crate::obj::{Object, Result, Args};

	pub fn at_num(args: Args) -> Result<Object> {
		let this = args.this()?;
		debug_assert!(this.is_a::<Number>(), "bad `this` given: {:?}", args.this());
		this.call_attr("clone", args.clone())
	}
	
	pub fn at_text(args: Args) -> Result<Object> {
		let this = args.this()?.try_downcast_ref::<Number>()?;

		if let Some(arg) = args.arg(0).ok() {
			let radix = arg.downcast_call::<Number>()?.try_to_int()?;
			let this = this.try_to_int()?;

			match radix {
            2 => Ok(format!("{:b}", this).into()),
            8 => Ok(format!("{:o}", this).into()),
            16 => Ok(format!("{:x}", this).into()),
            10 => Ok(this.to_string().into()),
            0 | 1 => Err(format!("invalid radix {}", radix).into()),
            _ => todo!("unsupported radix {}", radix)
			}
		} else {
			Ok(this.0.to_string().into())
		}
	}

	pub fn at_bool(args: Args) -> Result<Object> {
		let this = args.this()?.try_downcast_ref::<Number>()?;
		Ok((this.0 != Number::ZERO.0).into())
	}

	pub fn clone(args: Args) -> Result<Object> {
		let this = args.this()?.try_downcast_ref::<Number>()?;
		Ok(this.clone().into())
	}


	pub fn call(args: Args) -> Result<Object> {
		let this = args.this()?;
		this.call_attr("*", args.clone())
	}

	pub fn add(args: Args) -> Result<Object> {
		let this = *args.this()?.try_downcast_ref::<Number>()?;
		let rhs = args.arg(0)?.downcast_call::<Number>()?;
		Ok((this + rhs).into())
	}

	pub fn sub(args: Args) -> Result<Object> {
		let this = *args.this()?.try_downcast_ref::<Number>()?;
		let rhs = args.arg(0)?.downcast_call::<Number>()?;
		Ok((this - rhs).into())
	}

	pub fn mul(args: Args) -> Result<Object> {
		let this = *args.this()?.try_downcast_ref::<Number>()?;
		let rhs = args.arg(0)?.downcast_call::<Number>()?;
		Ok((this * rhs).into())
	}

	pub fn div(args: Args) -> Result<Object> {
		let this = *args.this()?.try_downcast_ref::<Number>()?;
		let rhs = args.arg(0)?.downcast_call::<Number>()?;
		Ok((this / rhs).into())
	}

	pub fn r#mod(args: Args) -> Result<Object> {
		let this = *args.this()?.try_downcast_ref::<Number>()?;
		let rhs = args.arg(0)?.downcast_call::<Number>()?;
		Ok((this % rhs).into())
	}

	pub fn pow(args: Args) -> Result<Object> {
		let this = *args.this()?.try_downcast_ref::<Number>()?;
		let rhs = args.arg(0)?.downcast_call::<Number>()?;
		Ok(this.pow(rhs).into())
	}


	pub fn bitand(args: Args) -> Result<Object> {
		let this = *args.this()?.try_downcast_ref::<Number>()?;
		let rhs = args.arg(0)?.downcast_call::<Number>()?;
		(this & rhs).map(Object::from)
	}

	pub fn bitor(args: Args) -> Result<Object> {
		let this = *args.this()?.try_downcast_ref::<Number>()?;
		let rhs = args.arg(0)?.downcast_call::<Number>()?;
		(this | rhs).map(Object::from)
	}

	pub fn bitxor(args: Args) -> Result<Object> {
		let this = *args.this()?.try_downcast_ref::<Number>()?;
		let rhs = args.arg(0)?.downcast_call::<Number>()?;
		(this ^ rhs).map(Object::from)
	}

	pub fn shl(args: Args) -> Result<Object> {
		let this = *args.this()?.try_downcast_ref::<Number>()?;
		let rhs = args.arg(0)?.downcast_call::<Number>()?;
		(this << rhs).map(Object::from)
	}

	pub fn shr(args: Args) -> Result<Object> {
		let this = *args.this()?.try_downcast_ref::<Number>()?;
		let rhs = args.arg(0)?.downcast_call::<Number>()?;
		(this >> rhs).map(Object::from)
	}


	pub fn neg(args: Args) -> Result<Object> {
		let this = *args.this()?.try_downcast_ref::<Number>()?;
		Ok((-this).into())
	}

	pub fn pos(args: Args) -> Result<Object> {
		let this = args.this()?;
		this.call_attr("abs", args.clone())
	}

	pub fn bitnot(args: Args) -> Result<Object> {
		unimplemented!()
		// let this = *args.this()?.try_downcast_ref::<Number>()?;
		// Ok((!this.0).into())
	}

	pub fn sqrt(args: Args) -> Result<Object> {
		unimplemented!()
		// Ok(Number::from(args.this_downcast_ref::<Number>()?.0.sqrt()).into())
	}

	pub fn abs(args: Args) -> Result<Object> {
		let this = *args.this()?.try_downcast_ref::<Number>()?;
		Ok(this.abs().into())
	}

	pub fn floor(args: Args) -> Result<Object> {
		let this = *args.this()?.try_downcast_ref::<Number>()?;
		Ok(this.to_int().into())
	}

	pub fn eql(args: Args) -> Result<Object> {
		let this = args.this()?.try_downcast_ref::<Number>()?;
		if let Some(rhs) = args.arg(0)?.downcast_ref::<Number>() {
			Ok((*this == *rhs).into())
		} else {
			Ok(false.into())
		}
	}

	pub fn cmp(args: Args) -> Result<Object> {
		let this = args.this()?.try_downcast_ref::<Number>()?;
		let rhs = args.arg(0)?.downcast_call::<Number>()?;

		match this.partial_cmp(&rhs).ok_or_else(Object::default)? {
			std::cmp::Ordering::Greater => Ok(1.into()),
			std::cmp::Ordering::Equal => Ok(0.into()),
			std::cmp::Ordering::Less => Ok((-1).into())
		}
	}

	pub fn idiv(args: Args) -> Result<Object> {
		todo!("idiv");
	}

	pub fn ceil(args: Args) -> Result<Object> {
		todo!("ceil");
	}
	pub fn round(args: Args) -> Result<Object> {
		todo!("round");
	}

	pub fn is_integer(args: Args) -> Result<Object> {
		todo!("is_integer");
	}
}

impl_object_type!{
for Number [(parent super::Basic) (convert "@num")]:
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
	"<" => (|args| args.this()?
			.call_attr("<=>", args.args(..)?)?
			.call_attr("==", vec![(-1).into()])),
	"sqrt" => impls::sqrt,
	"abs" => impls::abs,
	"idiv" => impls::idiv,
	"is_integer" => impls::is_integer,
	"round" => impls::round,
	"ceil" => impls::ceil,
	"floor" => impls::floor
}
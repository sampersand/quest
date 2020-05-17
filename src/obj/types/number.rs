use crate::obj::{self, Object, Mapping, types};
use std::sync::{Arc, RwLock};
use std::convert::TryFrom;
use std::fmt::{self, Debug, Formatter};

type Inner = f64;
type InnerInt = i64;

pub const ZERO: Number = Number::new(0 as _);
pub const  ONE: Number = Number::new(1 as _);


impl Eq for Number {}
#[derive(Clone, Copy, PartialEq)]
pub struct Number(Inner);

impl Number {
	pub const fn new(n: Inner) -> Self {
		Number(n)
	}

	pub fn try_to_int(&self) -> obj::Result<InnerInt> {
		let int = self.to_int();
		if self.0 == int as Inner {
			Ok(int)
		} else {
			Err(format!("non-integer number {}", self.0).into())
		}
	}

	pub fn to_int(&self) -> InnerInt {
		self.0 as InnerInt
	}

	pub fn from_str_radix(inp: &str, radix: u32) -> Result<Number, std::num::ParseIntError> {
		InnerInt::from_str_radix(inp, radix).map(Number::from)
	}

	pub fn abs(self) -> Number {
		Number::from(self.0.abs())
	}

	pub fn pow(self, rhs: Number) -> Number {
		self.0.powf(rhs.0).into()
	}

	pub fn from_str(inp: &str) -> Result<Number, std::num::ParseFloatError> {
		use std::str::FromStr;
		Inner::from_str(inp).map(Number::from)
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

impl From<Number> for Inner {
	fn from(num: Number) -> Self {
		num.0
	}
}

macro_rules! impl_from_integer {
	($($ty:ty)*) => {
		$(
			impl From<$ty> for Number {
				fn from(num: $ty) -> Self {
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
	f32 Inner
}

impl AsRef<Inner> for Number {
	fn as_ref(&self) -> &Inner {
		&self.0
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

macro_rules! impl_binary_op {
	($($binary:ident $binary_method:ident)* ; $($bitwise:ident $bitwise_method:ident)*) => {
		$(
			impl std::ops::$binary for Number {
				type Output = Self;
				fn $binary_method(self, rhs: Self) -> Self::Output {
					((self.0).$binary_method(rhs.0)).into()
				}
			}
		)*

		$(
			impl std::ops::$bitwise for Number {
				type Output = obj::Result<Self>;
				fn $bitwise_method(self, rhs: Self) -> Self::Output {
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

impl std::ops::Neg for Number {
	type Output = Self;
	fn neg(self) -> Self::Output {
		Self::from(-Inner::from(self))
	}
}


mod impls {
	use super::*;
	use crate::obj::{Object, Result, Args};

	pub fn at_num(args: Args) -> Result<Object> {
		let this = args.this()?;
		debug_assert!(this.is_a::<Number>(), "bad `this` given");
		this.call("clone", args.get_rng(1..)?)
	}
	
	pub fn at_text(args: Args) -> Result<Object> {
		if let Some(arg) = args.get(1).ok() {
			let this = args._this_downcast::<Number>()?.try_to_int()?;
			let radix = arg.call("@num", args.new_args_slice(&[]))?.try_downcast_ref::<Number>()?.try_to_int()?;
			match radix {
            2 => Ok(format!("{:b}", this).into()),
            8 => Ok(format!("{:o}", this).into()),
            16 => Ok(format!("{:x}", this).into()),
            10 => Ok(this.to_string().into()),
            0 | 1 => Err(format!("invalid radix {}", radix).into()),
            _ => todo!("unsupported radix {}", radix)
			}
		} else {
			Ok(args._this_downcast::<Number>()?.0.to_string().into())
		}
	}

	pub fn at_bool(args: Args) -> Result<Object> {
		Ok((args._this_downcast::<Number>()?.0 != ZERO.0).into())
	}

	pub fn clone(args: Args) -> Result<Object> {
		Ok(args._this_downcast::<Number>()?.0.into())
	}


	pub fn call(args: Args) -> Result<Object> {
		args._this_obj::<Number>()?.call("*", args.get_rng(1..)?)
	}

	pub fn add(args: Args) -> Result<Object> {
		use std::ops::Add;
		Ok(args._this_downcast::<Number>()?.add(*getarg!(Number; args)).into())	}

	pub fn sub(args: Args) -> Result<Object> {
		use std::ops::Sub;
		Ok(args._this_downcast::<Number>()?.sub(*getarg!(Number; args)).into())
	}

	pub fn mul(args: Args) -> Result<Object> {
		use std::ops::Mul;
		Ok(args._this_downcast::<Number>()?.mul(*getarg!(Number; args)).into())
	}

	pub fn div(args: Args) -> Result<Object> {
		use std::ops::Div;
		Ok(args._this_downcast::<Number>()?.div(*getarg!(Number; args)).into())
	}

	pub fn r#mod(args: Args) -> Result<Object> {
		use std::ops::Rem;
		Ok(args._this_downcast::<Number>()?.rem(*getarg!(Number; args)).into())
	}

	pub fn pow(args: Args) -> Result<Object> {
		Ok(args._this_downcast::<Number>()?.pow(*getarg!(Number; args)).into())
	}


	pub fn bitand(args: Args) -> Result<Object> {
		use std::ops::BitAnd;
		Ok(args._this_downcast::<Number>()?.bitand(*getarg!(Number; args))?.into())
	}

	pub fn bitor(args: Args) -> Result<Object> {
		use std::ops::BitOr;
		Ok(args._this_downcast::<Number>()?.bitor(*getarg!(Number; args))?.into())
	}

	pub fn bitxor(args: Args) -> Result<Object> {
		use std::ops::BitXor;
		Ok(args._this_downcast::<Number>()?.bitxor(*getarg!(Number; args))?.into())
	}

	pub fn shl(args: Args) -> Result<Object> {
		use std::ops::Shl;
		Ok(args._this_downcast::<Number>()?.shl(*getarg!(Number; args))?.into())
	}

	pub fn shr(args: Args) -> Result<Object> {
		use std::ops::Shr;
		Ok(args._this_downcast::<Number>()?.shr(*getarg!(Number; args))?.into())
	}


	pub fn neg(args: Args) -> Result<Object> {
		use std::ops::Neg;
		Ok(args._this_downcast::<Number>()?.neg().into())
	}

	pub fn pos(args: Args) -> Result<Object> {
		args._this_obj::<Number>()?.call("abs", args.new_args_slice(&[]))
	}

	pub fn bitnot(args: Args) -> Result<Object> {
		Ok((!args._this_downcast::<Number>()?.try_to_int()?).into())
	}

	pub fn sqrt(args: Args) -> Result<Object> {
		Ok(Number::from(args.this_downcast::<Number>()?.0.sqrt()).into())
	}

	pub fn abs(args: Args) -> Result<Object> {
		Ok(args._this_downcast::<Number>()?.abs().into())
	}

	pub fn floor(args: Args) -> Result<Object> {
		Ok(args._this_downcast::<Number>()?.to_int().into())
	}

	pub fn eql(args: Args) -> Result<Object> {
		Ok((args._this_downcast::<Number>()?.0 == args.get_downcast::<Number>(1)?.0).into())
	}

	pub fn cmp(args: Args) -> Result<Object> {
		todo!("<=>");
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

impl_object_type!{for Number, super::Basic;
	"@num" => (impls::at_num),
	"@text" => (impls::at_text),
	"@bool" => (impls::at_bool),
	"clone" => (impls::clone),
	"()" => (impls::call),
	"+" => (impls::add),
	"-" => (impls::sub),
	"*" => (impls::mul),
	"/" => (impls::div),
	"%" => (impls::r#mod),
	"**" => (impls::pow),
	"&" => (impls::bitand),
	"|" => (impls::bitor),
	"^" => (impls::bitxor),
	"<<" => (impls::shl),
	">>" => (impls::shr),
	"-@" => (impls::neg),
	"+@" => (impls::pos),
	"~" => (impls::bitnot),
	"==" => (impls::eql),
	"<=>" => (impls::cmp),
	"sqrt" => (impls::sqrt),
	"abs" => (impls::abs),
	"idiv" => (impls::idiv),
	"is_integer" => (impls::is_integer),
	"round" => (impls::round),
	"ceil" => (impls::ceil),
	"floor" => (impls::floor)
}
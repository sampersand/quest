use crate::obj::{self, Object, Mapping, types::ObjectType};
use std::sync::{Arc, RwLock};
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

impl_object_type!{for Number, super::Basic;
	"@num" => (|args| {
		args.this_obj::<Number>()?.call("clone", args.new_args_slice(&[]))
	}),

	"@text" => (|args| {
		if let Some(arg) = args.get(1).ok() {
			let this = args.this::<Number>()?.try_to_int()?;
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
			Ok(args.this::<Number>()?.0.to_string().into())
		}
	}),

	"@bool" => (|args| {
		Ok((args.this::<Number>()?.0 != ZERO.0).into())
	}),

	"clone" => (|args| {
		Ok(args.this::<Number>()?.0.into())
	}),

	"+" => (|args| {
		use std::ops::Add;
		Ok(args.this::<Number>()?.add(*getarg!(Number; args)).into())
	}),

	"-" => (|args| {
		use std::ops::Sub;
		Ok(args.this::<Number>()?.sub(*getarg!(Number; args)).into())
	}),

	"*" => (|args| {
		use std::ops::Mul;
		Ok(args.this::<Number>()?.mul(*getarg!(Number; args)).into())
	}),
	"/" => (|args| {
		use std::ops::Div;
		Ok(args.this::<Number>()?.div(*getarg!(Number; args)).into())
	}),
	"%" => (|args| {
		use std::ops::Rem;
		Ok(args.this::<Number>()?.rem(*getarg!(Number; args)).into())
	}),
	"**" => (|args| {
		Ok(args.this::<Number>()?.pow(*getarg!(Number; args)).into())
	}),


	"&" => (|args| {
		use std::ops::BitAnd;
		Ok(args.this::<Number>()?.bitand(*getarg!(Number; args))?.into())
	}),
	"|" => (|args| {
		use std::ops::BitOr;
		Ok(args.this::<Number>()?.bitor(*getarg!(Number; args))?.into())
	}),
	"^" => (|args| {
		use std::ops::BitXor;
		Ok(args.this::<Number>()?.bitxor(*getarg!(Number; args))?.into())
	}),
	"<<" => (|args| {
		use std::ops::Shl;
		Ok(args.this::<Number>()?.shl(*getarg!(Number; args))?.into())
	}),
	">>" => (|args| {
		use std::ops::Shr;
		Ok(args.this::<Number>()?.shr(*getarg!(Number; args))?.into())
	}),


	"-@" => (|args| {
		use std::ops::Neg;
		Ok(args.this::<Number>()?.neg().into())
	}),
	"+@" => (|args| {
		args.this_obj::<Number>()?.call("abs", args.new_args_slice(&[]))
	}),
	"~" => (|args| {
		Ok((!args.this::<Number>()?.try_to_int()?).into())
	}),
	"abs" => (|args| {
		Ok(args.this::<Number>()?.abs().into())
	}),
	"floor" => (|args| {
		Ok(args.this::<Number>()?.to_int().into())
	}),

	"==" => (|args| {
		Ok((args.this::<Number>()?.0 == args.get_downcast::<Number>(1)?.0).into())
	}),

	"<" => (|args| todo!("<")),
	"<=" => (|args| todo!("<=")),
	">" => (|args| todo!(">")),
	">=" => (|args| todo!(">=")),
	"<=>" => (|args| todo!("<=>")),
	"idiv" => (|args| todo!("idiv")),
	"is_integer" => (|args| todo!("is_integer")),
	"ceil" => (|args| todo!("ceil")),
	"round" => (|args| todo!("round")),
	"is_integer" => (|args| todo!("is_integer")),
}
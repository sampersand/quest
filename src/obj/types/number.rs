use crate::obj::{self, Object, Mapping, types::ObjectType};
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};


type Inner = f64;
type InnerInt = i64;

pub const ZERO: Number = Number(0 as _);
pub const  ONE: Number = Number(1 as _);

impl Eq for Number {}
#[derive(Clone, Copy, PartialEq)]
pub struct Number(Inner);

impl Number {
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

	pub fn from_str(inp: &str) -> Result<Number, std::num::ParseFloatError> {
		use std::str::FromStr;
		Inner::from_str(inp).map(Number::from)
	}
}

impl From<bool> for Number {
	fn from(inp: bool) -> Self {
		if inp {
			ONE
		} else {
			ZERO
		}
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

macro_rules! operator {
	(binary $oper:tt) => {(|args| {
		Ok((args.this::<Number>()?.0 $oper getnum!(args).0).into())
	})};

	(binary int $oper:tt) => {(|args| {
		Ok((args.this::<Number>()?.try_to_int()? $oper getnum!(args).try_to_int()?).into())
	})};

	(binary int $oper:tt) => {(|args| {
		Ok((args.this::<Number>()?.try_to_int()? $oper getnum!(args).try_to_int()?).into())
	})};

	(unary $unary:tt) => {(|args| {
		Ok(($unary args.this::<Number>()?.0).into())
	})};
}

macro_rules! getnum {
	($args:expr) => {
		getnum!($args, 1)
	};

	($args:expr, $pos:expr) => {
		$args.get($pos)?.call("@num", &[])?.try_downcast_ref::<Number>()?
	};
}

impl_object_type!{for Number, super::Basic;
	"@num" => (|args| {
		args.this_obj::<Number>()?.call("clone", &[])
	}),

	"@text" => (|args| {
		if let Some(arg) = args.get(1).ok() {
			let this = args.this::<Number>()?.try_to_int()?;
			let radix = arg.call("@num", &[])?.try_downcast_ref::<Number>()?.try_to_int()?;
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

	"+" => operator!(binary +),
	"-" => operator!(binary -),
	"*" => operator!(binary *),
	"/" => operator!(binary /),
	"%" => operator!(binary %),
	"**" => (|args| {
		Ok((args.this::<Number>()?.0.powf(getnum!(args).0)).into())
	}),
	"&" => operator!(binary int &),
	"|" => operator!(binary int |),
	"^" => operator!(binary int ^),
	"<<" => operator!(binary int <<),
	">>" => operator!(binary int >>),

	"-@" => (|args| {
		Ok((-args.this::<Number>()?.0).into())
	}),
	"+@" => (|args| {
		args.this_obj::<Number>()?.call("abs", &[])
	}),
	"~" => (|args| {
		Ok((!args.this::<Number>()?.try_to_int()?).into())
	}),
	"abs" => (|args| {
		Ok(args.this::<Number>()?.0.abs().into())
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
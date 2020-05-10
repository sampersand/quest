use crate::obj::{Object, Mapping, types::ObjectType};
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};

type NumType = f64;

#[derive(Clone, Copy, PartialEq)]
pub struct Number(NumType);

impl Eq for Number {}


impl Number {
	pub fn into_inner(self) -> NumType {
		self.0
	}
}
impl From<bool> for Number {
	fn from(inp: bool) -> Number {
		Number(if inp { 1.0 } else { 0.0 })
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
	f32 f64
}


impl AsRef<NumType> for Number {
	fn as_ref(&self) -> &NumType {
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

// impl_object_conversions!(
// 	Number "@num" as_num_obj(Number) as_num into_num try_as_num try_into_num call_into_num NumType
// );

macro_rules! operator {
	(binary $oper:tt) => {(|args| {
		Ok(Number::from(args.this::<Number>()?.into_inner() $oper
			args.get(1)?.call("@num", &[])?.try_downcast_ref::<Number>()?.into_inner()).into())
	})};
	(unary $t:tt) => {(|args| unimplemented!())};
}

impl_object_type!{for Number, super::Basic;
	"@num" => (|args| {
		args.this_obj::<Number>()?.call("clone", &[])
	}),

	"@text" => (|args| {
		Ok(Text::from(args.this::<Number>()?.as_ref().to_string()).into())
	}),

	"@bool" => (|args| {
		Ok(Boolean::from(args.this::<Number>()?.into_inner() != 0.0).into())
	}),

	"clone" => (|args| {
		Ok(args.this::<Number>()?.clone().into())
	}),

	"+" => operator!(binary +),
	"-" => operator!(binary -),
	"*" => operator!(binary *),
	"/" => operator!(binary /),
}


// impl ObjectType for Number {
// 	fn mapping() -> Arc<RwLock<Mapping>> {
// 		// use std::sync::Once;
// 		// static MAPPING: Mapping = {
// 		use crate::obj::Object;
// 		let mut m = Mapping::new(None);
// 		m.insert(
// 			super::Text::new("+").into(),
// 			super::RustFn::new("+",
// 				(|x, y| Ok(x.clone()))
// 			).into()
// 		);
// 		Arc::new(RwLock::new(m))
// 		// m.insert()
// 		// };

// 		// MAPPING
// 	}
// }



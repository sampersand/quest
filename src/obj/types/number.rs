use crate::obj::{DataEnum, Object, Mapping, types::ObjectType};
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};

type NumType = f64;

#[derive(Clone, Copy, PartialEq)]
pub struct Number(NumType);

impl Eq for Number {}


impl Number {
	pub fn new<T: Into<NumType>>(num: T) -> Self {
		Number(num.into())
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


impl From<Number> for DataEnum {
	fn from(this: Number) -> DataEnum {
		DataEnum::Number(this)
	}
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

impl_object_conversions!(
	Number "@num" as_num_obj(Number) as_num into_num try_as_num try_into_num call_into_num NumType
);

macro_rules! assert_this_is_number {
	($args:expr) => {{
		let this = $args.get(0).unwrap();
		assert!(this.as_num().is_some(), "bad `this` given: {:#?}", this);
	}};
}

macro_rules! operator {
	(binary $oper:tt) => {(|args| {
		assert_this_is_number!(args);
		Ok(Number::from(args.get(0)?.try_into_num()? $oper args.get(1)?.call_into_num()?).into())
	})};
	(unary $t:tt) => {(|args| unimplemented!())};
}
impl_object_type!{for Number, super::Basic;
	"@num" => (|args| {
		assert_this_is_number!(args);
		args.get(0)?.call("clone", &[])
	}),

	"@text" => (|args| {
		assert_this_is_number!(args);
		Ok(args.get(0)?.try_as_num()?.to_string().into())
	}),

	"@bool" => (|args| {
		assert_this_is_number!(args);
		Ok((args.get(0)?.try_into_num()? != 0.0).into())
	}),

	"clone" => (|args| {
		assert_this_is_number!(args);
		Ok(Number::from(args.get(0)?.try_into_num()?).into())
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



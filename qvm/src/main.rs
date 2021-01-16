#![allow(unused)]
use qvm::{*, value::*};

fn main() {
	// let func = value::BuiltinFn::new(Literal::new("abc"), |_, _| panic!());
	// let func2 = value::BuiltinFn::new(Literal::new("abc1"), |_, _| panic!());
	// println!("{:?}", std::mem::size_of_val(&func));

	#[derive(Debug)]
	struct Custom(u32);

	impl try_traits::clone::TryClone for Custom {
		type Error = Error;
		fn try_clone(&self) -> Result<Self> { Ok(Self(self.0)) }
	}

	impl QuestObject for Custom {}

	println!("{:?}", Value::new(Custom(34)));
	// return;
	// println!("{:?}", Value::new(Boolean::new(true)));
	// println!("{:?}", Value::new(Boolean::new(false)));
	// println!("{:?}", Value::new_smallint(123));
	// println!("{:?}", Value::new(value::Null));
	// println!("{:?}", Value::new_custom("foo"));
}

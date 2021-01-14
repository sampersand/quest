#![allow(unused)]
use qvm::{*, value::*};

fn main() {
	let func = value::BuiltinFn::new(Literal::new("abc"), |_, _| panic!());
	let func2 = value::BuiltinFn::new(Literal::new("abc1"), |_, _| panic!());
	println!("{:?}", std::mem::size_of_val(&func));
	// return;
	// println!("{:?}", Value::new(Boolean::new(true)));
	// println!("{:?}", Value::new(Boolean::new(false)));
	// println!("{:?}", Value::new_smallint(123));
	// println!("{:?}", Value::new(value::Null));
	// println!("{:?}", Value::new_custom("foo"));
}

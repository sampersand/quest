use qvm::{*};

fn main() {
	println!("{:?}", Value::new(true));
	println!("{:?}", Value::new(false));
	println!("{:?}", Value::new_smallint(123));
	println!("{:?}", Value::new(value::Null));
	println!("{:?}", Value::new_custom("foo"));
}

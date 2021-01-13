use qvm::{*, value::Boolean};

fn main() {
	println!("{:?}", Value::new(Boolean::new(true)));
	println!("{:?}", Value::new(Boolean::new(false)));
	println!("{:?}", Value::new_smallint(123));
	println!("{:?}", Value::new(value::Null));
	println!("{:?}", Value::new_custom("foo"));
}

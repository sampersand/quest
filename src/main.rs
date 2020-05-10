#![allow(unused)]

mod obj;

use obj::Object;

fn main() {
	let twelve = Object::from(12);
	let twenty = Object::from(20);

	// let num_class = twelve.call("__get_attr__", &[&"__parent__".into()]).unwrap();
	// let basic_class = num_class.call("__get_attr__", &[&"__parent__".into()]).unwrap();
	let text = Object::from("text");
	println!("text: {:?}", text);
	// (*text.downcast_mut::<obj::types::Text>().unwrap()).repl("A");
	println!("text: {:?}", text);
	// println!("==: {:?}", twelve.downcast::<obj::types::Text>().map(|x| x.clone()));
	println!("{:?}", twelve.call("+", &[&twenty]).unwrap());
	// println!("{:?}", basic_class.call("__get_attr__", &[&"name".into()]).unwrap());

	// twelve.call("__set_attr__", &[&"__parent__".into(), &basic_class]).unwrap();

	// println!("{:?}", twelve.call("+", &[&twenty]));
	// println!("{:?}", twenty.call("+", &[&twelve]).unwrap());

	// twelve.call("__set_attr__", &[
	// 	&"+".into(),
	// 	&twenty.call("__get_attr__", &[&"+".into()]).unwrap()
	// ]).unwrap();

	// println!("{:?}", twelve.call("+", &[&twenty]).unwrap());
}

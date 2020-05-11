#![allow(unused)]

mod obj;

use obj::Object;

fn main() {
	let ref twelve = Object::from(12);
	let ref twelve5 = Object::from(12.4);
	let ref twenty = Object::from(20);


	println!("{:?}", twelve5.call("floor", &[]));
	// let ref rustfn = twelve.get_attr(&"+".into());
	// println!("{:?}", rustfn);
	// println!("{:?}", rustfn.unwrap().call("@text", &[]));
	// let ref tru = Object::from(true);
	// let ref fals = Object::from(false);
	// println!("{:?}", Object12.3.call("==", &[twelve]));
	// println!("{:?}", tru.call("==", &[tru]));
	// println!("{:?}", fals.call("==", &[fals]));
	// println!("{:?}", tru.call("==", &[fals]));
	// println!("{:?}", fals.call("==", &[tru]));

/*	// let num_class = twelve.call("__get_attr__", &[&"__parent__".into()]).unwrap();
	// let basic_class = num_class.call("__get_attr__", &[&"__parent__".into()]).unwrap();
	let text = Object::from("text");
	println!("text: {:?}", text);
	// (*text.downcast_mut::<obj::types::Text>().unwrap()).repl("A");
	println!("text: {:?}", text);
	// println!("==: {:?}", twelve.downcast::<obj::types::Text>().map(|x| x.clone()));
	println!("{:?}", twelve.call("+", &[&twenty]).unwrap());
	println!("{:?}", twelve.get_attr(&"__parent__".into()).unwrap());
	// println!("{:?}", basic_class.call("__get_attr__", &[&"name".into()]).unwrap());

*/	// twelve.call("__set_attr__", &[&"__parent__".into(), &basic_class]).unwrap();

	// println!("{:?}", twelve.call("+", &[&twenty]));
	// println!("{:?}", twenty.call("+", &[&twelve]).unwrap());

	// twelve.call("__set_attr__", &[
	// 	&"+".into(),
	// 	&twenty.call("__get_attr__", &[&"+".into()]).unwrap()
	// ]).unwrap();

	// println!("{:?}", twelve.call("+", &[&twenty]).unwrap());
}

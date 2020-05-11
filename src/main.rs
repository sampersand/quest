#![allow(unused)]

mod obj;

use obj::Object;

fn foo<'obj>(l1: &'obj Object) {
	let a1 = Object::from("a1");
	bar(&a1, &[l1]);
}
fn bar<'slice, 'obj>(a1: &Object, l2: &[&obj::Object]) {
	println!("{:?}", a1.call_attr(&"@num".into(), l2).unwrap()
	);
}

fn main() {
	let ref twelve = Object::from(12);
	let ref twelve4 = Object::from(12.4);
	let ref twenty = Object::from(16);

	let a1 = Object::from("a1");
	let l1 = Object::from(16);
	foo(&l1)
	// println!("{:?}",
	// 	a1.call_attr_bound::<&[&Object]>(
	// 		&"@num".into(),
	// 		&[l1]
	// 	).unwrap()
	// );
	// let ref add = twelve.call("__get_attr__", &[&"+".into()]).unwrap();
	// let ref call = add.call("__get_attr__", &[&"()".into()]).unwrap();

	// println!("{:?}", call.call("()", &[add, twelve, twenty]));	
	// 		.call("@text", &[&16.into()])
	// );
	// let ref rustfn = twelve.get_attr(&"+".into()).unwrap();
	// println!("{:?}", rustfn);
	// println!("{:?}", rustfn.call("@text", &[]));
	// let ref tru = Object::from(true);
	// let ref fals = Object::from(false);
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

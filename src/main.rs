#![allow(unused)]

mod obj;


fn main() {
	use obj::Object;
	let num: Object = 1.into();
	println!("{:#?}", num);
	println!("{:#?}", num.call("+", &[&2.into()]));
}

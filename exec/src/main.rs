use quest_parser::expression::Executable;
use quest_parser::{stream::BufStream, Stream, Expression};

fn main() {
	let filename = std::env::args().nth(1).unwrap_or_else(|| "code/test.qs".to_string());
	let stream = BufStream::new_from_path(filename).unwrap();
	
	match Expression::parse_stream(stream.tokens()) {
		Ok(expr) => 
			match expr.execute() {
				Ok(val) => println!("{:?}", val),
				Err(err) => println!("{:?}", err)
			},
		Err(err) => println!("{}", err)
	}
	// let filename = env::args().nth(1).unwrap_or_else(|| "code/test.qs".to_string());
	// let mut stream = BufStream::try_from(<_ as AsRef<std::path::Path>>::as_ref(&filename))
	// 	.expect("couldn't open file")
	// 	.collect::<TokenizeResult<Vec<_>>>()
	// 	.unwrap()
	// 	.into_iter();

	// let expression = Expression::try_from_iter(&mut stream).unwrap();
	// let mut args: Vec<Object> = std::env::args()
	// 	.skip(1)
	// 	.map(Object::from)
	// 	.collect::<Vec<Object>>();
	// args.insert(0, Object::default());
	// let result = Binding::new_stackframe(args.into(), |_| expression.execute());
	// if cfg!(debug) {
	// 	println!("{:?}", result);
	// } else {
	// 	result.unwrap();
	// }
}

// #![deny(warnings)]

// use quest::{Object, Binding};
// use quest_parser::{Result as ParseResult, BufStream, Expression};
// use std::convert::TryFrom;

// // TODO: repl
// mod repl;

// use std::env;

// fn main() {
// 	let filename = env::args().nth(1).unwrap_or_else(|| "code/test.qs".to_string());
// 	let mut stream = BufStream::try_from(<_ as AsRef<std::path::Path>>::as_ref(&filename))
// 		.expect("couldn't open file")
// 		.collect::<ParseResult<Vec<_>>>()
// 		.unwrap()
// 		.into_iter();

// 	let expression = Expression::try_from_iter(&mut stream).unwrap();
// 	let mut args: Vec<Object> = std::env::args()
// 		.skip(1)
// 		.map(Object::from)
// 		.collect::<Vec<Object>>();
// 	args.insert(0, Object::default());
// 	let result = Binding::new_stackframe(args.into(), |_| expression.execute());
// 	if cfg!(debug) {
// 		println!("{:?}", result);
// 	} else {
// 		result.unwrap();
// 	}
// }

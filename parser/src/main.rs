use quest_parser::{stream::BufStream, Stream, Expression};

fn main() {
	let stream = BufStream::new_from_path("../code/test.qs").unwrap();
	
	match Expression::parse_stream(stream.tokens()) {
		Ok(expr) => println!("{}", expr),
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
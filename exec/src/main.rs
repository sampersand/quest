#![allow(unused)]

mod repl;
mod opts;
mod buf_stream;
mod error;

use error::{Error, Result};
use opts::Opts;
use repl::Repl;
use buf_stream::BufStream;

use quest::{Binding, Object};

// use quest_parser::expression::Executable;
// use quest_parser::{Stream, Expression};

fn main() {
	use clap::Clap;

	let mut opts = Opts::parse();
	let exec = Object::new(quest::types::Scope);
	exec.set_attr("name", Object::from("exec".to_string())).unwrap();

	let mut args: quest::Args = opts.args.iter()
		.map(|x| x.to_string().into())
		.collect::<Vec<_>>().into();
		
	args.add_this(exec);

	Binding::new_stackframe(args, move |binding| {
		binding.set_attr("name", Object::from("main"))?;
		opts.run()
	}).expect("error");

	// quest::Binding::new_stackframe(args, move |_| {
	// 	let filename = std::env::args().nth(1);

	// 	let stream = buf_stream::BufStream::new_from_path(filename.unwrap()).unwrap();

	// 	Expression::parse_stream(stream.tokens())
	// 		.map_err(|err| Object::from(err.to_string()))?
	// 		.execute()
	// }).expect("couldn't execute");

		// .unwrap_or_else(|| "code/test.qs".to_string());

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

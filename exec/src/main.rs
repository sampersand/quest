#![allow(unused)]

mod run;
mod error;

use error::{Error, Result};
use quest::Object;
use clap::Clap;

/// Run the Quest programming language
#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "Sam Westerman <sam@sampersand.me>")]
struct Opts {
	/// Define the file to run. If `-` is supplied, STDIN is read.
	#[clap(short="f", long, conflicts_with="eval")]
	file: Option<std::path::PathBuf>,

	/// Evaluate a passed command as quest code. Omit `file`.
	#[clap(short, long, conflicts_with="file")]
	eval: Option<String>,

	#[clap(last=true)]
	args: Vec<String>

	/*
	#[clap(short, long, env="QUEST_DEBUG")]
	debug: bool
	*/
}


fn run_options(Opts { file, eval, args, .. }: Opts) -> Result<Object> {
	let args = args.into_iter().map(Object::from).collect::<quest::Args>();
	match (file, eval) {
		(Some(_), Some(_)) => panic!("both options set?"),
		(Some(file), None) if file.to_str() == Some("-") => run::run_stdin(args),
		(Some(file), None) => run::run_file(file, args),
		(None, Some(expr)) => run::run_expression(expr, args),
		(None, None)       => run::run_repl(args)
	}
}

fn main() {
	match run_options(Opts::parse()) {
		Ok(_) => {},
		Err(err) => println!("uncaught error encountered:\n{}", err)
	}
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

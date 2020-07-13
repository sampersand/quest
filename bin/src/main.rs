#![deny(warnings)]

mod run;
mod error;

use error::Result;
use run::BufStream;
use quest_core::Object;
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
	let mut args: Vec<Object> = args.into_iter().map(Object::from).collect();

	if let Some(file) = file.as_ref() {
		args.insert(0, file.display().to_string().into());
	}

	let args = args.iter().collect();

	match (file, eval) {
		(Some(_), Some(_)) => panic!("both options set?"),
		(Some(file), None) if file.to_str() == Some("-") => run::stdin(args),
		(Some(file), None) => run::file(file, args),
		(None, Some(expr)) => run::expression(expr, args),
		(None, None)       => run::repl(args)
	}
}

pub fn init() {
	use quest_core::types::{ObjectType, RustFn, Text, rustfn::Binding};
	use quest_parser::{Stream, expression::Executable};

	Text::mapping().set_attr_lit("eval", RustFn::new("Text::eval", |this, args| {
		fn execute_text(text: String) -> quest_core::Result<Object> {
			quest_parser::Expression::parse_stream(BufStream::from(text).tokens())
				.map_err(|err| err.to_string())?
				.execute()
				.map_err(Into::into)
		}

		this.try_downcast_and_then(|this: &Text| {
			if let Ok(binding) = args.arg(0) {
				Binding::new_stackframe(Some(binding.clone()), args, |_| execute_text(this.to_string()))
			} else {
				execute_text(this.to_string())
			}
		})
	}));
}

fn main() {
	// run::run_file("code.ignore/fib.qs", vec![&"--".into(), &"10_000".into()].into()).unwrap();
	// run::run_file("code.ignore/ib.qs", vec![&"--".into(), &"guess.kn".into()].into()).unwrap();

	// if true { return; }
	quest_core::init();
	quest_parser::init();
	init();

	match run_options(Opts::parse()) {
		Ok(_) => {},
		// Ok(x) => println!("{:?}", x),
		Err(err) => eprintln!("uncaught error encountered:\n{}", err)
	}
}

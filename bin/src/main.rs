mod run;
mod error;

use error::Result;
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
		(Some(file), None) if file.to_str() == Some("-") => run::run_stdin(args),
		(Some(file), None) => run::run_file(file, args),
		(None, Some(expr)) => run::run_expression(expr, args),
		(None, None)       => run::run_repl(args)
	}
}

fn main() {
	quest_core::initialize();
	quest_parser::initialize();

	// if true {run::run_file("code.ignore/fib.qs", Default::default()).unwrap(); return}
	// run::run_file("code.ignore/fib.qs", vec![&"--".into(), &"50_000".into()].into()).unwrap();
	// run::run_file("code.ignore/ib.qs", vec![&"--".into(), &"guess.kn".into()].into()).unwrap();

	match run_options(Opts::parse()) {
		Ok(_) => {},
		// Ok(x) => println!("{:?}", x),
		Err(err) => eprintln!("uncaught error encountered:\n{}", err)
	}
}

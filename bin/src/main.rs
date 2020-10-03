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

fn setup_tracing() {
	use tracing_subscriber::{layer::SubscriberExt, registry::Registry};
	use tracing_tree::HierarchicalLayer;
	let layer = HierarchicalLayer::default()
		.with_indent_lines(true)
		.with_indent_amount(2)
		.with_thread_names(true)
		.with_thread_ids(true)
		.with_verbose_exit(true)
		.with_verbose_entry(true)
		.with_targets(true);

	let subscriber = Registry::default().with(layer);
	tracing::subscriber::set_global_default(subscriber).unwrap();

	// tracing_subscriber::fmt()
	// 	.with_max_level(tracing::level_filters::LevelFilter::TRACE)
	// 	.with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
	// 	.init();
}

fn main() {
	setup_tracing();

	quest_core::initialize();
	quest_parser::initialize();

	// The following line is used by me internally for benchmarking.
	// if true {run::run_file("code.ignore/fib.qs", Default::default()).unwrap(); return}

	match run_options(Opts::parse()) {
		Ok(_) => {},
		// Ok(x) => println!("{:?}", x),
		Err(err) => eprintln!("uncaught error encountered:\n{}", err)
	}
}

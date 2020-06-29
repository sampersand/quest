mod repl;
mod buf_stream;

pub use buf_stream::BufStream;
use repl::Repl;
use quest::{Object, Binding, Args};
use crate::{Error, Result};
use std::path::Path;
use std::convert::TryFrom;

pub trait Runner {
	fn run(self) -> Result<Object>;
}

pub fn run_file<P: AsRef<Path>>(path: P, args: Args) -> Result<Object> { 
	run(BufStream::try_from(path.as_ref())?, args).map_err(From::from)
}

pub fn run_expression(expr: String, args: Args) -> Result<Object> {
	run(BufStream::from(expr), args).map_err(From::from)
}

pub fn run_stdin(args: Args) -> Result<Object> {
	run(BufStream::stdin(), args).map_err(From::from)
}

pub fn run_repl(args: Args) -> Result<Object> {
	run(Repl::new(), args).map_err(From::from)
}

pub fn run<R: Runner>(runner: R, args: Args) -> quest::Result<Object> {
	let main = Object::new(quest::types::Scope);
	main.set_attr("name", Object::from("main"))?;

	Binding::new_stackframe(Some(main), args, move |_| {
		runner.run().map_err(|err| quest::Error::Boxed(Box::new(err)))
	})
}









mod repl;
mod buf_stream;

pub use buf_stream::BufStream;
use repl::Repl;
use quest::{Object, Binding, ArgsOld};
use crate::{Error, Result};
use std::path::Path;
use std::convert::TryFrom;

pub trait Runner {
	fn run(self) -> Result<Object>;
}

pub fn run_file<'a, P: AsRef<Path>, A: Into<ArgsOld<'a>>>(path: P, args: A) -> Result<Object> {
	run(BufStream::try_from(path.as_ref())?, args).map_err(From::from)
}

pub fn run_expression<'a, A: Into<ArgsOld<'a>>>(expr: String, args: A) -> Result<Object> {
	run(BufStream::from(expr), args).map_err(From::from)
}

pub fn run_stdin<'a, A: Into<ArgsOld<'a>>>(args: A) -> Result<Object> {
	run(BufStream::stdin(), args).map_err(From::from)
}

pub fn run_repl<'a, A: Into<ArgsOld<'a>>>(args: A) -> Result<Object> {
	run(Repl::new(), args).map_err(From::from)
}

pub fn run<'a, R: Runner, A: Into<ArgsOld<'a>>>(runner: R, args: A) -> quest::Result<Object> {
	use quest_parser::expression::Executable;

	let mut args = args.into();
	args.add_this({
		let main = Object::new(quest::types::Scope);
		main.set_attr("name", Object::from("outermost-scope"))?;
		main
	});

	Binding::new_stackframe(args, move |binding| {
		binding.set_attr("name", Object::from("main"))?;
		runner.run().map_err(|err| quest::Error::Boxed(Box::new(err)))
	})
}





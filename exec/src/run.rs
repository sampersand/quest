mod repl;
mod buf_stream;

pub use buf_stream::BufStream;
use repl::Repl;
use quest::{Object, Binding, Args};
use crate::Result;
use std::path::Path;
use std::convert::TryFrom;

pub trait Runner {
	fn run(self) -> Result<Object>;
}

pub fn run_file<'a, P: AsRef<Path>, A: Into<Args<'a>>>(path: P, args: A) -> Result<Object> {
	run(BufStream::try_from(path.as_ref())?, args)
}

pub fn run_expression<'a, A: Into<Args<'a>>>(expr: String, args: A) -> Result<Object> {
	run(BufStream::from(expr), args)
}

pub fn run_stdin<'a, A: Into<Args<'a>>>(args: A) -> Result<Object> {
	run(BufStream::stdin(), args)
}

pub fn run_repl<'a, A: Into<Args<'a>>>(args: A) -> Result<Object> {
	run(Repl::new(), args)
}

pub fn run<'a, R: Runner, A: Into<Args<'a>>>(runner: R, args: A) -> Result<Object> {
	use quest_parser::expression::Executable;

	let mut args = args.into();
	args.add_this({
		let main = Object::new(quest::types::Scope);
		main.set_attr("name", Object::from("outermost-scope"))?;
		main
	});

	Binding::new_stackframe(args, move |binding| {
		binding.set_attr("name", Object::from("main"))?;
		runner.run()
	})
}


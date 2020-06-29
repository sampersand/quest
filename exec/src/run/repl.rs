use quest_parser::{Stream, Context, Contexted};
use crate::run::Runner;
use std::io::{self, Seek, SeekFrom};

#[derive(Debug, Clone)]
pub struct Repl {
	context: Context
}

impl Runner for Repl {
	fn run(self) -> crate::Result<quest_core::Object> {
		unimplemented!()
	}
}

impl Repl {
	pub fn new() -> Self {
		Repl { context: Context::new(Some("<repl>".into())) }
	}
}
impl Iterator for Repl {
	type Item = quest_parser::Result<char>;
	fn next(&mut self) -> Option<Self::Item> {
		unimplemented!()
	}
}

impl Seek for Repl {
	fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
		unimplemented!()
	}
}

impl Contexted for Repl {
	fn context(&self) -> &Context {
		&self.context
	}
}

impl Stream for Repl {
	fn starts_with(&mut self, s: &str) -> quest_parser::Result<bool> {
		unimplemented!()
		// self.read_next_line_if_applicable()?;
		// Ok(self.as_ref().starts_with(s))
	}
}

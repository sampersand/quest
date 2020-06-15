use std::path::PathBuf;

pub trait Contexted {
	fn context(&self) -> &Context;
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct Context {
	pub file: Option<PathBuf>,
	pub lineno: usize,
	pub column: usize,
	pub line: String,
}

impl Context {
	pub fn new(file: Option<PathBuf>) -> Self {
		Context { file, ..Context::default() }
	}
}

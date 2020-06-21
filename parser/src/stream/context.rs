use std::path::PathBuf;

/// Types implementing this trait should be able to supply a "current execution context".
/// 
/// This is used to provide useful context for error messages.
pub trait Contexted {
	/// Get the current context.
	fn context(&self) -> &Context;
}

/// A type representing the current state of a [`Stream`](trait.Stream.html).
#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct Context {
	/// The file, if one exists, that is associated with this context
	pub file: Option<PathBuf>,
	/// The line number we're on.
	pub lineno: usize,
	/// The column within [`line`](#structfield.line) that's being parsed.
	pub column: usize,
	/// The current line that is being parsed
	pub line: String
}

impl Context {
	/// Create a new context
	pub fn new(file: Option<PathBuf>) -> Self {
		Context { file, ..Context::default() }
	}
}

impl<T: Into<PathBuf>> From<T> for Context {
	fn from(file: T) -> Context {
		Context::new(Some(file.into()))
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn default() {
		assert_eq!(
			Context::default(),
			Context { file: None, lineno: 0, column: 0, line: String::default() }
		);
	}


	#[test]
	fn new() {
		assert_eq!(Context::new(None), Context::default());
		assert_eq!(
			Context::new(Some("/plato/republic.txt".into())),
			Context { file: Some("/plato/republic.txt".into()), ..Context::default() });
	}

	#[test]
	fn from() {
		assert_eq!(
			Context::from("/plato/meno.txt"),
			Context { file: Some("/plato/meno.txt".into()), ..Context::default() });
	}
}
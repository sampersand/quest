use crate::stream::Context;
use std::fmt::{self, Display, Formatter};

/// The types of errors that can occur whilst parsing Quest code.
#[derive(Debug)]
pub enum ErrorType {
	/// The stream wasn't able to be read from.
	CantReadStream(std::io::Error),

	/// An error happend whilst tokenizing
	CantTokenize(crate::token::Error),

	/// Couldn't create an expression
	CantCreateExpression(crate::expression::Error),
}


/// An error that occurs whilst parsing Quest code.
///
/// This contains a [`Context`] for better error messages.
#[derive(Debug)]
pub struct Error {
	context: Context,
	ty: ErrorType
}

/// A wrapper around [`std::result::Result<T>`]
#[must_use]
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
	/// Create a new [`Error`] for the given [context](Context) and [error type](ErrorType).
	pub fn new(context: Context, ty: ErrorType) -> Self {
		Error { context, ty }
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		const TAB_REPLACEMENT: &str = "  ";

		let Context { ref file, lineno, mut column, ref line } = self.context;
		let file = file.as_ref()
			.map(|x| x.to_string_lossy().to_owned().to_string())
			.unwrap_or_else(|| "<eval>".to_string());

		// replace tabs with a standardized representation for error messages
		let mut line = line.clone();
		while let Some(tab_pos) = line.find('\t') {
			line.replace_range(tab_pos..=tab_pos, TAB_REPLACEMENT);
			column += TAB_REPLACEMENT.len() - 1;
		}

		write!(f, concat!("{file}:{lineno}:{column}: parse error, {error}",
					 "\n    |",
					 "\n {lineno:<3}| {context}",
					 "\n    |{padding}^ here"),
			file=file,
			lineno=lineno,
			column=column,
			error=self.ty,
			context=line.trim_end(),
			padding=" ".repeat(column))
	}
}

impl Display for ErrorType {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::CantReadStream(err) => write!(f, "can't read next character: {}", err),
			Self::CantTokenize(err) => write!(f, "can't tokenize: {}", err),
			Self::CantCreateExpression(err) => write!(f, "cant create expression: {}", err),
		}
	}
}


impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self.ty {
			ErrorType::CantReadStream(ref err) => Some(err),
			ErrorType::CantTokenize(ref err) => Some(err),
			ErrorType::CantCreateExpression(ref err) => Some(err)
		}
	}
}
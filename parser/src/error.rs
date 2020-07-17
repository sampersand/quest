use crate::stream::Context;
use crate::token::{Token, ParenType};
use std::fmt::{self, Display, Formatter};

/// The types of errors that can occur whilst parsing Quest code.
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorType {
	/// The stream wasn't able to be read from.
	CantReadStream(std::io::Error),
	/// An error happend whilst tokenizing
	CantTokenize(crate::token::Error),
	UnexpectedToken(Token),
	Message(&'static str),
	MessagedString(String),
	ExpectedExpression,
	MissingClosingParen(ParenType),

	// StreamError(std::io::Error),
	// Tokenize(TokenizeError),
	// Constructable(ConstructableError)
}


#[derive(Debug)]
pub struct Error {
	context: Context,
	r#type: ErrorType
}

#[must_use]
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
	pub fn new(context: Context, r#type: ErrorType) -> Self {
		Error { context, r#type }
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
			error=self.r#type,
			context=line.trim_end(),
			padding=" ".repeat(column))
	}
}

impl Display for ErrorType {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		use ErrorType::*;
		match self {
			CantReadStream(err) => write!(f, "can't read next character: {}", err),
			CantTokenize(err) => write!(f, "can't tokenize: {}", err),
			UnexpectedToken(tkn) => write!(f, "unexpected token `{}`", tkn),
			MissingClosingParen(paren) => write!(f, "missing closing paren `{}`", paren.right()),
			ExpectedExpression => write!(f, "expected an expression"),
			Message(msg) => write!(f, "{}", msg),
			MessagedString(msg) => write!(f, "{}", msg),
		}
	}
}


impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self.r#type {
			ErrorType::CantReadStream(ref err) => Some(err),
			ErrorType::CantTokenize(ref err) => Some(err),
			_ => None
		}
	}
}





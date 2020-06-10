use crate::stream::Context;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorType {
	CantReadStream(std::io::Error),
	BadNumber(String),
	UnterminatedBlockComment,
	UnknownTokenStart(char),
	UnterminatedQuote,
	BadEscapeChar(char),
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
		let Context { ref file, lineno, column, ref line } = self.context;
		let file = file.as_ref()
			.map(|x| x.to_string_lossy().to_owned().to_string())
			.unwrap_or_else(|| "<eval>".to_string());

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
		match self {
			ErrorType::CantReadStream(err) => write!(f, "can't read next character: {}", err),
			ErrorType::BadNumber(num) => write!(f, "bad number `{}`", num),
			ErrorType::UnknownTokenStart(chr) => write!(f, "unknown token start `{}`", chr),
			ErrorType::UnterminatedQuote => write!(f, "unterminated quote"),
			ErrorType::BadEscapeChar(chr) => write!(f, "bad escape char `{}`", chr),
			ErrorType::UnterminatedBlockComment => write!(f, "unterminated block comment"),
		}
	}
}


impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self.r#type {
			ErrorType::CantReadStream(ref err) => Some(err),
			_ => None
		}
	}
}





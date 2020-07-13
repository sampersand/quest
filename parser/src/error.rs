use crate::stream::Context;
use crate::token::{Token, ParenType};
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
#[non_exhaustive]
#[allow(clippy::module_name_repetitions)]
pub enum ErrorType {
	CantReadStream(std::io::Error),
	BadNumber(crate::token::primative::number::ParseError),
	BadRegex(crate::token::primative::regex::RegexError),
	UnterminatedBlockComment,
	UnknownTokenStart(char),
	UnterminatedQuote,
	BadEscapeChar(char),
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
	#[inline]
	pub const fn new(context: Context, r#type: ErrorType) -> Self {
		Self { context, r#type }
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		const TAB_REPLACEMENT: &str = "  ";

		let Context { ref file, lineno, mut column, ref line } = self.context;
		let file = file.as_ref()
			.map_or_else(|| "<eval>".to_string(), |x| x.to_string_lossy().to_owned().to_string());

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
		match self {
			Self::CantReadStream(err) => write!(f, "can't read next character: {}", err),
			Self::BadNumber(num) => write!(f, "bad number: {}", num),
			Self::BadRegex(err) => write!(f, "bad regex: {}", err),
			Self::UnknownTokenStart(chr) => write!(f, "unknown token start `{}`", chr),
			Self::UnterminatedQuote => write!(f, "unterminated quote"),
			Self::BadEscapeChar(chr) => write!(f, "bad escape char `{}`", chr),
			Self::UnterminatedBlockComment => write!(f, "unterminated block comment"),
			Self::UnexpectedToken(tkn) => write!(f, "unexpected token `{}`", tkn),
			Self::MissingClosingParen(paren) => write!(f, "missing closing paren `{}`", paren.right()),
			Self::ExpectedExpression => write!(f, "expected an expression"),
			Self::Message(msg) => write!(f, "{}", msg),
			Self::MessagedString(msg) => write!(f, "{}", msg),
		}
	}
}


impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self.r#type {
			ErrorType::CantReadStream(ref err) => Some(err),
			ErrorType::BadNumber(ref err) => Some(err),
			ErrorType::BadRegex(ref err) => Some(err),
			_ => None
		}
	}
}

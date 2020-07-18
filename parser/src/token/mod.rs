	//! Everything having to do with [`Tokens`] lives here.
macro_rules! unseek_char {
	(@COUNT) => { 0 };
	(@COUNT $_e:expr, $($o:tt)* ) => {{ let _ = $_e; 1 + unseek_char!(@COUNT $($o)*) }};
	($stream:expr; $($char:expr),+ $(,)?) => {{
		std::io::Seek::seek($stream, 
			std::io::SeekFrom::Current(-(unseek_char!(@COUNT $($char,)+)))
		).map_err(|err| parse_error!($stream, CantReadStream(err)))?;
	}};
}


mod text;
mod number;
mod variable;
mod stackpos;
mod regex;
mod error;
mod primative;
mod parenthesis;

pub mod operator;
mod token;

/// Represents the ability for a trait to parsed out of a series of [`Token`]s.
pub trait Tokenizable : Sized {
	/// try to create a type out of the stream.
	fn try_tokenize<S: crate::stream::Stream>(stream: &mut S) -> crate::Result<Option<Self>>;
}

pub use error::Error;
pub use parenthesis::Parenthesis;
pub use operator::Operator;
pub use primative::Primative;
pub use token::Token;
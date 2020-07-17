macro_rules! unseek_char {
	(@COUNT) => { 0 };
	(@COUNT $_e:expr, $($o:tt)* ) => {{ let _ = $_e; 1 + unseek_char!(@COUNT $($o)*) }};
	($stream:expr; $($char:expr),+ $(,)?) => {{
		std::io::Seek::seek($stream, 
			std::io::SeekFrom::Current(-(unseek_char!(@COUNT $($char,)+)))
		).map_err(|err| parse_error!($stream, CantReadStream(err)))?;
	}};
}

pub mod primative;
pub mod operator;
pub mod paren_type;
pub mod token;

pub trait Tokenizable : Sized {
	fn try_tokenize<S: crate::stream::Stream>(stream: &mut S) -> crate::Result<Option<Self>>;
}


pub use paren_type::ParenType;
pub use operator::Operator;
pub use primative::Primative;
pub use token::Token;
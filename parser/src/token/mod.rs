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
pub mod tokenizable;
pub mod whitespace;
pub mod comment;
pub mod parenthesis;
pub mod token;


pub use parenthesis::ParenType;
pub use operator::Operator;
pub use primative::Primative;
pub use tokenizable::{Tokenizable, TokenizeResult};
pub use token::Token;
macro_rules! try_seek {
	($stream:expr, $where:ident($val:expr)) => {
		std::io::Seek::seek($stream, std::io::SeekFrom::$where($val))
			.map_err(|err| parse_error!($stream, CantReadStream(err)))?
	};
	($stream:expr, $val:expr) => {
		try_seek!($stream, Current($val));
	};
}

pub mod literal;
pub mod operator;
pub mod tokenizable;
pub mod whitespace;
pub mod comment;
pub mod parenthesis;
pub mod token;


pub use parenthesis::ParenType;
pub use operator::Operator;
pub use literal::Literal;
pub use tokenizable::{Tokenizable, TokenizeResult};
pub use token::Token;
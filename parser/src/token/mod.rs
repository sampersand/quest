macro_rules! try_seek {
	($stream:expr, $where:ident($val:expr)) => {
		std::io::Seek::seek($stream, std::io::SeekFrom::$where($val))
			.map_err(|err| parse_error!($stream, CantReadStream(err)))?
	};
	($stream:expr, $val:expr) => {
		try_seek!($stream, Current($val));
	};
}

mod literal;
mod operator;
mod tokenizable;
mod whitespace;
mod comment;
mod parenthesis;
mod token;


use self::parenthesis::Parenthesis;
pub use self::parenthesis::ParenType;
pub use self::operator::Operator;
pub use self::literal::Literal;
pub use self::tokenizable::{Tokenizable, TokenizeResult};
pub use self::token::Token;
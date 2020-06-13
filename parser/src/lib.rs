#![deny(unused_must_use)]
#![allow(unused)]

macro_rules! parse_error {
	(context=$context:expr, $type:ident $($tt:tt)*) => {
		$crate::Error::new($context, $crate::ErrorType::$type$($tt)*)
	};

	($stream:expr, $type:ident $($tt:tt)*) => {
		parse_error!(context=$stream.context().clone(), $type$($tt)*)
	};
}


mod error;
pub mod token;
pub mod stream;
// mod expression;
// mod block;

// pub use self::block::{Block, Line};
pub use self::error::{Error, ErrorType, Result};
pub use self::token::{Token/*, ParenType, Literal*/};
// pub use self::expression::Expression;
pub use self::stream::{Stream, Context};
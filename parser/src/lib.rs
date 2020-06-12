#![deny(unused_must_use)]

mod token;
mod stream;
mod error;
// mod expression;
// mod block;

// pub use self::block::{Block, Line};
pub use self::error::{Error, ErrorType, Result};
pub use self::token::{Token/*, ParenType, Literal*/};
// pub use self::expression::Expression;
pub use self::stream::Stream;
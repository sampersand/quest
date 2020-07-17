//! The crate that's responsible for parsing quest code.
#![allow(clippy::module_inception)]
// #![warn(missing_docs)]

/// Setup the quest parser. This should be run before anything within `quest_parser` is used.
pub fn init() {
	use quest_core::types::ObjectType;
	quest_core::types::Kernel::mapping().set_attr_lit("Block", Block::mapping());
}

#[macro_use]
mod macros;
mod error;
pub mod expression;
pub mod token;
pub mod stream;
pub mod block;

// TODO: change public exports to more minimal.
pub use block::Block;
pub use error::{Error, ErrorType, Result};
pub use token::Token;
pub use expression::Expression;
pub use stream::{Stream, Context, Contexted};

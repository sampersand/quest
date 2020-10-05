#![allow(clippy::module_inception, clippy::missing_const_for_fn)]

/// Setup the quest parser. This should be run before anything within `quest_parser` is used.
pub fn init() {
	use quest_core::{Object, types::{ObjectType, RustFn, Text, Kernel, rustfn::Binding}};
	use crate::expression::Executable;

	use std::sync::Once;

	static INITIALIZE: Once = Once::new();

	INITIALIZE.call_once(|| {
		Block::initialize().expect("couldn't initialize block");

		Kernel::mapping().set_attr_lit("Block", Block::mapping().clone())
			.expect("couldn't defined Block");

		Text::mapping().set_value_lit("eval", RustFn::new("Text::eval", |this, args| {
			fn execute_text(text: String) -> quest_core::Result<Object> {
				Expression::parse_stream(stream::BufStream::from(text).tokens())
					.map_err(|err| Box::new(err) as Box<_>)?
					.execute()
			}

			this.try_downcast::<Text>().and_then(|this| {
				if let Ok(binding) = args.try_arg(0) {
					Binding::new_stackframe(Some(binding.clone()), args, |_| execute_text(this.to_string()))
				} else {
					execute_text(this.to_string())
				}
			})
		})).expect("couldn't define `eval`");
	});
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

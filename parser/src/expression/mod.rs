//! Anything relating to [`Expression`]s lives here.
mod block;
mod expression;
mod constructor;
mod bound_operator;
mod error;

/// Represents the ability for a type to be executed.
/// 
/// Executing a type means that you return a [`Object`](quest_core::Object) that corresponds to
/// its contents. 
pub trait Executable {
	/// Execute the obejct.
	fn execute(&self) -> quest_core::Result<quest_core::Object>;
}

/// A temporary hack to allow for "un-reading" items.
///
/// _Some_ form of this is required, due toQquest's design, but this current implementation isn't
/// ideal.
pub trait PutBack : Iterator {
	/// unread the item.
	fn put_back(&mut self, item: Self::Item);
}

/// Represents the ability for a type to be constructed from a stream of [`Token`](
/// crate::token::Token)s.
///
/// (Well, technically, an [`Iterator`] over [`Result<Token>`](crate::Result), but whatever.)
pub trait Constructable : Sized {
	/// Try to construct this value out of the series of tokens.
	/// 
	/// This is a bad name and should probably be renamed `try_construct` in the future.
	fn try_construct_primary<C>(ctor: &mut C) -> crate::Result<Option<Self>>
	where
		C: Iterator<Item=crate::Result<crate::token::Token>> + PutBack + crate::stream::Contexted;
}

pub(crate) use constructor::Constructor;
pub use bound_operator::BoundOperator;
pub use expression::Expression;
pub use block::Block;
pub use error::Error;
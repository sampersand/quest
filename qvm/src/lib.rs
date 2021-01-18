//! The execution environment for Quest.
#![allow(unused, deprecated)]

#[macro_use]
extern crate static_assertions;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate tracing;

#[macro_use]
mod macros;

mod literal;
mod lmap;
mod alloc;
mod eval;
pub mod error;
pub mod value;


pub use literal::Literal;

#[doc(inline)]
pub use value::{Value, ValueType};

#[doc(inline)]
pub use error::*;

/// Initializes Quest. This should be run before any other function is called, and may be repeatedly called.
pub fn initialize() {
	literal::initialize();
	value::initialize();
}

/// Indicates the ability for a type to be shallowly copied
pub trait ShallowClone : Sized {
	fn shallow_clone(&self) -> crate::Result<Self>;
}

/// Indicates the ability for a type to be deeply copied
pub trait DeepClone : Sized {
	/// Copies the actual data of the object.
	///
	/// When you [`clone()`] a [`Value`], you're actually just creating another reference to the
	/// same object in memory. This actually creates another distinct object.
	fn deep_clone(&self) -> crate::Result<Self>;
}

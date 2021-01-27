//! The execution environment for Quest.
#![allow(unused)]

#[macro_use]
extern crate qvm_derive;
pub use qvm_derive::*;

#[macro_use]
extern crate static_assertions;

#[macro_use]
extern crate tracing;

mod literal;
mod lmap;
mod alloc;
mod eval;
mod traits;
pub mod error;
pub mod value;

pub use traits::*;
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

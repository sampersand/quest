#![allow(unused, deprecated)]

#[macro_use]
extern crate static_assertions;

#[macro_use]
extern crate bitflags;

#[macro_use]
mod macros;

mod literal;
mod lmap;
mod alloc;
mod eval;
pub mod value;

pub use value::{Value, ValueType};
pub use lmap::LMap;
pub use literal::Literal;

#[derive(Debug)]
pub enum Error {
	TypeError(String)
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn initialize() {
	literal::initialize();
}

/// Indicates the ability for a type to be shallowly copied
pub trait ShallowClone : Sized {
	fn shallow_clone(&self) -> crate::Result<Self>;
}

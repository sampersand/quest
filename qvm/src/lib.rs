#![allow(unused)]

#[macro_use]
extern crate static_assertions;

#[macro_use]
extern crate bitflags;

mod literal;
mod lmap;
mod alloc;
mod eval;
pub mod value;


pub use value::{Value, QuestValue};
pub use lmap::LMap;
pub use literal::Literal;

pub enum Error {

}

pub type Result<T> = std::result::Result<T, Error>;

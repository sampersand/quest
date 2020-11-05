#![allow(unused)]
// pub mod bytecode;
mod literal;
mod rustfn;
pub mod value;
pub mod block;
mod error;

use block::Block;
use literal::Literal;
pub use value::Value;

pub use error::{Error, Result};
pub use rustfn::RustFn;

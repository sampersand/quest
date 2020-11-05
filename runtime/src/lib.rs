#![allow(unused)]

extern crate static_assertions as sa;

mod int48;
mod rustfn;
pub mod qvm;
pub mod value;
pub mod block;
mod error;

use block::Block;
pub use value::Value;

pub use error::{Error, Result};
pub use rustfn::RustFn;
use int48::Int48 as Integer;

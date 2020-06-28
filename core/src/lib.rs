#![deny(warnings)]
#![allow(deprecated)]
#![feature(never_type)]
extern crate rand;

mod error;

pub mod obj;
pub mod types;
pub mod literals;

pub use obj::{Object, ToObject};
pub use error::{Error, Result};
pub use types::rustfn::{ArgsOld, Args, Binding};

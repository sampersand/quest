#![deny(warnings)]
#![allow(deprecated)]
#![feature(never_type)]
extern crate rand;

mod error;
mod arc_cow;


pub fn init() {
	/* todo: move all mapping initialization stuff here. */
}

pub mod obj;
pub mod types;
pub mod literals;

use arc_cow::ArcCow;
pub use obj::{Object, ToObject};
pub use error::{Error, Result};
pub use types::rustfn::{Args, Binding};

#[deprecated]
pub use types::rustfn::ArgsOld;
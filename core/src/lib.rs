#![deny(warnings)]
#![feature(never_type)]
extern crate rand;

macro_rules! unreachable_debug_or_unchecked {
	() => {
		if cfg!(debug_assertions) {
			unreachable!()
		} else {
			#[allow(unused_unsafe)]
			unsafe { ::std::hint::unreachable_unchecked() }
		}
	};
}

mod shared_cow;
pub mod error;


pub fn init() {
	/* todo: move all mapping initialization stuff here. */
}

pub mod utils;
pub mod obj;
pub mod types;
pub mod literals;

use shared_cow::SharedCow;
pub use obj::{Object, ToObject};
pub use error::{Error, Result};
pub use types::rustfn::{Args, Binding};
pub use literals::Literal;
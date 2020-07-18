#![allow(clippy::unnested_or_patterns, clippy::pub_enum_variant_names)]
#![feature(never_type)]

#![allow(
	// TODO
	clippy::missing_safety_doc,
)]

macro_rules! unreachable_debug_or_unchecked {
	() => {
		if cfg!(debug_assertions) {
			unreachable!()
		} else {
			::std::hint::unreachable_unchecked()
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
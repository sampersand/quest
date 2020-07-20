//! The core of the Quest Programming language.
//!
//! All the functionality required to actually execute Quest live here.
//!
//! # See Also
//! - [`quest-parser`](#TODO) for parsing quest
//! - [`quest-bin`](#TODO) the quest executable
#![allow(clippy::unnested_or_patterns, clippy::pub_enum_variant_names)]
#![feature(never_type)]

#![allow(
	// TODO
	clippy::missing_safety_doc,
)]

mod shared_cow;
mod obj;
#[doc(hidden)]
pub mod utils;
pub mod error;
pub mod types;
pub mod literals;

use shared_cow::SharedCow;
pub use obj::Object;
pub use error::{Error, Result};
pub use types::rustfn::{Args, Binding};

pub fn init() {
	/* todo: move all mapping initialization stuff here. */
}

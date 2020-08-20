// #![warn(missing_docs)]

//! The core of the Quest Programming language.
//!
//! All the functionality required to actually execute Quest live here.
//!
//! # See Also
//! - [`quest-parser`](#TODO) for parsing quest
//! - [`quest-bin`](#TODO) the quest executable
#![allow(clippy::pub_enum_variant_names)]

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod shared_cow;
mod obj;
mod literal;
pub mod utils;
pub mod error;
pub mod types;

use shared_cow::SharedCow;
pub use literal::Literal;
pub use obj::Object;
pub use error::{Error, Result};
pub use types::rustfn::{Args, Binding};

/// Start up Quest by initializing all the types.
pub fn initialize() {
	use types::*;
	use parking_lot::Once;

	macro_rules! initialize {
		($($ty:ty),*) => {{
			$(
				<$ty>::initialize().expect(concat!("couldn't initialize ", stringify!($ty)));
			)*
		}};
	}

	static INITIALIZE: Once = Once::new();

	INITIALIZE.call_once(||
		initialize!(
			Pristine, Basic, Boolean, BoundFunction, Comparable, Function, Kernel,
			List, Null, Number, Regex, RustFn, Scope, Text, Iterable, Tcp,
			iterable::BoundRustFn
		)
	)
}

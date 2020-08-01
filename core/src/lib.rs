//! The core of the Quest Programming language.
//!
//! All the functionality required to actually execute Quest live here.
//!
//! # See Also
//! - [`quest-parser`](#TODO) for parsing quest
//! - [`quest-bin`](#TODO) the quest executable
#![allow(clippy::pub_enum_variant_names)]


#![allow(
	// TODO
	clippy::missing_safety_doc,
)]

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod shared_cow;
#[doc(hidden)]
pub mod obj;
pub mod utils;
pub mod error;
pub mod types;
pub mod literal;

use shared_cow::SharedCow;
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

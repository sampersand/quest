use crate::{Literal, Object, Args, Result};

/// Get a human-readable representation of the object.
pub trait Inspect {
	/// The Quest name for the method. (defaults to `inspect`).
	const METHOD: Literal = "inspect";

	/// Inspects the object.
	#[allow(non_snake_case)]
	fn qs_inspect(this: &Object, args: Args) -> Result<Object>;
}

/// Hashing values
pub trait Hash {
	/// The Quest name for the method. (defaults to `hash`).
	const METHOD: Literal = "hash";

	/// Hash the object
	fn qs_hash(this: &Object, args: Args) -> Result<Object>;
}
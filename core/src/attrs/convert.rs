use crate::{Literal, Object, Args, Result};

/// Convert a type to a [`Text`](crate::types::Text).
pub trait AtText {
	/// The Quest name for the method (defaults to `@text`).
	const METHOD: Literal = "@text";

	/// Convert this into a [`Text`](crate::types::Text).
	fn qs_at_text(this: &Object, args: Args) -> Result<Object>;
}

/// Convert a type to a [`Number`](crate::types::Number)
pub trait AtNumber {
	/// The Quest name for the method (defaults to `@num`).
	const METHOD: Literal = "@num";

	/// Convert this into a [`Number`](crate::types::Number).
	fn qs_at_num(this: &Object, args: Args) -> Result<Object>;
}

/// Convert a type to a [`Boolean`](crate::types::Boolean).
pub trait AtBoolean {
	/// The Quest name for the method (defaults to `@bool`).
	const METHOD: Literal = "@bool";

	/// Convert this into a [`Boolean`](crate::types::Boolean).
	fn qs_at_bool(this: &Object, args: Args) -> Result<Object>;
}

/// Convert a type to a [`List`](crate::types::List)
pub trait AtList {
	/// The Quest name for the method (defaults to `@list`).
	const METHOD: Literal = "@list";

	/// Convert this into a [`List`](crate::types::List).
	fn qs_at_list(this: &Object, args: Args) -> Result<Object>;
}
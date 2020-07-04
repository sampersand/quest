use crate::{Literal, Object, Args, Result};

pub trait AtNumber {
	const METHOD: Literal = "@num";

	fn at_num(this: &Object, args: Args) -> Result<Object>;
}


impl AtNumber for crate::types::Boolean {
	/// Convert this into a [`Number`].
	///
	/// This is simply a wrapper around [`Number::from(Boolean)`](Number#impl-From<Boolean>).
	fn at_num(this: &Object, _: Args) -> Result<Object> {
		this.try_downcast_map(|this: &Self| crate::types::Number::from(*this).into())
	}
}

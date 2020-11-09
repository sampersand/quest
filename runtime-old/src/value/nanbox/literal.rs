use super::{Value, Tag};

/// A literal, used to quickly and uniquely identify functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Literal(u32);

impl Literal {
	pub const NOT: Self = Self(0);

	pub fn new(_literal: &'static str) -> Self {
		todo!()
	}
}


impl Value {
	/// Creates a new [`Literal`] [`Value`].
	pub fn new_literal(literal: Literal) -> Self {
		let [_, _, data @ ..] = (literal.0 as u64).to_be_bytes();

		// SAFETY: This is guaranteed to be safe because `literal`'s a valid literal, and converting it into its raw
		// representation doesn't change that.
		unsafe {
			Self::new_tagged(Tag::LITERAL, data)
		}
	}

	/// Checks to see if `self` is a [`Literal`].
	pub fn is_literal(&self) -> bool {
		self.tag() == Tag::LITERAL
	}

	/// Returns the underlying [`Literal`] if `self` is a literal.
	pub fn as_literal(&self) -> Option<Literal> {
		if self.is_literal() {
			// SAFETY: Assuming we were created from `new_literal`, the data will be a valid literal.
			Some(Literal(self.masked_data() as u32))
		} else {
			None
		}
	}
}

use std::fmt::{self, Display, Formatter};

/// A type representing a class within Quest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Class(&'static str);

impl Class {
	/// Create a new [`Class`] with the given name.
	#[inline]
	pub const fn new(name: &'static str) -> Self {
		Self(name)
	}

	#[inline]
	pub const fn name(&self) -> &str {
		self.0
	}
}

impl Display for Class {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

// /// Quest functions
// impl Boolean {
// 	/// Inspects `this`.
// 	pub fn qs_inspect(this: &Object, _: Args) -> Result<Object> {
// 		write!(this, )
// 		Self::qs_at_text(this, args)
// 	}
// }


impl_object_type! {
for Class [(parents super::Basic) (no_convert)]:
}

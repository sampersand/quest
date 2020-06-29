use crate::{Args, obj::Key};

pub trait QsEql {
	const KEY: Key = Key::Literal("==");

	type Error: Into<crate::Error>;
	fn qs_eql(&self, args: Args) -> Result<bool, Self::Error>;
}

impl<T: PartialEq + 'static> QsEql for T {
	type Error = crate::error::KeyError;

	#[inline]
	fn qs_eql(&self, args: Args) -> Result<bool, Self::Error> {
		if let Some(rhs) = args.arg(0)?.downcast_ref::<Self>() {
			Ok(*self == *rhs)
		} else {
			Ok(false)
		}
	}
}
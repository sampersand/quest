//! Errors that can occur within Quest.
use crate::{Object, Binding};
use std::fmt::{self, Display, Formatter};

mod key_error;
mod type_error;
mod value_error;

pub use type_error::TypeError;
pub use key_error::KeyError;
pub use value_error::ValueError;

#[derive(Debug)]
#[non_exhaustive]
/// The generic type for all errors that can occur within quest.
pub enum Error {
	/// Something that we don't have an error type for yet
	Messaged(String),

	/// An invalid key was requested
	KeyError(KeyError),

	/// An invalid type was supplied somewhere
	TypeError(TypeError),

	/// An invalid value was supplied somewhere
	ValueError(ValueError),

	/// Some quest assertion failed.
	AssertionFailed(Option<String>),

	/// Boxed error
	Boxed(Box<dyn std::error::Error + Send + Sync + 'static>),

	/// Returning a value.
	///
	/// While this isn't technically an "error" in the strict sense of an error, it's much easier
	/// to propegate errors with this mechanism than any other one.
	Return {
		/// The place to return to.
		to: Binding,
		/// The value to return.
		obj: Object
	}
}

#[must_use]
/// An alias for results within quest.
pub type Result<T> = ::std::result::Result<T, Error>;

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::Messaged(err) => Display::fmt(&err, f),
			Self::KeyError(err) => Display::fmt(&err, f),
			Self::TypeError(err) => Display::fmt(&err, f),
			Self::ValueError(err) => Display::fmt(&err, f),
			Self::AssertionFailed(Some(err)) => {
				Display::fmt(&"assertion failed: ", f)?;
				Display::fmt(&err, f)
			},
			Self::AssertionFailed(None) => Display::fmt("assertion failed", f),
			Self::Boxed(err) => Display::fmt(&err, f),
			Self::Return { to, obj } => write!(f, "uncaught return to {:?}: {:?}", to, obj)
		}
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		if let Self::Boxed(err) = self {
			Some(err.as_ref())
		} else {
			None
		}
	}
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
	fn from(boxed_err: Box<dyn std::error::Error + Send + Sync>) -> Self {
		Self::Boxed(boxed_err)
	}
}

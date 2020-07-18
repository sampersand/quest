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
pub enum Error {
	/// Something internal that shouldn't have occured
	Internal(&'static str),

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
	Boxed(Box<dyn std::error::Error + 'static>),

	/// Returning a value
	Return { to: Binding, obj: Object }
}

impl From<String> for Error {
	fn from(err: String) -> Self { Error::Messaged(err) }
}

impl From<!> for Error {
	fn from(_: !) -> Self {
		unsafe { unreachable_debug_or_unchecked!(); }
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Error::Internal(err) => write!(f, "internal error: {}", err),
			Error::Messaged(err) => Display::fmt(&err, f),
			Error::KeyError(err) => Display::fmt(&err, f),
			Error::TypeError(err) => Display::fmt(&err, f),
			Error::ValueError(err) => Display::fmt(&err, f),
			Error::AssertionFailed(Some(err)) => write!(f, "assertion failed: {}", err),
			Error::AssertionFailed(None) => write!(f, "assertion failed"),
			Error::Boxed(err) => Display::fmt(&err, f),
			Error::Return { to, obj } => write!(f, "uncaught return to {:?}: {:?}", to, obj)
		}
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Error::Boxed(err) => Some(err.as_ref()),
			_ => None
		}
	}
}

#[must_use]
pub type Result<T> = ::std::result::Result<T, Error>;

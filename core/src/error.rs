use crate::Object;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Error {
	Object(Object),
}

impl From<Object> for Error {
	fn from(obj: Object) -> Error {
		Error::Object(obj)
	}
}

#[deprecated]
impl From<String> for Error {
	fn from(err: String) -> Self { Error::Object(Object::from(err)) }
}

#[deprecated]
impl From<crate::types::Text> for Error {
	fn from(err: crate::types::Text) -> Self { Error::Object(Object::from(err)) }
}

#[deprecated]
impl From<&'_ str> for Error {
	fn from(err: &'_ str) -> Self { Error::Object(Object::from(err.to_string())) }
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Error::Object(obj) => std::fmt::Debug::fmt(&obj, f)
		}
	}
}

impl std::error::Error for Error {
	// there's no cause currently
}

#[must_use]
pub type Result<T> = ::std::result::Result<T, Error>;
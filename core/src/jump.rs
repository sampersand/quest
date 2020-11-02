mod exception;
pub use exception::Exception;

use crate::{Object, Binding};
use std::fmt::{self, Display, Formatter};

/// A type that indicates execution should go somewhere else.
#[derive(Debug)]
#[non_exhaustive] // we may add call/cc later?
pub enum Jump {
	/// Resumes execution at the given binding as if its callee returned `result`.
	Return { to: Binding, result: Option<Object> },
	
	/// Raises an exception, going up stackframes until a `catch` block is encountered.
	Exception(Exception),

	Yield { /* TODO: how the heck should yield work */ }
}

impl From<Exception> for Jump {
	#[inline]
	fn from(exception: Exception) -> Self {
		Self::Exception(exception)
	}
}

/// A type alias for returning `T`s or [`Jump`]s.
pub type Result<T> = ::std::result::Result<T, Jump>;

impl std::error::Error for Jump {}

impl Display for Jump {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::Return { to, result: None } => write!(f, "return to {:?}", to),
			Self::Return { to, result: Some(result) } => write!(f, "return to {:?} with result {:?}", to, result),
			Self::Exception(exception) => Display::fmt(exception, f),
			Self::Yield { } => todo!("display for yield.")
		}		
	}
}



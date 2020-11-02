use std::any::Any;
use crate::{Binding, Object};
use crate::types::rustfn::StackTrace;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};

/// Exceptions that are possible in Quest.
///
/// These will have an associated stacktrace with them. Check out [`Jump`] for options
/// that don't.
#[derive(Debug)]
pub struct Exception {
	stacktrace: StackTrace,
	error: ExceptionType
}

pub enum ExceptionType {
	/// Raised when there's a problem when passing arguments, such as when too few are given.
	pub struct ArgumentError;

	/// Raised when there's a problem accessing fields, such as when one doesn't exist.
	pub struct FieldError;

	/// Raised when there's a problem with a type, such as when an object cannot be converted correctly.
	pub struct TypeError;

	/// Raised when there's a problem with a value, such as attempting to convert "a" to a [`Number`](
	/// crate::types::Number).
	pub struct ValueError;

	Custom(Box<dyn Error>),
	Quest(Object)
}

/// A trait that represents an error in Quest.
///
/// This is _almost_ the same as `std::error::Error + Clone + PartialEq + Send + Sync + 'static`,
/// except we need to have it as a trait object. As such, `dyn_clone` and `dyn_eq` should be used instead.
pub trait Error : StdError + Any + Send + Sync + 'static {
	/// Same as [`Clone::clone`], but return a `Box<dyn Error>` so we don't need to have a sized `Self`.
	fn dyn_clone(&self) -> Box<dyn Error>;

	/// Same as [`PartialEq::eq`], except `rhs` should be downcast (so we don't need to reference `Self`).
	fn dyn_eq(&self, rhs: &dyn Error) -> bool;

	/// Converts `self` to an `Any`, so as to to be able to be downcast.
	fn as_any(&self) -> &dyn Any;
}

impl<T: StdError + Send + Sync + 'static + Clone + PartialEq> Error for T {
	fn dyn_clone(&self) -> Box<dyn Error> {
		Box::new(self.clone())
	}

	fn dyn_eq(&self, rhs: &dyn Error) -> bool {
		rhs.as_any().downcast_ref::<Self>().map_or(false, |rhs| self == rhs)
	}

	#[inline]
	fn as_any(&self) -> &dyn Any {
		self as _
	}
}

impl Exception {
	/// Creates a new exception with the current stacktrace.
	pub fn new<E: Error>(error: E) -> Self {
		Self::with_stacktrace(error, Binding::stacktrace())
	}

	/// Creates a new exception with an explicitly given stacktrace.
	pub fn with_stacktrace<E: Error>(error: E, stacktrace: StackTrace) -> Self {
		Self { error: Box::new(error), stacktrace }
	}

	/// Gets the stacktrace for this exception.
	#[inline]
	pub fn stacktrace(&self) -> &StackTrace {
		&self.stacktrace
	}

	/// Gets the error associated with this exception.
	#[inline]
	pub fn error(&self) -> &dyn Error {
		self.error.as_ref()
	}
}

impl Display for Exception {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "uncaught exception: {}", self.error)
	}
}

macro_rules! exception_type {
	($($(#[$meta:meta])* pub struct $name:ident;)*) => {
		$(
			$(#[$meta])*
			#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
			pub struct $name {
				message: String
			}

			impl $name {
				/// Creates a new exception with the given message.
				#[inline]
				pub fn new<M: Into<String>>(message: M) -> Self {
					Self::from(message.into())
				}

				/// Retrieves the message that was used to make this exception.
				pub fn message(&self) -> &str {
					&self.message
				}
			}

			impl From<String> for $name {
				#[inline]
				fn from(message: String) -> Self {
					Self { message }
				}
			}

			impl From<&str> for $name {
				#[inline]
				fn from(message: &str) -> Self {
					Self::from(message.to_string())
				}
			}

			impl std::error::Error for $name {}

			impl Display for $name {
				fn fmt(&self, f: &mut Formatter) -> fmt::Result {
					if self.message.is_empty() {
						write!(f, stringify!($name))
					} else {
						write!(f, concat!(stringify!($name), ": {}"), self.message)
					}
				}
			}

			impl From<$name> for Exception {
				fn from(error: $name) -> Self {
					Self::new(error)
				}
			}
		)*
	};
}

exception_type! {
	/// Raised when there's a problem when passing arguments, such as when too few are given.
	pub struct ArgumentError;

	/// Raised when there's a problem accessing fields, such as when one doesn't exist.
	pub struct FieldError;

	/// Raised when there's a problem with a type, such as when an object cannot be converted correctly.
	pub struct TypeError;

	/// Raised when there's a problem with a value, such as attempting to convert "a" to a [`Number`](
	/// crate::types::Number).
	pub struct ValueError;
}

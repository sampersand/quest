//! Types relating to errors that can occur during Quest's execution.

/// Errors that can exist during Quest's execution.
#[derive(Debug)]
pub enum Error {
	TypeError(String)
}

/// A type alias for errors.
pub type Result<T> = std::result::Result<T, Error>;

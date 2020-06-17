use crate::Object;

pub type Error = Object;

#[must_use]
pub type Result<T> = ::std::result::Result<T, Object>;
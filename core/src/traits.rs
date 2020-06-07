use crate::{Object, Result};
use std::fmt::Debug;

pub trait EqResult<Rhs = Self> : Debug
where
	Rhs: ?Sized
{
	fn equals(&self, rhs: &Rhs) -> Result<bool>;
	fn into_object(&self) -> Object {
		format!("{:?}", self).into()
	}
}
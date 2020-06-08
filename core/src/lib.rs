#![deny(warnings)]
extern crate rand;

pub trait EqResult<Rhs = Self> : std::fmt::Debug
where
	Rhs: ?Sized
{
	fn equals(&self, rhs: &Rhs) -> Result<bool>;

	fn into_object(&self) -> Object {
		format!("{:?}", self).into()
	}
}

mod error;

pub mod obj;
pub mod types;
pub mod literals;

pub use self::obj::{Object, Key, Value};
pub use self::error::Result;
pub use self::types::rustfn::{Args, Binding};

mod obj;
pub mod mapping;
mod result;

pub mod literals;
pub mod types;


pub trait EqResult<Rhs = Self> : std::fmt::Debug
where Rhs: ?Sized {
	fn equals(&self, rhs: &Rhs) -> Result<bool>;
	fn into_object(&self) -> Object { format!("{:?}", self).into() }
}
// impl<L: PartialEq<R>, R> EqResult<R> for L {
// 	fn equals(&self, rhs: &R) -> Result<bool> {
// 		Ok(self == rhs)
// 	}
// }

use self::mapping::Mapping;
pub use self::obj::Object;
pub use self::types::rustfn::{Args, Binding};
pub use self::result::Result;
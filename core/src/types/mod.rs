use std::{fmt::Debug, any::Any};
use crate::Object;

#[macro_use]
pub mod macros;

mod convert;

pub trait ObjectType : Debug + Any + Send + Sync + Clone {
	fn mapping() -> Object;

	#[inline]
	fn new_object(self) -> Object where Self: Sized {
		Object::new_with_parent(self, vec![Self::mapping()])
	}

	// #[cfg(test)]
	// todo: remove this
	fn _wait_for_setup_to_finish() {}
}

mod pristine;
mod kernel;
mod basic;

mod function;
mod comparable;
mod bound_function;
mod scope;
pub mod rustfn;

pub mod tcp;

mod null;
mod boolean;
pub mod number;
mod text;
pub mod regex;
mod list;

pub use convert::Convertible;
pub use comparable::Comparable;
pub use pristine::Pristine;
pub use kernel::Kernel;
pub use basic::Basic;
pub use bound_function::BoundFunction;
pub use function::Function;
pub use rustfn::RustFn;
pub use scope::Scope;
pub use null::Null;
pub use boolean::Boolean;
pub use number::Number;
pub use text::Text;
pub use list::List;
pub use self::regex::Regex;

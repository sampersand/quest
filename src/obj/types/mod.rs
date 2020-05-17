#[macro_use]
mod macros;
mod convert;

pub trait ObjectType : std::fmt::Debug + std::any::Any + Send + Sync + Clone {
	fn mapping() -> super::Object;
	#[cfg(test)]
	fn _wait_for_setup_to_finish() {}
}

pub mod pristine;
pub mod kernel;
pub mod basic;

pub mod function;
pub mod rustfn;
pub mod block;

pub mod null;
pub mod boolean;
pub mod number;
pub mod text;

pub mod list;
pub mod map;

pub use self::convert::Convertible;
pub use self::pristine::Pristine;
pub use self::kernel::Kernel;
pub use self::basic::Basic;
pub use self::function::Function;
pub use self::rustfn::{RustFn, Args};
pub use self::block::Block;
pub use self::null::Null;
pub use self::boolean::Boolean;
pub use self::number::Number;
pub use self::text::Text;
pub use self::list::List;
pub use self::map::Map;
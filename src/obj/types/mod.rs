#[macro_use]
mod macros;

pub trait ObjectType : ::std::fmt::Debug + Send + Sync + ::std::any::Any + Clone {
	fn mapping() -> super::Object;
}

pub mod pristine;
pub mod basic;

pub mod function;
pub mod rustfn;

pub mod null;
pub mod boolean;
pub mod number;
pub mod text;

pub mod list;
pub mod map;

pub use self::pristine::Pristine;
pub use self::basic::Basic;
pub use self::function::Function;
pub use self::rustfn::{RustFn, Args};
pub use self::null::Null;
pub use self::boolean::Boolean;
pub use self::number::Number;
pub use self::text::Text;
pub use self::list::List;
pub use self::map::Map;
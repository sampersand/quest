#[macro_use]
mod macros;

pub trait ObjectType : ::std::fmt::Debug + Send + Sync + ::std::any::Any + Clone {
	fn mapping() -> super::Object;
}

pub mod pristine;
pub use self::pristine::Pristine;

pub mod basic;
pub use self::basic::Basic;

pub mod function;
pub use self::function::Function;

pub mod rustfn;
pub use self::rustfn::{RustFn, Args};

pub mod null;
pub use self::null::Null;

pub mod number;
pub use self::number::Number;

pub mod text;
pub use self::text::Text;

pub mod boolean;
pub use self::boolean::Boolean;

pub mod list;
pub use self::list::List;

pub mod map;
pub use self::map::Map;
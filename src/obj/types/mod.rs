#[macro_use]
mod macros;

pub mod null;
pub mod number;
pub mod rustfn;
pub mod text;
pub mod boolean;
pub mod basic;
pub mod pristine;

pub trait ObjectType : Into<super::DataEnum> + ::std::fmt::Debug + Send + Sync {
	fn mapping() -> super::Object;
}

pub use self::basic::Basic;
pub use self::pristine::Pristine;
pub use self::boolean::Boolean;
pub use self::text::Text;
pub use self::null::Null;
pub use self::number::Number;
pub use self::rustfn::RustFn;
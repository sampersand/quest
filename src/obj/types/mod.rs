pub mod null;
pub mod number;
pub mod rustfn;
pub mod text;
pub mod boolean;


pub trait ObjectType : Into<super::DataEnum> + ::std::fmt::Debug + Send + Sync {
	fn mapping() -> ::std::sync::Arc<::std::sync::RwLock<super::Mapping>>;
}

pub use self::boolean::Boolean;
pub use self::text::Text;
pub use self::null::Null;
pub use self::number::Number;
pub use self::rustfn::RustFn;
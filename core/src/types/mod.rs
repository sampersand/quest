//! The core list of types within Quest.
// #![warn(missing_docs)]
// #![allow(missing_docs)]

#[macro_use]
mod macros;
mod convert;

/// A trait representing the ability to have default associated attribuets.
pub trait ObjectType : Send + Sync + Clone + std::fmt::Debug + 'static {
	/// The list of attributes that objects of this type will have.
	fn mapping() -> &'static crate::Object;

	/// initialize an object type's mapping.
	fn initialize() -> crate::Result<()>;

	/// Convert `self` into an [`Object`].
	///
	/// The default implementation simply calls [`Object::new_with_parent`] with the `parents` arg
	/// arg as [`Self::mapping()`](ObjectType::mapping), but it can be overwritten to perform
	/// cacheing of intermediate results.
	#[inline]
	fn new_object(self) -> crate::Object {
		crate::Object::new_with_parent(self, vec![Self::mapping()])
	}
}

mod pristine;
mod kernel;
mod basic;

mod function;
pub mod iterable;
mod comparable;
mod bound_function;
mod scope;
pub mod rustfn;

mod null;
mod boolean;
mod class;
pub mod number;
mod text;
pub mod regex;
mod list;

mod tcp;
pub use tcp::Tcp;

pub use class::Class;
pub use convert::Convertible;
pub use comparable::Comparable;
pub use pristine::Pristine;
pub use kernel::Kernel;
pub use basic::Basic;
pub use bound_function::BoundFunction;
pub use function::Function;
pub use iterable::Iterable;
pub use rustfn::RustFn;
pub use scope::Scope;
pub use null::Null;
pub use boolean::Boolean;
pub use number::Number;
pub use text::Text;
pub use list::List;
pub use self::regex::Regex;

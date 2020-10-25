//! The core list of types within Quest.
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
mod iterable;
mod comparable;
mod bound_function;
mod scope;
pub mod rustfn;

mod null;
mod class;
mod text;
pub mod boolean;
pub mod number;
pub mod regex;
pub mod io;
mod list;

mod tcp;

pub use function::BoundRustFn;

#[doc(inline)]
pub use io::Io;

#[doc(inline)]
pub use tcp::Tcp;

#[doc(inline)]
pub use class::Class;

#[doc(inline)]
pub use convert::Convertible;

#[doc(inline)]
pub use comparable::Comparable;

#[doc(inline)]
pub use pristine::Pristine;

#[doc(inline)]
pub use kernel::Kernel;

#[doc(inline)]
pub use basic::Basic;

#[doc(inline)]
pub use bound_function::BoundFunction;

#[doc(inline)]
pub use function::Function;

#[doc(inline)]
pub use iterable::Iterable;

#[doc(inline)]
pub use rustfn::{RustFn, RustClosure};

#[doc(inline)]
pub use scope::Scope;

#[doc(inline)]
pub use null::Null;

#[doc(inline)]
pub use boolean::Boolean;

#[doc(inline)]
pub use number::Number;

#[doc(inline)]
pub use text::Text;

#[doc(inline)]
pub use list::List;

#[doc(inline)]
pub use self::regex::Regex;


use crate::{Object, Args, Result};
use crate::types::{Boolean, List, Number, Text};
use std::fmt::{self, Display, Formatter};
use tracing::instrument;

/// A type that represents "nothing" in Quest.
///
/// Functions that are only used for their side effects will generally return [`Null`], as well as certain situations
/// where data cannot be accessed.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Null;

impl Display for Null {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.write_str("null")
	}
}

impl Null {
	#[inline(always)]
	pub const fn new() -> Self {
		Self
	}
}

impl From<()> for Object {
	#[inline]
	fn from(_: ()) -> Self {
		Null.into()
	}
}

impl From<()> for Null {
	#[inline]
	fn from(_: ()) -> Self {
		Self
	}
}

impl From<Null> for Boolean {
	#[inline]
	fn from(_: Null) -> Self {
		Self::default()
	}
}

impl From<Null> for List {
	#[inline]
	fn from(_: Null) -> Self {
		Self::default()
	}
}

impl From<Null> for Number {
	#[inline]
	fn from(_: Null) -> Self {
		Self::default()
	}
}

impl From<Null> for Text {
	#[inline]
	fn from(_: Null) -> Self {
		Self::const_new("null")
	}
}

impl Null {
	/// Inspects `this`.
	///
	/// # Arguments
	/// None.
	///
	/// # Returns
	/// A [`Text`] object containing `"null"`.
	///
	/// # Errors
	/// None.
	///
	/// # Rust Examples
	/// ```rust
	/// use quest_core::types::{Null, Text};
	///
	/// assert_eq!(
	/// 	*Null::qs_inspect(&Null.into(), Default::default()).unwrap()
	/// 		.downcast::<Text>().unwrap(),
	/// 	Text::new("null")
	/// );
	/// ```
	///
	/// # Quest Examples
	/// ```quest
	/// assert(null.$inspect() == "null");
	/// ```
	///
	/// # See Also
	/// - [`Null::qs_at_text`] -- Identical to this function.
	#[instrument(name="Null::inspect", level="trace")]
	pub fn qs_inspect(_: &Object, _: Args) -> Result<Object> {
		Ok(Text::from(Self).into())
	}

	/// Converts this to a [`Boolean`].
	///
	/// # Arguments
	/// None.
	///
	/// # Returns
	/// The [`Boolean`] [`false`](Boolean::FALSE).
	///
	/// # Errors
	/// None.
	///
	/// # Rust Examples
	/// ```rust
	/// use quest_core::types::{Null, Boolean};
	///
	/// assert_eq!(
	/// 	*Null::qs_at_bool(&Null.into(), Default::default()).unwrap()
	/// 		.downcast::<Boolean>().unwrap(),
	/// 	Boolean::new(false)
	/// );
	/// ```
	///
	/// # Quest Examples
	/// ```quest
	/// assert(null.$@bool() == false);
	/// ```
	#[instrument(name="Null::@bool", level="trace")]
	pub fn qs_at_bool(_: &Object, _: Args) -> Result<Object> {
		Ok(Boolean::from(Self).into())
	}

	/// Converts this to a [`List`].
	///
	/// # Arguments
	/// None.
	///
	/// # Returns
	/// An empty [`List`].
	///
	/// # Errors
	/// None.
	///
	/// # Rust Examples
	/// ```rust
	/// use quest_core::types::{Null, List};
	///
	/// assert!(
	/// 	Null::qs_at_list(&Null.into(), Default::default()).unwrap()
	/// 		.downcast::<List>().unwrap()
	/// 		.is_empty()
	/// );
	/// ```
	///
	/// # Quest Examples
	/// ```quest
	/// assert(null.$@list() == []);
	/// ```
	#[instrument(name="Null::@list", level="trace")]
	pub fn qs_at_list(_: &Object, _: Args) -> Result<Object> {
		Ok(List::from(Self).into())
	}

	/// Converts this to a [`Number`].
	///
	/// # Arguments
	/// None.
	///
	/// # Returns
	/// The number [zero](Number::ZERO).
	///
	/// # Errors
	/// None.
	///
	/// # Rust Examples
	/// ```rust
	/// use quest_core::types::{Null, Number};
	///
	/// assert_eq!(
	/// 	*Null::qs_at_num(&Null.into(), Default::default()).unwrap()
	/// 		.downcast::<Number>().unwrap(),
	///	Number::ZERO
	/// );
	/// ```
	///
	/// # Quest Examples
	/// ```quest
	/// assert(null.$@num() == 0);
	/// ```
	#[instrument(name="Null::@num", level="trace")]
	pub fn qs_at_num(_: &Object, _: Args) -> Result<Object> {
		Ok(Number::from(Self).into())
	}

	/// Converts this to a [`Text`].
	///
	/// # Arguments
	/// None.
	///
	/// # Returns
	/// A [`Text`] object containing `"null"`.
	///
	/// # Errors
	/// None.
	///
	/// # Rust Examples
	/// ```rust
	/// use quest_core::types::{Null, Text};
	///
	/// assert_eq!(
	/// 	*Null::qs_at_text(&Null.into(), Default::default()).unwrap()
	/// 		.downcast::<Text>().unwrap(),
	/// 	Text::new("null")
	/// );
	/// ```
	///
	/// # Quest Examples
	/// ```quest
	/// assert(null.$@text() == "null");
	/// ```
	///
	/// # See Also
	/// - [`Null::qs_inspect`] -- Identical to this function.
	#[instrument(name="Null::@text", level="trace")]
	pub fn qs_at_text(_: &Object, _: Args) -> Result<Object> {
		Ok(Text::from(Self).into())
	}

	/// Calls [`Null`], returning [`Null`] regardless of the provided arguments.
	///
	/// # Arguments
	/// None.
	///
	/// # Returns
	/// A [`Null`] regardless of the arguments given.
	///
	/// # Errors
	/// None.
	///
	/// # Rust Examples
	/// ```rust
	/// use quest_core::types::Null;
	///
	/// assert!(
	/// 	Null::qs_call(&Null.into(), Default::default()).unwrap()
	/// 		.downcast::<Null>()
	/// 		.is_some()
	/// );
	/// ```
	///
	/// # Quest Examples
	/// ```quest
	/// assert(null() == null);
	/// ```
	#[instrument(name="Null::()", level="trace")]
	pub fn qs_call(_: &Object, _: Args) -> Result<Object> {
		Ok(Self.into())
	}

	/// Checks to see if the right-hand-side is a [`Null`].
	///
	/// # Arguments
	/// 1. (required) The argument to check to see if it's null.
	///
	/// # Returns
	/// [true](Boolean::TRUE) if the first argument is [`Null`], [false](Boolean::FALSE) otherwise.
	///
	/// # Errors
	/// Returns an [`ArgumentError`] if an argument isn't provided.
	///
	/// # Rust Examples
	/// ```rust
	/// use quest_core::types::{Null, Boolean};
	///
	/// assert_eq!(
	/// 	*Null::qs_eql(&Null.into(), vec![&Null.into()].into()).unwrap()
	/// 		.downcast::<Boolean>().unwrap(),
	/// 	Boolean::new(true)
	/// );
	///
	/// assert_eq!(
	/// 	*Null::qs_eql(&Null.into(), vec![&false.into()].into()).unwrap()
	/// 		.downcast::<Boolean>().unwrap(),
	/// 	Boolean::new(false)
	/// );
	/// ```
	///
	/// # Quest Examples
	/// ```quest
	/// assert(null == null);
	/// assert(null != false);
	/// ```
	#[instrument(name="Null::==", level="trace")]
	pub fn qs_eql(_: &Object, args: Args) -> Result<Object> {
		Ok(args.try_arg(0)?.is_a::<Self>().into())
	}
}

impl_object_type!{
for Null {
	#[inline]
	fn new_object(self) -> Object {
		use lazy_static::lazy_static;
		use crate::types::ObjectType;

		lazy_static! {
			static ref NULL: Object = Object::new_with_parent(Null, vec![Null::mapping()]);
		}

		NULL.deep_clone()
	}
}
[(parents super::Basic) (no_convert)]:
	"@text" => method Self::qs_at_text,
	"inspect" => method Self::qs_inspect,
	"@bool" => method Self::qs_at_bool,
	"@list" => method Self::qs_at_list,
	"@num" => method Self::qs_at_num,
	"()" => method Self::qs_call,
	"==" => method Self::qs_eql,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn display() {
		assert_eq!(Null.to_string(), "null");
	}

	#[test]
	fn from_unit() {
		<Null as crate::types::ObjectType>::initialize().unwrap();

		assert_eq!(Null::from(()), Null);
		Object::from(()).downcast::<Null>().unwrap();
	}

	mod qs {
		use super::*;
		#[test]
		fn at_bool() {
			assert_call_eq!(Null::qs_at_bool(Null) -> Boolean, false);

			assert_call_idempotent!(Null::qs_at_bool(Null));
		}

		#[test]
		fn at_num() {
			assert_call_eq!(Null::qs_at_num(Null) -> Number, 0);

			assert_call_idempotent!(Null::qs_at_num(Null));
		}

		#[test]
		fn at_text() {
			assert_call_eq!(Null::qs_at_text(Null) -> Text, *"null");

			assert_call_idempotent!(Null::qs_at_text(Null));
		}

		#[test]
		fn inspect() {
			assert_call_eq!(Null::qs_inspect(Null) -> Text, *"null");

			assert_call_idempotent!(Null::qs_inspect(Null));
		}

		#[test]
		fn at_list() {
			assert_call!(Null::qs_at_list(Null); |l| List::is_empty(&l));

			assert_call_idempotent!(Null::qs_at_list(Null));
		}

		#[derive(Debug, Clone)]
		struct Dummy;
		impl_object_type! { for Dummy [(parents crate::types::Basic)]: }

		#[test]
		fn call() {
			<Dummy as crate::types::ObjectType>::initialize().unwrap();

			assert_call_eq!(Null::qs_call(Null) -> Null, Null);
			assert_call_eq!(Null::qs_call(Null, Dummy) -> Null, Null);
			assert_call_eq!(Null::qs_call(Null, Dummy, Dummy) -> Null, Null);

			assert_call_idempotent!(Null::qs_call(Null));
		}

		#[test]
		fn eql() {
			<Dummy as crate::types::ObjectType>::initialize().unwrap();

			assert_call_eq!(Null::qs_eql(Null, Dummy) -> Boolean, false);
			assert_call_eq!(Null::qs_eql(Null, Null) -> Boolean, true);
			assert_call_eq!(Null::qs_eql(Null, Null, Dummy) -> Boolean, true);

			assert_call_missing_parameter!(Null::qs_eql(Null), 0);
			assert_call_idempotent!(Null::qs_eql(Null, Null));
			assert_call_idempotent!(Null::qs_eql(Null, Dummy));
		}
	}
}

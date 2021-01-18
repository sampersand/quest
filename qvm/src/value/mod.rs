mod value;
mod float;
mod boolean;
mod smallint;
mod null;
mod builtinfn;
mod allocated;

pub use value::*;
pub use null::*;
pub use float::*;
pub use smallint::*;
pub use boolean::*;
pub use builtinfn::*;
use allocated::Allocated;
pub use allocated::ExternType;
pub use crate::Literal;

/// A Trait that represents the ability for something to have a name.
pub trait NamedType {
	/// The name of this type.
	#[inline(always)]
	fn typename() -> &'static str {
		std::any::type_name::<Self>()
	}
}


pub trait QuestConvertible : ValueType {
	const CONVERT_FUNCTION: Literal;
}

/// A trait representing any value within Quest.
///
/// # Implementing
/// This trait shouldn't need to be implemented directly by types outside of Quest, as the only by-value types in Quest
/// are defined within this crate. Instead, types outside this crate should implement [`QuestObject`]
/// 
/// # Safety
/// The implementor must ensure that:
/// - Every [`into_value()`] produces a unique [`Value`], which no other implementation will return.
/// - [`is_value_a()`] will always return `true` if the value was constructed via [`into_value`] and `false` otherwise.
/// - [`try_value_into()`] must return `Ok(Self)` when the given `value` was constructed via [`Self::into_value()`], and
///   return the original [`Value`] if the value isn't a `Self`.
/// - [`value_into_unchecked()`] must return valid results for any [`Value`] constructed via `Self::into_value`.
///
/// If left unchanged, the default implementation of [`ValueType`] does all this correctly.
pub unsafe trait ValueType : std::fmt::Debug + Sized {
	/// Convert `self` into a [`Value`].
	fn into_value(self) -> Value;

	/// Checks to see if a [`Value`] is a `self`.
	fn is_value_a(value: &Value) -> bool;

	/// Tries to unpack `value` into `Self`, returning `Err(Value)` if the value's not the right type
	///
	/// Implementations generally won't need to override this, as the default behaviour is in terms of
	/// [`is_value_a`] and [`value_into_unchecked`].
	fn try_value_into(value: Value) -> Result<Self, Value> {
		if Self::is_value_a(&value) {
			// SAFETY: we just checked that `value` is a valid `Self`.
			Ok(unsafe { Self::value_into_unchecked(value) })
		} else {
			Err(value)
		}
	}

	/// Converts a `value` to `Self` without checking `value`'s type.
	///
	/// # Safety
	/// The `value` must be a valid `Self`.
	unsafe fn value_into_unchecked(value: Value) -> Self;
}

pub trait HasAttrs {
	/// Checks to see if the value, or one of its parents, has the given attribute.
	///
	/// The default implementation simply checks to see if `get_attr` returns `Some`.
	fn has_attr(&self, attr: Literal) -> bool {
		self.get_attr(attr).is_some()
	}

	/// Returns a reference the value associated with `attr`, defined either on `self` itself, or one of its parents.
	fn get_attr(&self, attr: Literal) -> Option<&Value>;

	/// Returns a mutable reference the value associated with `attr`, defined either on `self` itself, or one of its
	/// parents.
	fn get_attr_mut(&mut self, attr: Literal) -> Option<&mut Value>;

	/// Deletes the given `attr` on `self`, returning the value associated with it, if it existed.
	fn del_attr(&mut self, attr: Literal) -> Option<Value>;

	/// Sets the attribute `attr` for `self` to `value`.
	fn set_attr(&mut self, attr: Literal, value: Value);

	/// Calls the attribute `attr` for `self` with the given `args`.
	fn call_attr(&self, attr: Literal, args: &[&Value]) -> crate::Result<Value> {
		todo!()
		// self.get_attr(attr)
		// 	.expect("todo: return value error")
		// 	.call_attr(Literal::OP_CALL, args)
	}
}

/// A trait representing an immediate value within Quest, ie it has `Copy`.
///
/// # Safety
/// The implementor must ensure that:
/// - [`try_value_copy()`] must return `Some(Self)` when the given `value` was constructed via [`Self::into_value()`],
///   and `None` otherwise.
/// - [`value_copy_unchecked()`] must return valid results for any [`Value`] constructed via `Self::into_value`.
///
/// If left unchanged, the default implementation of [`ValueType`] does all this correctly.
pub unsafe trait ValueTypeImmediate : ValueType + Copy {
	/// Tries to retrieve `Self` from `value`, returning `None` if the value wasn't a `Self`.
	///
	/// Implementations generally won't need to override this, as the default behaviour is in terms of
	/// [`is_value_a`] and [`value_into_unchecked`].
	fn try_value_copy(value: &Value) -> Option<Self> {
		if Self::is_value_a(&value) {
			// SAFETY: we just checked that `value` is a valid `Self`.
			Some(unsafe { Self::value_copy_unchecked(value) })
		} else {
			None
		}
	}

	/// Converts a `value` to `Self` without checking `value`'s type.
	///
	/// # Safety
	/// The `value` must be a valid `Self`.
	unsafe fn value_copy_unchecked(value: &Value) -> Self {
		debug_assert!(Self::is_value_a(&value), "invalid value given to `value_copy_unchecked`: {:?}", value);

		// Destructuring it like this is valid because `value` must be a `Self`, per the contract, and 
		// `Self` must be `Copy`.
		Self::value_into_unchecked(Value::from_bits_unchecked(value.bits()))
	}
}

unsafe impl<T: ValueType + Copy> ValueTypeImmediate for T {}

/// A trait representing a heap-allocated type within Quest.
///
/// # Safety
/// The implementor must ensure that:
/// - [`try_value_as_ref()`] and [`try_value_as_mut()`] must return `Some(&Self)` or `Some(&mut Self)` (respectively)
///   when the given `value` was constructed via [`Self::into_value()`], and `None` otherwise.
/// - [`value_as_ref_unchecked()`] and [`value_as_mut_unchecked()`] must return valid results for any [`Value`]
///   constructed via `Self::into_value`.
///
/// If left unchanged, the default implementation of [`ValueType`] does all this correctly.
pub unsafe trait ValueTypeRef : ValueType {
	/// Tries to convert a reference to a [`Value`] into one for `Self`, returning `None` if the value's not the right
	/// type.
	///
	/// Implementations generally won't need to override this, as the default behaviour is in terms of
	/// [`is_value_a`] and [`value_as_ref_unchecked`].
	fn try_value_as_ref(value: &Value) -> Option<&Self>  {
		if Self::is_value_a(value) {
			// SAFETY: we just checked that `value` is a valid `Self`.
			Some(unsafe { Self::value_as_ref_unchecked(value) })
		} else {
			None
		}
	}

	/// Converts a reference to a [`Value`] into one of `Self` without checking `value`'s type.
	///
	/// # Safety
	/// The `value` must be a valid `Self`.
	unsafe fn value_as_ref_unchecked(value: &Value) -> &Self;

	/// Tries to convert a mutable reference to a [`Value`] into one for `Self`, returning `None` if the value's not the
	/// right type.
	///
	/// Implementations generally won't need to override this, as the default behaviour is in terms of
	/// [`is_value_a`] and [`value_as_mut_unchecked`].
	fn try_value_as_mut(value: &mut Value) -> Option<&mut Self> {
		if Self::is_value_a(value) {
			// SAFETY: we just checked that `value` is a valid `Self`.
			Some(unsafe { Self::value_as_mut_unchecked(value) })
		} else {
			None
		}
	}

	/// Converts a mutable reference to a [`Value`] into one of `Self` without checking `value`'s type.
	///
	/// # Safety
	/// The `value` must be a valid `Self`.
	unsafe fn value_as_mut_unchecked(value: &mut Value) -> &mut Self;
}


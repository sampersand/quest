mod float;
mod boolean;
mod smallint;
mod null;
mod allocated;

pub use null::*;
pub use float::*;
pub use smallint::*;
pub use boolean::*;
pub use allocated::*;

use crate::Literal;
use std::fmt::{self, Debug, Formatter};

/// A type that represents any value in Quest.
// 000...000000 = FALSE (so it can be converted to `false` easily)
// XXX...XXXXX1 = i63
// 000...000010 = TRUE
// 000...000100 = NULL
// XXX...XXX000 = pointer
// 000...X10100 = f32
//
#[repr(transparent)]
pub struct Value(u64);

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
/// If left unchanged, the default implementation of [`QuestValue`] does all this correctly.
pub unsafe trait QuestValue : Debug + Sized {
	/// Convert `self` into a [`Value`].
	fn into_value(self) -> Value {
		Allocated::new(self).into_value()
	}

	/// Checks to see if a [`Value`] is a `self`.
	fn is_value_a(value: &Value) -> bool {
		value.try_as_ref::<Allocated>().map_or(false, Allocated::is_alloc_a::<Self>)
	}

	/// Tries to unpack `value` into `Self`, returning `Err(Value)` if the value's not the right type
	///
	/// Implementations generally won't need to override this, as the default behaviour is in terms of
	/// [`is_value_a`] and [`value_into_unchecked`].
	fn try_value_into(value: Value) -> Result<Self, Value>  {
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
	unsafe fn value_into_unchecked(value: Value) -> Self {
		debug_assert!(Self::is_value_a(&value), "invalid value given to `value_into_unchecked`: {:?}", value);

		Allocated::value_into_unchecked(value).into_unchecked()
	}

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

	fn set_attr(&mut self, attr: Literal, value: Value);

	fn call_attr(&self, attr: Literal, args: &[&Value]) -> crate::Result<Value> {
		self.get_attr(attr).expect("todo: return value error")
			.call_attr(Literal::OP_CALL, args)
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
/// If left unchanged, the default implementation of [`QuestValue`] does all this correctly.
pub unsafe trait QuestValueImmediate : QuestValue + Copy {
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
		Self::value_into_unchecked(Value(value.0))
	}
}

unsafe impl<T: QuestValue + Copy> QuestValueImmediate for T {}

/// A trait representing a heap-allocated type within Quest.
///
/// # Safety
/// The implementor must ensure that:
/// - [`try_value_as_ref()`] and [`try_value_as_mut()`] must return `Some(&Self)` or `Some(&mut Self)` (respectively)
///   when the given `value` was constructed via [`Self::into_value()`], and `None` otherwise.
/// - [`value_as_ref_unchecked()`] and [`value_as_mut_unchecked()`] must return valid results for any [`Value`]
///   constructed via `Self::into_value`.
///
/// If left unchanged, the default implementation of [`QuestValue`] does all this correctly.
pub unsafe trait QuestValueRef : QuestValue {
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

impl Value {
	/// Creates a new [`Value`] for the given built-in type `T`.
	pub fn new<T: QuestValue>(data: T) -> Self {
		data.into_value()
	}

	/// Creates a new [`Value`] for the given `T` by heap allocating it.
	pub fn new_custom<T>(data: T) -> Self {
		Allocated::new(data).into_value()
	}

	/// Get the bits of the [`Value`].
	#[inline]
	const fn bits(&self) -> u64 {
		self.0
	}

	/// Creates a new value from the `bits`.
	///
	/// # Safety
	/// The caller must ensure that `bits` is a valid [`Value`].
	const unsafe fn from_bits_unchecked(bits: u64) -> Self {
		Self(bits)
	}

	/// Checks to see if `self` is a `T`.
	#[inline]
	pub fn is_a<T: QuestValue>(&self) -> bool {
		T::is_value_a(self)
	}

	#[inline]
	pub fn try_into<T: QuestValue>(self) -> Result<T, Self> {
		T::try_value_into(self)
	}

	#[inline]
	pub fn try_copy<T: QuestValueImmediate>(&self) -> Option<T> {
		T::try_value_copy(self)
	}

	#[inline]
	pub fn try_as_ref<T: QuestValueRef>(&self) -> Option<&T> {
		T::try_value_as_ref(self)
	}

	#[inline]
	pub fn try_as_mut<T: QuestValueRef>(&mut self) -> Option<&mut T> {
		T::try_value_as_mut(self)
	}
}

impl Drop for Value {
	fn drop(&mut self) {
		if let Some(alloc_ref_mut) = self.try_as_mut::<Allocated>() {
			// SAFETY: since we're in `drop`, we know we won't be used again, and 
			// we know `Value`s always house valid pointers.
			unsafe {
				std::ptr::drop_in_place(alloc_ref_mut as *mut Allocated)
			}
		}
	}
}

impl Value {
	pub fn try_clone(&self) -> crate::Result<Self> {
		if let Some(alloc) = self.try_as_ref::<Allocated>() {
			alloc.try_clone().map(Self::new)
		} else {
			// SAFETY: this is literally just us rewrapping `self`, so we know it's safe.
			unsafe {
				Ok(Self::from_bits_unchecked(self.bits()))
			}
		}
	}

	pub fn try_eq(&self, rhs: &Self) -> crate::Result<bool> {
		if self.is_a::<Allocated>() && rhs.is_a::<Allocated>() {
			// SAFETY: we literally just checked both of them.
			unsafe {
				Allocated::value_as_ref_unchecked(self).try_eq(Allocated::value_as_ref_unchecked(rhs))
			}
		} else {
			Ok(self.bits() == rhs.bits())
		}
	}
}

impl Debug for Value {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if self.is_a::<Null>() {
			Debug::fmt(&Null, f)
		} else if let Some(boolean) = self.try_copy::<Boolean>() {
			Debug::fmt(&boolean, f)
		} else if let Some(integer) = self.try_copy::<SmallInt>() {
			Debug::fmt(&integer, f)
		} else if let Some(float) = self.try_copy::<Float>() {
			Debug::fmt(&float, f)
		} else if let Some(alloc) = self.try_as_ref::<Allocated>() {
			Debug::fmt(alloc, f)
		} else {
			unreachable!("invalid value given: {:?}", self)
		}
	}
}

enum ValueEnum<'a> {
	Null,
	Boolean(bool),
	SmallInt(SmallInt),
	Float(Float),
	Allocated(*const (), std::marker::PhantomData<&'a ()>)
}

unsafe impl QuestValue for Value {
	#[inline]
	fn into_value(self) -> Value {
		self
	}

	#[inline]
	fn is_value_a(value: &Value) -> bool {
		true
	}

	#[inline]
	fn try_value_into(value: Value) -> Result<Self, Value>  {
		Ok(value)
	}

	unsafe fn value_into_unchecked(value: Value) -> Self {
		value
	}

	fn get_attr(&self, attr: Literal) -> Option<&Value> {
		unsafe {
			match self.0 {
				null::NULL_BITS => Null.get_attr(attr),
				boolean::TRUE_BITS => true.get_attr(attr),
				boolean::FALSE_BITS => false.get_attr(attr),
				_ => todo!()
			}
		}
	}

	fn get_attr_mut(&mut self, attr: Literal) -> Option<&mut Value> {
		todo!()
	}

	fn del_attr(&mut self, attr: Literal) -> Option<Value> {
		todo!()
	}

	fn set_attr(&mut self, attr: Literal, value: Value) {
		todo!()
	}

	fn call_attr(&self, attr: Literal, args: &[&Value]) -> crate::Result<Value> {
		todo!()
	}
}



unsafe impl QuestValueRef for Value {
	#[inline]
	fn try_value_as_ref(value: &Value) -> Option<&Self>  {
		Some(value)
	}

	#[inline]
	unsafe fn value_as_ref_unchecked(value: &Value) -> &Self {
		value
	}

	#[inline]
	fn try_value_as_mut(value: &mut Value) -> Option<&mut Self> {
		Some(value)
	}

	#[inline]
	unsafe fn value_as_mut_unchecked(value: &mut Value) -> &mut Self {
		value
	}
}

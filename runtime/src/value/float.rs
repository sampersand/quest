use super::{Value, ValueInner, Tag};
use std::convert::TryFrom;

pub type Float = f64;

impl Value {
	/// Creates a new floating-point `Value`. All NANs are collapsed into a single NAN type.
	pub fn new_float(float: Float) -> Self {
		if float.is_nan() {
			Self::NAN
		} else {
			Self(ValueInner { float }, std::marker::PhantomData)
		}
	}

	/// Checks to see if `self` is a float.
	pub fn is_float(&self) -> bool {
		self.tag().is_float() || *self == Value::NAN
	}

	/// Converts `self` to a `Float` without checking the tag first.
	///
	/// # Safety
	/// The caller must ensure this is called on a Float value.
	pub unsafe fn as_float_unchecked(&self) -> Float {
		debug_assert!(self.tag().is_float() || *self == Value::NAN, "bad float: {:?}", self);

		if *self == Value::NAN {
			Float::NAN
		} else {
			// SAFETY: The onus is on the caller to make sure `self` is a float.
			unsafe { self.0.float }
		}
	}

	/// Attempts to convert `self` to a float.
	pub fn as_float(&self) -> Option<Float> {
		if self.is_float() {
			// SAFETY: we just verified it was a float, so we can just interpret it as a `f64` safely.
			Some(unsafe { self.as_float_unchecked() })
		} else {
			None
		}
	}
}

impl From<Float> for Value {
	#[inline]
	fn from(float: Float) -> Self {
		Self::new_float(float)
	}
}

impl TryFrom<Value> for Float {
	type Error = Value;
	fn try_from(value: Value) -> Result<Self, Self::Error> {
		value.as_float().ok_or(value)
	}
}

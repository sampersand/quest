use std::mem::ManuallyDrop;
use super::{Value, Tag};
use std::convert::TryFrom;
use quest_core::types::Text;

/// Methods relating to [`Text`].
impl Value {
	/// Creates a new `Text` Value.
	pub fn new_text(text: Text) -> Self {
		let ptr = Box::into_raw(Box::new(text));

		// SAFETY: The pointer is a valid `String` pointer because we just made it. It also has `'static` lifetime.
		unsafe {
			Self::from_ptr(Tag::TEXT, ptr as *const ())
		}
	}

	/// Checks to see if `self` is a piece of text.
	pub fn is_text(&self) -> bool {
		self.tag().0 == Tag::TEXT.0
	}

	/// Assumes `self` is a `Text` and returns a reference to it.
	///
	/// # Safety
	/// The caller is responsible for ensuring that we are, in fact, a `Text`.
	pub unsafe fn as_text_unchecked(&self) -> &Text {
		debug_assert_eq!(self.tag(), Tag::TEXT, "unchecked conversion to Text was incorrect!");

		// SAFETY: The caller ensures that this is valid by the contract with the function.
		unsafe {
			&*(self.as_ptr() as *const _)
		}
	}

	/// Assumes `self` is a `Text` and returns a mutable reference to it.
	///
	/// # Safety
	/// The caller is responsible for ensuring that we are, in fact, a `Text`.
	pub unsafe fn as_text_mut_unchecked(&mut self) -> &mut Text {
		debug_assert_eq!(self.tag(), Tag::TEXT, "unchecked conversion to Text was incorrect!");

		// SAFETY: The caller ensures that this is valid by the contract with the function.
		unsafe {
			&mut *(self.as_mut_ptr() as *mut _)
		}
	}

	/// Assumes `self` is a `Text` and returns the value itself.
	///
	/// # Safety
	/// The caller is responsible for ensuring that we are, in fact, a `Text`.
	pub unsafe fn into_text_unchecked(mut self) -> Text {
		debug_assert_eq!(self.tag(), Tag::TEXT, "unchecked conversion to Text was incorrect!");

		// SAFETY: Assuming we were constructed correctly, we know this pointer must be a `*const Text`
		let text = unsafe {
			*Box::from_raw(self.as_text_mut_unchecked() as *mut _)
		};

		// We need to forget `self` so we don't free the pointer we just extracted.
		ManuallyDrop::new(self);

		text
	}

	/// Attempts to return `self` as a text reference.
	pub fn as_text(&self) -> Option<&Text> {
		if self.is_text() {
			// SAFETY: we just verified that we are a text in the previous line.
			unsafe {
				Some(self.as_text_unchecked())
			}
		} else {
			None
		}
	}

	/// Attempts to return `self` as a mutable text reference.
	pub fn as_text_mut(&mut self) -> Option<&mut Text> {
		if self.is_text() {
			// SAFETY: we just verified that we are a text in the previous line.
			unsafe {
				Some(self.as_text_mut_unchecked())
			}
		} else {
			None
		}
	}

	/// Attempts to convert `self` into a `Text`.
	pub fn into_text(self) -> Result<Text, Self> {
		if self.is_text() {
			// SAFETY: we just verified that we are a text in the previous line.
			unsafe {
				Ok(self.into_text_unchecked())
			}
		} else {
			Err(self)
		}
	}
}

impl From<Text> for Value {
	fn from(text: Text) -> Self {
		Self::new_text(text)
	}
}

impl TryFrom<Value> for Text {
	type Error = Value;
	fn try_from(value: Value) -> Result<Self, Self::Error> {
		value.into_text()		
	}
}

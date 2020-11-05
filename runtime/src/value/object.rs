use std::any::Any;
use std::fmt::Debug;
use std::mem::ManuallyDrop;
use super::{Value, Tag};
use std::collections::HashMap;
use std::convert::TryFrom;


pub trait BoxedClone : Debug + 'static {
	fn clone(&self) -> Box<dyn BoxedClone>;
}

impl<T: 'static + Clone + Debug> BoxedClone for T {
	fn clone(&self) -> Box<dyn BoxedClone> {
		Box::new(self.clone())
	}
} 

#[derive(Debug)]
pub struct Object {
	data: Box<dyn BoxedClone>,
	attrs: HashMap<String, Value>
}

impl Object {
	pub fn new<T: BoxedClone>(data: T) -> Self {
		Self { data: Box::new(data), attrs: Default::default() }
	}
}

impl Clone for Object {
	fn clone(&self) -> Self {
		Self { data: self.data.clone(), attrs: Default::default() }
	}
}

/// Methods relating to [`Object`].
impl Value {
	/// Creates a new `Object` Value.
	pub fn new_object<D: BoxedClone>(data: D) -> Self {
		let ptr = Box::into_raw(Box::new(Object::new(data)));

		// SAFETY: The pointer is a valid `Object` pointer because we just made it. It also has `'static` lifetime.
		unsafe {
			Self::from_ptr(Tag::OBJECT, ptr as *const ())
		}
	}

	/// Checks to see if `self` is an object.
	pub fn is_object(&self) -> bool {
		self.tag() == Tag::OBJECT
	}

	/// Assumes `self` is a `Object` and returns a reference to it.
	///
	/// # Safety
	/// The caller is responsible for ensuring that we are, in fact, a `Object`.
	pub unsafe fn as_object_unchecked(&self) -> &Object {
		debug_assert_eq!(self.tag(), Tag::OBJECT, "unchecked conversion to Object was incorrect!");

		// SAFETY: The caller ensures that this is valid by the contract with the function.
		unsafe {
			&*(self.as_ptr() as *const _)
		}
	}

	/// Assumes `self` is a `Object` and returns a mutable reference to it.
	///
	/// # Safety
	/// The caller is responsible for ensuring that we are, in fact, a `Object`.
	pub unsafe fn as_object_mut_unchecked(&mut self) -> &mut Object {
		debug_assert_eq!(self.tag(), Tag::OBJECT, "unchecked conversion to Object was incorrect!");

		// SAFETY: The caller ensures that this is valid by the contract with the function.
		unsafe {
			&mut *(self.as_mut_ptr() as *mut _)
		}
	}

	/// Assumes `self` is a `Object` and returns the value itself.
	///
	/// # Safety
	/// The caller is responsible for ensuring that we are, in fact, a `Object`.
	pub unsafe fn into_object_unchecked(mut self) -> Object {
		debug_assert_eq!(self.tag(), Tag::OBJECT, "unchecked conversion to Object was incorrect!");

		// SAFETY: Assuming we were constructed correctly, we know this pointer must be a `*const Object`
		let text = unsafe {
			*Box::from_raw(self.as_object_mut_unchecked() as *mut _)
		};

		// We need to forget `self` so we don't free the pointer we just extracted.
		ManuallyDrop::new(self);

		text
	}

	/// Attempts to return `self` as a text reference.
	pub fn as_object(&self) -> Option<&Object> {
		if self.is_text() {
			// SAFETY: we just verified that we are a text in the previous line.
			unsafe {
				Some(self.as_object_unchecked())
			}
		} else {
			None
		}
	}

	/// Attempts to return `self` as a mutable text reference.
	pub fn as_object_mut(&mut self) -> Option<&mut Object> {
		if self.is_text() {
			// SAFETY: we just verified that we are a text in the previous line.
			unsafe {
				Some(self.as_object_mut_unchecked())
			}
		} else {
			None
		}
	}

	/// Attempts to convert `self` into a `Text`.
	pub fn into_object(self) -> Result<Object, Self> {
		if self.is_object() {
			// SAFETY: we just verified that we are a text in the previous line.
			unsafe {
				Ok(self.into_object_unchecked())
			}
		} else {
			Err(self)
		}
	}
}

impl From<Object> for Value {
	fn from(text: Object) -> Self {
		Self::new_object(text)
	}
}

impl TryFrom<Value> for Object {
	type Error = Value;

	fn try_from(value: Value) -> Result<Self, Self::Error> {
		value.into_object()		
	}
}

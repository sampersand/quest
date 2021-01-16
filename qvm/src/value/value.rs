use super::*;
use crate::Literal;
use std::fmt::{self, Debug, Formatter};

/// A type that represents any value in Quest.
// 000...000 0000 = FALSE (so it can be converted to `false` easily)
// XXX...XXX X000 = allocated
// XXX...XXX XXX1 = i63
// 000...000 0010 = TRUE
// 000...XXX X010 = literal (X=32 bits; nonzero.)
// 000...000 0100 = NULL
// XXX...XXX X100 = builtinfn
// 000...XXX X110 = f32 (X=32 bits)
#[repr(transparent)]
pub struct Value(u64);

// impl<T: QuestValue> From<T> for Value {
// 	fn from(questvalue: T) -> Self {
// 		questvalue.into_value()
// 	}
// }

impl Value {
	/// Creates a new [`Value`] for the given built-in type `T`.
	pub fn new<T: QuestValue>(data: T) -> Self {
		data.into_value()
	}

	/// Get the bits of the [`Value`].
	#[inline]
	pub(crate) const fn bits(&self) -> u64 {
		self.0
	}

	/// Creates a new value from the `bits`.
	///
	/// # Safety
	/// The caller must ensure that `bits` is a valid [`Value`].
	pub(crate) const unsafe fn from_bits_unchecked(bits: u64) -> Self {
		Self(bits)
	}

	pub fn id(&self) -> usize {
		self.0 as usize
	}

	pub fn typename(&self) -> &'static str {
		if self.is_a::<Null>() {
			Null::TYPENAME
		} else if self.is_a::<Boolean>() {
			Boolean::TYPENAME
		} else if self.is_a::<SmallInt>() {
			SmallInt::TYPENAME
		} else if self.is_a::<Float>() {
			Float::TYPENAME
		} else if let Some(alloc) = self.downcast::<Allocated>() {
			alloc.typename()
		} else {
			unreachable!("invalid value given: {:?}", self)
		}
	}

	/// Copies the actual data of the object.
	///
	/// When you [`clone()`] a [`Value`], you're actually just creating another reference to the
	/// same object in memory. This actually creates another distinct object.
	pub fn deep_clone(&self) -> Self {
		if let Some(alloc) = self.downcast::<Allocated>() {
			// alloc.deep_clone().into_value()
			todo!()
		} else {
			Self(self.0)
		}
	}

	/// Checks to see if `self` is a `T`.
	#[inline]
	pub fn is_a<T: QuestValue>(&self) -> bool {
		T::is_value_a(self)
	}

	#[inline]
	pub fn downcast<T: QuestValueRef>(&self) -> Option<&T> {
		T::try_value_as_ref(self)
	}

	#[inline]
	pub fn downcast_mut<T: QuestValueRef>(&mut self) -> Option<&mut T> {
		T::try_value_as_mut(self)
	}

	#[inline]
	pub fn downcast_into<T: QuestValue>(self) -> Result<T, Self> {
		T::try_value_into(self)
	}

	#[inline]
	pub fn downcast_copy<T: QuestValueImmediate>(&self) -> Option<T> {
		T::try_value_copy(self)
	}

	#[inline]
	pub fn downcast_call<T: QuestValue>(self) -> crate::Result<T> {
		if self.is_a::<T>() {
			// safety: we just verified it was a `T`.
			unsafe {
				Ok(T::value_into_unchecked(self))
			}
		} else {
			todo!()
		}
	}
}

impl Drop for Value {
	fn drop(&mut self) {
		if let Some(alloc_ref_mut) = self.downcast_mut::<Allocated>() {
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
		if let Some(alloc) = self.downcast::<Allocated>() {
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
		} else if let Some(boolean) = self.downcast_copy::<Boolean>() {
			Debug::fmt(&boolean, f)
		} else if let Some(integer) = self.downcast_copy::<SmallInt>() {
			Debug::fmt(&integer, f)
		} else if let Some(float) = self.downcast_copy::<Float>() {
			Debug::fmt(&float, f)
		} else if let Some(alloc) = self.downcast::<Allocated>() {
			Debug::fmt(alloc, f)
		} else {
			unreachable!("invalid value given: {:?}", self)
		}
	}
}

unsafe impl QuestValue for Value {
	const TYPENAME: &'static str = "qvm::Value";

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
		todo!()
		// if let Some(allocated) = self.downcast::<Allocated>()  {
		// 	allocated.get_attr(attr)
		// } else if let Some(smallint) = self.downcast_copy::<SmallInt>() {
		// 	S
		// }
		// unsafe {
		// 	match self.0 {
		// 		null::NULL_BITS => Null.get_attr(attr),
		// 		boolean::TRUE_BITS => true.get_attr(attr),
		// 		boolean::FALSE_BITS => false.get_attr(attr),
		// 		bits if Number::
		// 		_ => todo!()
		// 	}
		// }
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

impl Clone for Value {
	fn clone(&self) -> Self {
		if let Some(alloc) = self.downcast::<Allocated>() {
			alloc.clone().into_value()
		} else {
			Self(self.0)
		}
	}
}

#[cfg(test)]
mod name {
	use super::*;

	#[derive(Debug, PartialEq, Eq)]
	struct Custom(u64);

	#[test]
	#[ignore]
	fn all_16_bit_reprs_are_valid() {
		crate::literal::initialize();

		for i in 0..0xffffu64 {
			let value = Value(i);


			if i == 0b000 {
				assert_eq!(value.downcast_copy(), Some(Boolean::new(false)));
			} else if i == 0b010 {
				assert_eq!(value.downcast_copy(), Some(Boolean::new(true)));
			} else {
				assert_eq!(value.downcast_copy::<Boolean>(), None);
			}

			if i & 0b111 == 0 && i != 0b000 {
				assert!(value.downcast::<Allocated>().is_some());
			} else {
				assert!(value.downcast::<Allocated>().is_none());
			}

			if i & 1 == 1 {
				assert_eq!(value.downcast_copy(), Some(SmallInt::new(i as i64 >> 1).unwrap()));
			} else {
				assert_eq!(value.downcast_copy::<SmallInt>(), None);
			}

			if i == 0b100 {
				assert_eq!(value.downcast_copy(), Some(Null));
			} else {
				assert_eq!(value.downcast_copy::<Null>(), None);
			}

			let literal = Literal::intern(i.to_string());
			if i & 0b111 == 0b010 && i != 0b010 {
				// SAFETY: we're creating a new `literal` every iteration, so we're guaranteed that `i` will
				// be a valid literal.
				assert_eq!(value.downcast_copy(), Some(unsafe { Literal::from_bits_unchecked(i as u32 >> 3) }));
				// value.downcast_copy::<Literal>().map(|l| l.bits() as u64) == Some(i),
			} else {
				assert_eq!(value.downcast_copy::<Literal>(), None);
			}

			if i & 0b111 == 0b100 && i != 0b100 {
				let builtin = BuiltinFn::new(literal, unsafe { std::mem::transmute::<usize, _>(i as usize >> 3) });
				assert_eq!(value.downcast_copy(), Some(builtin));
			} else {
				assert!(value.downcast_copy::<BuiltinFn>().is_none());
			}

			if i & 0b111 == 0b110 {
				assert_eq!(value.downcast_copy(), Some(f32::from_bits(i as u32 >> 3)));
			} else {
				assert_eq!(value.downcast_copy::<f32>(), None);
			}

			std::mem::forget(value); // in case its allocated.
		}
	}



	#[test]
	fn false_is_zero() {
		let value = Value::new(Boolean::new(false));

		assert_eq!(value.0, 0);
		assert_eq!(value.downcast_copy::<Boolean>(), Some(Boolean::new(false)));
	}

	#[test]
	fn allocated_has_lower_3_bits_zero() {
		#[derive(Debug, PartialEq, Eq)]
		struct Custom(u64);

		impl try_traits::clone::TryClone for Custom {
			type Error = crate::Error;

			fn try_clone(&self) -> crate::Result<Self> {
				Ok(Self(self.0))
			}
		}

		impl QuestObject for Custom {}

		let allocated = Value::new(Custom(123));
		assert_eq!(allocated.0 & 0b111, 0b000);
		// todo: downcast
		// assert_eq!(allocated.downcast::<Custom>(), Some(&Custom(123)));
	}

	#[test]
	fn i63_starts_with_one() {
		let value = Value::new(SmallInt::new(123).unwrap());

		assert_eq!(value.0 & 1, 1);
		assert_eq!(value.downcast_copy::<SmallInt>().unwrap(), SmallInt::new(123).unwrap());
	}

	#[test]
	fn true_is_two() {
		let value = Value::new(Boolean::new(true));

		assert_eq!(value.0, 2);
		assert_eq!(value.downcast_copy::<Boolean>(), Some(Boolean::new(true)));
	}

	#[test]
	fn literal_starts_with_two() {
		let add = Value::new(Literal::OP_ADD);

		assert_eq!(add.0 & 0b111, 0b010);
		assert_eq!(add.downcast_copy::<Literal>(), Some(Literal::OP_ADD));
		assert_eq!(Value::new(Boolean::new(true)).downcast_copy::<Literal>(), None);
	}

	#[test]
	fn null_is_four() {
		let value = Value::new(Null);

		assert_eq!(value.0, 0b100);
		assert_eq!(value.downcast_copy::<Null>(), Some(Null));
	}

	#[test]
	fn builtinfn_starts_with_four() {
		let builtinfn = BuiltinFn::new(Literal::new(concat!(file!(), "-", line!(), "-", column!())), |_, _| panic!());

		let value = Value::new(builtinfn);

		assert_eq!(value.0 & 0b111, 0b100);
		assert_eq!(value.downcast_copy::<BuiltinFn>(), Some(builtinfn));
	}

	#[test]
	fn f32_starts_with_six() {
		let value = Value::new(12.34);

		assert_eq!(value.0 & 0b111, 0b110);
		assert_eq!(value.downcast_copy::<f32>(), Some(12.34));
	}
}



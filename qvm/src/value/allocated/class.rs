use super::{AllocatedType, Allocated};
use crate::value::{Literal, Value, ValueType, NamedType};

/// A Class is really just an instance of an object, but with
/// the data value being `*mut ()`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct Class {
	name: &'static str,
}

impl Class {
	pub fn new(name: &'static str) -> Self {
		Self { name }
	}
}

impl NamedType for Class {
	#[inline(always)]
	fn typename() -> &'static str {
		"Class"
	}
}

unsafe impl AllocatedType for Class {
	fn into_alloc(self) -> Allocated {
		// Allocated { flags: 0, inner: super::allocated::Inner::Class(self) }
		todo!()
	}

	fn is_alloc_a(alloc: &Allocated) -> bool {
		todo!()
	/*
		Object::try_alloc_as_ref(alloc).map_or(false, Object::is_a::<Self>)
	*/}

	unsafe fn alloc_into_unchecked(alloc: Allocated) -> Self {
		todo!()
	/*
		debug_assert!(Self::is_alloc_a(&alloc), "invalid value given: {:#?}", alloc);
		
		Object::alloc_into_unchecked(alloc).into_unchecked()
	*/}

	unsafe fn alloc_as_ref_unchecked(alloc: &Allocated) -> &Self {
		todo!()
	/*
		debug_assert!(Self::is_alloc_a(alloc), "invalid value given: {:#?}", alloc);
		
		Object::alloc_as_ref_unchecked(alloc).as_ref_unchecked()
	*/}

	unsafe fn alloc_as_mut_unchecked(alloc: &mut Allocated) -> &mut Self {
		todo!()
	/*
		debug_assert!(Self::is_alloc_a(alloc), "invalid value given: {:#?}", alloc);
		
		Object::alloc_as_mut_unchecked(alloc).as_mut_unchecked()
	*/}
}

/*
unsafe impl ValueType for Class {
	const TYPENAME: &'static str = "<TODO>";

	fn into_value(self) -> Value {
		// Allocated::new(self).into_value()
		todo!()
	}

	fn is_value_a(value: &Value) -> bool {
		todo!()
		// value.downcast::<Allocated>().map_or(false, Allocated::is_alloc_a::<Self>)
	}

	unsafe fn value_into_unchecked(value: Value) -> Self {
		todo!()
		// debug_assert!(Self::is_value_a(&value), "invalid value given to `value_into_unchecked`: {:?}", value);

		// Allocated::value_into_unchecked(value).into_unchecked()
	}

	fn has_attr(&self, _attr: Literal) -> bool { todo!() }
	fn get_attr(&self, _attr: Literal) -> Option<&Value> { todo!() }
	fn get_attr_mut(&mut self, _attr: Literal) -> Option<&mut Value> { todo!() }
	fn del_attr(&mut self, _attr: Literal) -> Option<Value> { todo!() }
	fn set_attr(&mut self, _attr: Literal, _value: Value) { todo!() }
	fn call_attr(&self, _attr: Literal, _args: &[&Value]) -> crate::Result<Value> { todo!() }
}
*/

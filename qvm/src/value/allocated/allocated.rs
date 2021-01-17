use super::*;
use crate::Literal;
use crate::value::{Value, QuestValue, QuestValueRef};
use std::fmt::{self, Debug, Display, Formatter};
use std::mem::ManuallyDrop;

#[repr(C, align(8))]
pub struct Allocated {
	flags: u64,
	data: Data
}

#[repr(C, align(8))]
union Data {
	text: (),
	bignum: (),
	regex: (),
	list: ManuallyDrop<List>,
	map: (),
	class: ManuallyDrop<Class>,
	extern_data: ManuallyDrop<ExternData>,
}

const FLAG_INSTANCE_MASK: u64 =   0b00111111;
const FLAG_INSTANCE_OBJECT: u64 = 0b00000000;
const FLAG_INSTANCE_BIGNUM: u64 = 0b00000001;
const FLAG_INSTANCE_REGEX: u64 =  0b00000010;
const FLAG_INSTANCE_LIST: u64 =   0b00000011;
const FLAG_INSTANCE_MAP: u64 =    0b00000100;
const FLAG_INSTANCE_TEXT: u64 =   0b00000101;
const FLAG_INSTANCE_CLASS: u64 =  0b00000110;
const FLAG_INSTANCE_CUSTOM: u64 = 0b00000111;

macro_rules! impl_from {
	($($flag:ident $data:ty),*) => {
		$(
			impl From<$data> for Allocated {
				fn from(data: $data) -> Self {
					Self {
						flags: $flag,

					}
				}
			}
		)*
	};
}

impl From<List> for Allocated {
	fn from(list: List) -> Self {
		Self {
			flags: FLAG_INSTANCE_LIST,
			data: Data { list: ManuallyDrop::new(list) }
		}
	}
}

impl From<ExternData> for Allocated {
	fn from(extern_data: ExternData) -> Self {
		Self {
			flags: FLAG_INSTANCE_OBJECT,
			data: Data { extern_data: ManuallyDrop::new(extern_data) }
		}
	}
}


// TODO: allocate pages, and use those, instead of allocating individual pointers.
impl Allocated {
	pub fn new<T: AllocatedType>(data: T) -> Self {
		data.into_alloc()
	}

	pub fn into_ptr(self) -> *mut () {
		Box::into_raw(Box::new(self)) as *mut ()
	}

	pub fn is_alloc_a<T>(&self) -> bool {
		// T::is_alloc_a(self )
		// self.inner().
		false
	}

	pub unsafe fn from_ptr_ref<'a>(pointer: *const ()) -> &'a Self {
		&*(pointer as *const Self)
	}

	pub unsafe fn from_ptr_mut<'a>(pointer: *mut ()) -> &'a mut Self {
		&mut *(pointer as *mut Self)
	}

	pub unsafe fn from_ptr(ptr: *mut ()) -> Self {
		*Box::from_raw(ptr as *mut Self)
	}

	pub unsafe fn into_unchecked<T>(self) -> T {
		todo!()
	}

	pub fn typename(&self) -> &'static str {
		todo!()
	}
}

const ALLOC_TAG: u64   = 0b0000;
const ALLOC_MASK: u64  = 0b0111;
const ALLOC_SHIFT: u64 = 0;

unsafe impl QuestValue for Allocated {
	const TYPENAME: &'static str = "qvm::Allocated";

	fn into_value(self) -> Value {
		// SAFETY: This is the definition of a valid pointer.
		unsafe {
			Value::from_bits_unchecked(((self.into_ptr() as u64) << ALLOC_SHIFT) | ALLOC_TAG)
		}
	}

	fn is_value_a(value: &Value) -> bool {
		value.bits() != 0 && (value.bits() & ALLOC_MASK) == ALLOC_TAG
	}

	unsafe fn value_into_unchecked(value: Value) -> Self {
		debug_assert!(value.is_a::<Self>());

		Self::from_ptr(value.bits() as *mut ())
	}


	fn get_attr(&self, attr: Literal) -> Option<&Value> {
		todo!()
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
}

unsafe impl QuestValueRef for Allocated {
	unsafe fn value_as_ref_unchecked(value: &Value) -> &Self {
		debug_assert!(value.is_a::<Self>());

		Self::from_ptr_ref(value.bits() as *const ())
	}

	unsafe fn value_as_mut_unchecked(value: &mut Value) -> &mut Self {
		debug_assert!(value.is_a::<Self>());

		Self::from_ptr_mut(value.bits() as *mut ())
	}
}

impl Display for Allocated {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		// std::fmt::Debug::fmt(self, f)
		todo!()
	}
}

impl Debug for Allocated {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		// std::fmt::Debug::fmt(self, f)
		todo!()
	}
}

impl Allocated {
	pub fn try_clone(&self) -> crate::Result<Self> {
		todo!()
	}

	pub fn try_eq(&self, rhs: &Self) -> crate::Result<bool> {
		todo!()
	}
}

impl Drop for Allocated {
	fn drop(&mut self) {
		todo!();
	}
}

impl Clone for Allocated {
	fn clone(&self) -> Self {
		todo!()
	}
}

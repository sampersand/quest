use std::any::{Any, TypeId, type_name};
use std::fmt::{self, Debug, Formatter};
use crate::shared_cow::SharedCow;
use crate::types;

type AnyObj = dyn Any + Send + Sync;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub union DataType {
	null: types::Null,
	number: types::Number,
	boolean: types::Boolean,
	class: types::Class,
	any: *const dyn DataType
}

pub trait ConvertToDataType : Sized + 'static {
	fn into_datatype(self) -> DataType;
}

impl DataType {
	#[inline]
	pub fn any<T: Clone + Debug + Send + Sync + 'static>(data: T) -> Self {
		Self::Any(AnyData::new(data))
	}
}

impl AnyData {
	#[inline]
	fn new<T: Any + Send + Sync + Clone + Debug>(data: T) -> Self {
		Self {
			dbg: |x, f| match x.downcast_ref::<T>() {
				Some(val) => T::fmt(val, f),
				None => unreachable!("bad debug (expected {}): {:?}", type_name::<T>(), x),
			},
			clone: |x| match x.downcast_ref::<T>() {
				Some(val) => Box::new(T::clone(val)),
				None => unreachable!("bad clone (expected {}): {:?}", type_name::<T>(), x),
			},
			data: Box::new(data) as _
		}
	}
}

#[doc(hidden)]
pub struct AnyData {
	dbg: fn(&dyn Any, &mut Formatter) -> fmt::Result,
	clone: fn(&dyn Any) -> Box<AnyObj>,
	data: Box<AnyObj>
}

impl Clone for AnyData {
	#[inline]
	fn clone(&self) -> Self {
		Self {
			dbg: self.dbg,
			clone: self.clone,
			data: (self.clone)(self.data.as_ref())
		}
	}
}

impl Debug for AnyData {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		(self.dbg)(self.data.as_ref(), f)
	}
}

#[derive(Clone)]
pub struct Data {
	data: SharedCow<DataType>,
	typeid: TypeId,
	typename: &'static str
}

impl Data {
	pub fn new<T: ConvertToDataType + 'static>(data: T) -> Self {
		Self {
			data: SharedCow::new(data.into_datatype()),
			typeid: TypeId::of::<T>(),
			typename: type_name::<T>()
		}
	}

	#[inline]
	pub fn typename(&self) -> &'static str {
		self.typename
	}

	#[inline]
	pub fn is_a<T: Any>(&self) -> bool {
		self.typeid == TypeId::of::<T>()
	}

	pub fn downcast_and_then<T: Any, R, F: FnOnce(&T) -> R>(&self, f: F) -> Option<R> {
		if self.is_a::<T>() {
			unsafe {
				Some(self.downcast_unchecked_and_then(f))
			}
		} else { 
			None
		}
	}

	pub unsafe fn downcast_unchecked_and_then<T: Any, R, F: FnOnce(&T) -> R>(&self, f: F) -> R {
		use std::mem::transmute;

		debug_assert!(self.is_a::<T>(), "BUG: invalid downcast to {} from {}", 
			type_name::<T>(), self.typename);

		self.data.with_ref(|data|
			f(match data {
				DataType::Null(ref null) => transmute(null),
				DataType::Number(ref num) => transmute(num),
				DataType::Boolean(ref bool) => transmute(bool),
				DataType::Class(ref class) => transmute(class),
				DataType::Any(ref any) => any.data.downcast_ref().expect("invalid any encountered")
			})
		)
	}

	pub fn downcast_mut_and_then<T: Any, R, F: FnOnce(&mut T) -> R>(&self, f: F) -> Option<R> {
		if self.is_a::<T>() {
			unsafe {
				Some(self.downcast_mut_unchecked_and_then(f))
			}
		} else { 
			None
		}
	}

	pub unsafe fn downcast_mut_unchecked_and_then<T: Any, R, F: FnOnce(&mut T) -> R>(&self, f: F)
		-> R
	{
		use std::mem::transmute;

		debug_assert!(self.is_a::<T>(), "BUG: invalid downcast to {} from {}", 
			type_name::<T>(), self.typename);

		self.data.with_mut(|data|
			f(match data {
				DataType::Null(ref mut null) => transmute(null),
				DataType::Number(ref mut num) => transmute(num),
				DataType::Boolean(ref mut bool) => transmute(bool),
				DataType::Class(ref mut class) => transmute(class),
				DataType::Any(ref mut any) => any.data.downcast_mut().expect("invalid any encountered")
			}) 
		)
	}
}


impl Debug for Data {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			f.debug_struct("Data")
				.field("data", &self.data)
				.field("typename", &self.typename)
				.finish()
		} else {
			Debug::fmt(&self.data, f)
		}
	}
}


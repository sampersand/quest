use std::any::{Any, type_name};
use std::sync::Arc;
use std::fmt::{self, Debug, Formatter};
use std::borrow::{Borrow, BorrowMut};
use crate::shared_cow::{SharedCow, Sharable};

type AnyObj = dyn Any + Send + Sync;

pub trait ConvertToDataType : Send + Sync + Clone + Debug + 'static {
	fn into_datatype(self) -> DataType;
}
pub enum DataType {}

pub(crate) struct OwnedAny<T: AsRef<AnyObj>> {
	dbg: fn(&dyn Any, &mut Formatter) -> fmt::Result,
	clone: fn(&dyn Any) -> Box<AnyObj>,
	data: T
}

impl Borrow<AnyObj> for OwnedAny<Box<AnyObj>> {
	#[inline]
	fn borrow(&self) -> &AnyObj {
		self.data.borrow()
	}
}

impl BorrowMut<AnyObj> for OwnedAny<Box<AnyObj>> {
	#[inline]
	fn borrow_mut(&mut self) -> &mut AnyObj {
		self.data.borrow_mut()
	}
}

impl Borrow<AnyObj> for OwnedAny<Arc<AnyObj>> {
	#[inline]
	fn borrow(&self) -> &AnyObj {
		self.data.borrow()
	}
}

impl Clone for OwnedAny<Arc<AnyObj>> {
	#[inline]
	fn clone(&self) -> Self {
		Self { dbg: self.dbg, clone: self.clone, data: self.data.clone() }
	}
}

impl<T: AsRef<AnyObj>> Debug for OwnedAny<T> {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		(self.dbg)(self.data.as_ref(), f)
	}
}

impl Sharable for AnyObj {
	type Owned = OwnedAny<Box<Self>>;
	type Shared = OwnedAny<Arc<Self>>;
	
	#[inline]
	fn to_shared(OwnedAny { dbg, clone, data }: Self::Owned) -> Self::Shared {
		OwnedAny { data: data.into(), dbg, clone }
	}

	#[inline]
	fn to_owned(OwnedAny { dbg, clone, data }: &Self::Shared) -> Self::Owned {
		OwnedAny { data: (*clone)(data.as_ref()), dbg: *dbg, clone: *clone }
	}
}

#[derive(Clone)]
pub struct Data {
	data: SharedCow<AnyObj>,
	typename: &'static str
}

impl Data {
	pub fn new<T: Any + Debug + Send + Sync + Clone>(data: T) -> Self {
		Self {
			data: SharedCow::new(
				OwnedAny {
					dbg: |x, f| match x.downcast_ref::<T>() {
						Some(val) => T::fmt(val, f),
						None => unreachable!("bad x given to debug"),
					},
					clone: |x| match x.downcast_ref::<T>() {
						Some(val) => Box::new(T::clone(val)),
						None => unreachable!("bad x given to clone"),
					},
					data: Box::new(data) as _
				}
			),
			typename: type_name::<T>()
		}
	}

	#[inline]
	pub fn typename(&self) -> &'static str {
		self.typename
	}

	#[inline]
	pub fn is_a<T: Any>(&self) -> bool {
		self.data.with_ref(|data| data.is::<T>())
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
		self.data.with_ref(|any| match any.downcast_ref() {
			Some(val) => f(val),
			None => unreachable!("invalid downcast encountered")
		})
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

	pub unsafe fn downcast_mut_unchecked_and_then<T: Any, R, F: FnOnce(&mut T) -> R>(&self, f: F) -> R {
		self.data.with_mut(|any| match any.downcast_mut() {
			Some(val) => f(val),
			None => unreachable!("invalid downcast encountered")
		})
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


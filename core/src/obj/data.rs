use std::any::{Any, type_name};
use std::sync::Arc;
use std::fmt::{self, Debug, Formatter};
use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;
use crate::shared_cow::{SharedCow, Sharable};

type AnyObj = dyn Any + Send + Sync;

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

/// The data associated with an [`Object`](crate::Object).
#[derive(Clone)]
pub struct Data {
	data: SharedCow<AnyObj>,
	typename: &'static str,
}

struct DowncastWrapper<T, D>(D, PhantomData<T>);

impl<T: 'static, D: Deref<Target=AnyObj>> Deref for DowncastWrapper<T, D> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		self.0.downcast_ref().expect("bad downcast")
	}
}

impl<T: 'static, D: DerefMut<Target=AnyObj>> DerefMut for DowncastWrapper<T, D> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.0.downcast_mut().expect("bad downcast")
	}
}


impl Data {
	/// Create a new [`Data`] initialized with the associated `data`.
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

	/// Gets the name of this type. Used when debugging.
	#[inline]
	pub fn typename(&self) -> &'static str {
		self.typename
	}

	/// Checks to see if the contained data is a `T`.
	#[inline]
	pub fn is_a<T: Any>(&self) -> bool {
		self.data.read().is::<T>()
	}

	/// Tries to downcast the contained data to a `T`, returning `None` if it's not a `T`.
	pub fn downcast<'a, T: Any>(&'a self) -> Option<impl Deref<Target=T> + 'a> {
		let data = self.data.read();

		if data.is::<T>() {
			Some(DowncastWrapper(data, PhantomData))
		} else {
			None
		}
	}

	/// Tries to mutably downcast the contained data to a `T`, returning `None` if it's not a `T`.
	pub fn downcast_mut<'a, T: Any>(&'a self) -> Option<impl DerefMut<Target=T> + 'a> {
		let data = self.data.write();

		if data.is::<T>() {
			Some(DowncastWrapper(data, PhantomData))
		} else {
			None
		}
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

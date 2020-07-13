#![allow(unused)]
use std::any::{Any, type_name};
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
enum Ownership {
	Owned(Box<dyn Any + Send + Sync>),
	Shared(Arc<dyn Any + Send + Sync>)
}

struct Details {
	dbg: fn(&dyn Any, &mut Formatter) -> fmt::Result,
	clone: fn(&dyn Any) -> Box<dyn Any + Send + Sync>,
	typename: &'static str
}

pub struct Data {
	data: RwLock<Option<Ownership>>,
	details: Arc<Details>
}

impl Clone for Data {
	fn clone(&self) -> Self {
		let mut data = self.data.write().unwrap();

		*data = 
			Some(match data.take() {
				Some(Ownership::Owned(owned)) => Ownership::Shared(Arc::from(owned)),
				Some(Ownership::Shared(shared)) => Ownership::Shared(shared),
				None => unreachable!()
			});

		match data.as_ref().unwrap() {
			// we literally just swapped it away from owned
			Ownership::Owned(_) => unreachable_debug_or_unchecked!(),
			Ownership::Shared(ref data) => Data {
				data: RwLock::new(Some(Ownership::Shared(data.clone()))),
				details: self.details.clone()
			}
		}
	}
}

impl Data {
	pub fn new<T: Any + Debug + Send + Sync + Clone>(data: T) -> Self {
		// println!("{:?} {:?}", std::any::type_name::<T>(), data);
		Data {
			data: RwLock::new(Some(Ownership::Owned(Box::new(data)))),
			details: Arc::new(Details {
				dbg: |x, f| T::fmt(x.downcast_ref::<T>().expect("bad val given to debug"), f),
				clone: |x| Box::new(T::clone(x.downcast_ref::<T>().expect("bad val given to clone"))),
				typename: type_name::<T>(),
			})
		}
	}

	#[inline]
	pub fn typename(&self) -> &'static str {
		self.details.typename
	}

	#[inline]
	pub fn is_a<T: Any>(&self) -> bool {
		match self.data.read().unwrap().as_ref().unwrap() {
			Ownership::Owned(ref data) => data.is::<T>(),
			Ownership::Shared(ref data) => data.is::<T>()
		}
	}

	#[inline]
	pub fn downcast_and_then<T: Any, R, F: FnOnce(Option<&T>) -> R>(&self, f: F) -> R {
		if self.is_a::<T>() {
			unsafe { self.downcast_unchecked_and_then(|x| f(Some(x))) }
		} else { 
			f(None)
		}
	}

	#[inline]
	pub unsafe fn downcast_unchecked_and_then<T: Any, R, F: FnOnce(&T) -> R>(&self, f: F) -> R {
		#[allow(deprecated)]
		f(&*self.downcast_ref_unchecked::<T>())
	}

	#[inline]
	pub fn downcast_mut_and_then<T: Any, R, F: FnOnce(Option<&mut T>) -> R>(&self, f: F) -> R {
		if self.is_a::<T>() {
			unsafe { self.downcast_mut_unchecked_and_then(|x| f(Some(x))) }
		} else { 
			f(None)
		}
	}

	#[inline]
	pub unsafe fn downcast_mut_unchecked_and_then<T: Any, R, F: FnOnce(&mut T) -> R>(&self, f: F) -> R {
		#[allow(deprecated)]
		f(&mut *self.downcast_mut_unchecked::<T>())
	}

	#[inline]
	#[deprecated]
	unsafe fn downcast_ref_unchecked<'a, T: Any>(&'a self) -> impl Deref<Target=T> + 'a {
		use std::sync::RwLockReadGuard;
		use std::marker::PhantomData;

		struct Caster<'a, T>(RwLockReadGuard<'a, Option<Ownership>>, PhantomData<T>);
		impl<'a, T: 'static> Deref for Caster<'a, T> {
			type Target = T;
			fn deref(&self) -> &T {
				match self.0.as_ref().unwrap() {
					Ownership::Owned(ref data) => data.downcast_ref().unwrap(),
					Ownership::Shared(ref data) => data.downcast_ref().unwrap(),
				}
			}
		}

		debug_assert!(self.is_a::<T>(), "internal error: cannot downcast from {} to {}",
			self.typename(), type_name::<T>());

		Caster::<'a, T>(self.data.read().expect("poison error"), PhantomData)
	}

	#[inline]
	#[deprecated]
	unsafe fn downcast_mut_unchecked<'a, T: Any>(&'a self) -> impl DerefMut<Target=T> + 'a {
		use std::{sync::RwLockWriteGuard, marker::PhantomData};

		struct Caster<'a, T>(RwLockWriteGuard<'a, Option<Ownership>>, PhantomData<T>, &'a Details);
		impl<'a, T: 'static> Deref for Caster<'a, T> {
			type Target = T;
			fn deref(&self) -> &T {
				match self.0.as_ref().unwrap() {
					Ownership::Owned(ref data) => data.downcast_ref().unwrap(),
					Ownership::Shared(ref data) => data.downcast_ref().unwrap(),
				}
			}
		}

		impl<'a, T: 'static> DerefMut for Caster<'a, T> {
			fn deref_mut(&mut self) -> &mut T {
				if let Ownership::Shared(ref shared) = self.0.as_ref().unwrap() {
					*self.0 = Some(Ownership::Owned((self.2.clone)(&**shared)))
				}

				match self.0.as_mut().unwrap() {
					Ownership::Owned(ref mut owned) => owned.downcast_mut().unwrap(),
					Ownership::Shared(_) => unreachable_debug_or_unchecked!()
				}
			}
		}

		debug_assert!(self.is_a::<T>(), "internal error: cannot downcast from {} to {}",
			self.typename(), type_name::<T>());

		Caster::<'a, T>(self.data.write().expect("poison error"), PhantomData, &self.details)
	}
}


impl Debug for Data {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		struct DataDebug<'a>(&'a (dyn Any + Send + Sync), fn(&dyn Any, &mut Formatter) -> fmt::Result);

		impl Debug for DataDebug<'_> {
			fn fmt(&self, f: &mut Formatter) -> fmt::Result {
				(self.1)(self.0, f)
			}
		}

		let lock = self.data.read().unwrap();
		let any_ref = match lock.as_ref().unwrap() {
			Ownership::Owned(ref data) => data.as_ref(),
			Ownership::Shared(ref data) => data.as_ref()
		};


		let data_dbg = DataDebug(any_ref, self.details.dbg);

		if f.alternate() {
			f.debug_struct("Data")
				.field("data", &data_dbg)
				.field("typename", &self.details.typename)
				.finish()
		} else {
			Debug::fmt(&data_dbg, f)
		}
	}
}


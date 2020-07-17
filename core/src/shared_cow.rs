use parking_lot::RwLock;
use std::sync::Arc;
use std::fmt::{self, Debug, Formatter};

#[derive(Debug)]
enum Data<T> {
	Owned(T),
	Shared(Arc<T>)
}

pub struct SharedCow<T>(RwLock<Data<T>>);

impl<T: Debug> Debug for SharedCow<T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_tuple("SharedCow")
			.field(&self.0.read())
			.finish()
	}
}

impl<T: Default> Default for SharedCow<T> {
	#[inline]
	fn default() -> Self {
		Self::new(T::default())
	}
}

// TODO: it is possible to remove the `T: clone` bound here, and I probably should...
impl<T: Clone> Clone for SharedCow<T> {
	fn clone(&self) -> Self {
		// if we're cloning a shared resource, then that's easy: just copy our arc.
		if let Data::Shared(shared) = &*self.0.read() {
			return Self::new_shared(shared.clone())
		}

		// otherwise, we need to convert ourselves into an owned version.
		let mut data = self.0.write();

		match *data {
			// if someone already did our job for us before we got the lock, return the arc.
			Data::Shared(ref shared) => Self::new_shared(shared.clone()),
			Data::Owned(ref owned) => {
				let shared = Arc::new(owned.clone());
				*data = Data::Shared(shared.clone());
				Self::new_shared(shared)
			}
		}
	}
}

impl<T> SharedCow<T> {
	#[inline]
	pub fn new_shared(data: Arc<T>) -> Self {
		Self::from_data(Data::Shared(data))
	}

	#[inline]
	pub fn new(data: T) -> Self {
		Self::from_data(Data::Owned(data))
	}

	#[inline]
	fn from_data(data: Data<T>) -> Self {
		Self(RwLock::new(data))
	}

	pub fn downcast_and_then<F: FnOnce(&T) -> R, R>(&self, func: F) -> R {
		if let Data::Shared(ref arc) = *self.0.read() {
			return func(arc)
		}

		let data = self.0.write();

		// we have to check again in case something created a shared reference
		// before we acquired the data
		match *data {
			Data::Shared(ref arc) => func(arc),
			Data::Owned(ref owned) => func(owned)
		}
	}
}

impl<T: Clone> SharedCow<T> {
	pub fn downcast_mut_and_then<F: FnOnce(&mut T) -> R, R>(&self, func: F) -> R {
		// we have to lock regardless, because we will be accessing `Owned`.
		let mut data = self.0.write();

		if let Data::Shared(ref shared) = *data {
			*data = Data::Owned(T::clone(shared));
		}

		match *data {
			Data::Owned(ref mut owned) => func(owned),
			Data::Shared(_) => unreachable!("we just ensured we were owned")
		}
	}
}

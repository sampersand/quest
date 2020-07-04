use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct SharedCow<T: Clone, >(RwLock<Ownership<T>>);

// False positive, this isn't an implementation of the non-exhaustive pattern.
#[allow(clippy::manual_non_exhaustive)]
#[derive(Debug, Clone)]
enum Ownership<T> {
	Owned(T),
	Shared(Arc<T>),
	#[doc(hidden)]
	__Cloning
}

impl<T: Clone + Default> Default for SharedCow<T> {
	#[inline]
	fn default() -> Self {
		SharedCow::new(T::default())
	}
}


impl<T: Clone> From<T> for SharedCow<T> {
	#[inline]
	fn from(data: T) -> Self {
		SharedCow::from_inner(Ownership::Owned(data))
	}
}


impl<T: Clone> From<Arc<T>> for SharedCow<T> {
	#[inline]
	fn from(data: Arc<T>) -> Self {
		SharedCow::from_inner(Ownership::Shared(data))
	}
}

impl<T: Clone> Clone for SharedCow<T> {
	#[inline]
	fn clone(&self) -> Self {
		Self::from_inner(self.clone_data())
	}
}

impl<T: Clone> SharedCow<T> {
	#[inline]
	pub fn new(data: T) -> Self {
		Self::from_inner(Ownership::Owned(data))
	}

	#[inline]
	fn from_inner(inner: Ownership<T>) -> Self {
		SharedCow(RwLock::new(inner))
	}

	fn clone_data(&self) -> Ownership<T> {
		let data = self.0.read().expect("couldn't clone data");

		if let shared @ Ownership::Shared(..) = &*data {
			return shared.clone()
		}

		drop(data);
		let mut data = self.0.write().expect("couldn't write");

		match std::mem::replace(&mut *data, Ownership::__Cloning) {
			Ownership::Owned(owned) => {
				*data = Ownership::Shared(Arc::new(owned));

				data.clone()
			},
			shared @ Ownership::Shared(..) => shared,
			Ownership::__Cloning => unreachable!()
		}
	}

	// pub fn into_inner(self) -> T {
	// 	match self.0.into_inner().unwrap() {
	// 		Ownership::Owned(owned) => owned,
	// 		Ownership::Shared(shared) => shared.as_ref().clone(),
	// 		Ownership::__Cloning => unreachable!()
	// 	}
	// }

	pub fn with_ref<F: FnOnce(&T) -> R, R>(&self, func: F) -> R {
		match &*self.0.read().expect("couldn't read") {
			Ownership::Owned(owned) => func(&owned),
			Ownership::Shared(shared) => func(&shared),
			Ownership::__Cloning => unreachable!()
		}
	}

	pub fn with_mut<F: FnOnce(&mut T) -> R, R>(&self, func: F) -> R {
		let mut data = self.0.write().expect("couldn't write");

			// println!("becoming owned");
		if let Ownership::Shared(shared) = &*data {
			*data = Ownership::Owned(shared.as_ref().clone());
		}

		if let Ownership::Owned(owned) = &mut *data {
			func(owned)
		} else {
			unreachable!()
		}
	}
}

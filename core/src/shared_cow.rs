use std::sync::Arc;
use std::fmt::{self, Debug, Formatter};
use parking_lot::RwLock;

pub struct SharedCow<T>(RwLock<Data<T>>);

#[derive(Debug)]
enum Data<T> {
	Owned(T),
	Shared(Arc<T>)
}

impl<T: Debug> Debug for SharedCow<T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_tuple("SharedCow")
			.field(&*self.0.read())
			.finish()
	}
}

impl<T: Default> Default for SharedCow<T> {
	#[inline]
	fn default() -> Self {
		Self::new(T::default())
	}
}

impl<T> Clone for SharedCow<T> {
	fn clone(&self) -> Self {
		// if we're cloning a shared resource, then that's easy: just copy our arc.
		if let Data::Shared(ref shared) = *self.0.read() {
			return Self::new_shared(shared.clone())
		}

		// otherwise, we need to convert ourselves into an owned version.
		let mut ret: Option<Self> = None;

		take_mut::take(&mut *self.0.write(), |lock| {
			let shared =
				match lock {
					Data::Shared(shared) => shared,
					Data::Owned(owned) => Arc::new(owned)
				};
			ret = Some(Self::new_shared(shared.clone()));
			Data::Shared(shared)
		});

		match ret {
			Some(clone) => clone,
			None => unsafe { unreachable_debug_or_unchecked!() }
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
		SharedCow(RwLock::from(data))
	}

	pub fn with_ref<F: FnOnce(&T) -> R, R>(&self, func: F) -> R {
		match *self.0.read() {
			Data::Shared(ref shared) => func(shared),
			Data::Owned(ref owned) => func(owned)
		}
	}
}

impl<T: Clone> SharedCow<T> {
	pub fn with_mut<F: FnOnce(&mut T) -> R, R>(&self, func: F) -> R {
		let mut lock = self.0.write();

		match *lock {
			Data::Owned(ref mut owned) => func(owned),
			Data::Shared(ref shared) => {
				let mut owned = T::clone(&shared);
				let ret = func(&mut owned);
				*lock = Data::Owned(owned);
				ret
			}
		}
	}
}

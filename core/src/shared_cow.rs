use std::sync::{Arc, RwLock};
use std::cell::UnsafeCell;
use std::fmt::{self, Debug, Formatter};

#[derive(Debug)]
enum Data<T> {
	Owned(T),
	Shared(Arc<T>)
}

pub struct SharedCow<T> {
	own_lock: RwLock<()>,
	data: UnsafeCell<Data<T>>,
}

unsafe impl<T: Send + Sync> Sync for SharedCow<T> {}

impl<T: Debug> Debug for SharedCow<T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_tuple("SharedCow")
			.field(&self.data.get())
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
		let data_ptr = self.data.get();
		
		// if we're cloning a shared resource, then that's easy: just copy our arc.
		if let Data::Shared(shared) = unsafe { &*data_ptr } {
			return Self::new_shared(shared.clone())
		}

		// otherwise, we need to convert ourselves into an owned version.
		let lock = self.own_lock.write().expect("cant acquire write lock");

		// if someone already did our job for us before we got the lock, return the arc.
		if let Data::Shared(shared) = unsafe { &*data_ptr } {
			drop(lock); // drop the lock before we make a SharedCow
			return Self::new_shared(shared.clone());
		}

		// We both have the lock and checked for Shared; we know we _must_ have `Owned` value now.
		let shared = 
			match unsafe { std::ptr::read(data_ptr) } {
				Data::Owned(owned) => Arc::new(owned),
				Data::Shared(_) => unreachable_debug_or_unchecked!()
			};

		// since we take ownership of the data, we have to ensure that we don't double-drop it
		unsafe {
			std::ptr::write(data_ptr, Data::Shared(shared.clone()));
		}

		drop(lock); // we no longer need the lock.
		Self::new_shared(shared)
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
		SharedCow {
			data: UnsafeCell::new(data),
			own_lock: RwLock::new(())
		}
	}

	pub fn with_ref<F: FnOnce(&T) -> R, R>(&self, func: F) -> R {
		let data_ptr = self.data.get();
		match unsafe { &*data_ptr } {
			Data::Shared(arc) => func(arc),
			Data::Owned(_) => {
				let _lock = self.own_lock.read().expect("cant read lock");
				// we have to check again in case something created a shared reference
				// before we acquired the lock
				match unsafe { &*data_ptr } {
					Data::Shared(arc) => func(arc),
					Data::Owned(owned) => func(owned)
				}
			}
		}
	}
}

impl<T: Clone> SharedCow<T> {
	// #[deprecated]
	// pub fn lock<'a>(&'a self) -> impl std::ops::DerefMut<Target=T> + 'a {
	// 	use std::{sync::RwLockWriteGuard};

	// 	struct Caster<'a, T>(RwLockWriteGuard<'a, ()>, &'a SharedCow<T>);
	// 	impl<'a, T> std::ops::Deref for Caster<'a, T> {
	// 		type Target = T;
	// 		fn deref(&self) -> &T {
	// 			match unsafe { &*self.1.data.get() } {
	// 				Data::Owned(owned) => &owned,
	// 				Data::Shared(shared) => shared.as_ref()
	// 			}
	// 		}
	// 	}

	// 	impl<'a, T: Clone> std::ops::DerefMut for Caster<'a, T> {
	// 		fn deref_mut(&mut self) -> &mut T {
	// 			unsafe { 
	// 				self.1.ensure_owned();
	// 				match &mut *self.1.data.get() {
	// 					Data::Owned(ref mut owned) => owned,
	// 					Data::Shared(_) => unreachable!()
	// 				}
	// 			}
	// 		}
	// 	}

	// 	Caster(self.own_lock.write().expect("poison error"), self)
	// }

	unsafe fn ensure_owned(&self) {
		if let Data::Shared(shared) = &*self.data.get() {
			// we use `replace` because we want the arc to drop itself.
			std::ptr::replace(self.data.get(), Data::Owned(T::clone(&shared)));
		}
	}

	pub fn with_mut<F: FnOnce(&mut T) -> R, R>(&self, func: F) -> R {
		// we have to lock regardless, because we will be accessing `Owned`.
		let _lock = self.own_lock.write().expect("can't write lock");

		unsafe {
			self.ensure_owned();
			match &mut *self.data.get() {
				Data::Owned(owned) => func(owned),
				Data::Shared(_) => unreachable_debug_or_unchecked!()
			}
		}
	}
}









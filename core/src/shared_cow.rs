use std::sync::Arc;
use std::fmt::{self, Debug, Formatter};
use std::borrow::{Borrow, BorrowMut};
use parking_lot::RwLock;

pub(crate) trait Sharable {
	type Owned: Sized + BorrowMut<Self>;
	type Shared: Sized + Borrow<Self> + Clone;
	fn to_shared(owned: Self::Owned) -> Self::Shared;
	fn to_owned(shared: &Self::Shared) -> Self::Owned;
}

impl<T: Clone + Sized> Sharable for T {
	type Owned = Self;
	type Shared = Arc<Self>;

	#[inline]
	fn to_shared(owned: Self::Owned) -> Self::Shared {
		Arc::new(owned)
	}

	#[inline]
	fn to_owned(shared: &Self::Shared) -> Self::Owned {
		Self::clone(shared)
	}
}

pub(crate) struct SharedCow<T: Sharable + ?Sized>(RwLock<Data<T>>);

enum Data<T: Sharable + ?Sized> {
	Owned(T::Owned),
	Shared(T::Shared)
}

impl<T: Sharable + ?Sized> Debug for SharedCow<T>
where
	T::Owned: Debug,
	T::Shared: Debug
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self.0.read() {
			Data::Owned(ref owned) => f.debug_tuple("SharedCow::Owned").field(owned).finish(),
			Data::Shared(ref shared) => f.debug_tuple("SharedCow::Shared").field(shared).finish(),
		}
	}
}

impl<T: Sharable + ?Sized> Default for SharedCow<T>
where
	T::Owned: Default
{
	#[inline]
	fn default() -> Self {
		Self::new(T::Owned::default())
	}
}

impl<T: Sharable + ?Sized> Clone for SharedCow<T> {
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
					Data::Owned(owned) => T::to_shared(owned)
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

impl<T: Sharable + ?Sized> SharedCow<T> {
	#[inline]
	pub fn new_shared(data: T::Shared) -> Self {
		Self(RwLock::new(Data::Shared(data)))
	}

	#[inline]
	pub fn new(data: T::Owned) -> Self {
		Self(RwLock::new(Data::Owned(data)))
	}

	pub fn with_ref<F: FnOnce(&T) -> R, R>(&self, func: F) -> R {
		match *self.0.read() {
			Data::Shared(ref shared) => func(shared.borrow()),
			Data::Owned(ref owned) => func(owned.borrow())
		}
	}
}

impl<T: Sharable + ?Sized> SharedCow<T> {
	pub fn with_mut<F: FnOnce(&mut T) -> R, R>(&self, func: F) -> R {
		let mut lock = self.0.write();

		match *lock {
			Data::Owned(ref mut owned) => func(owned.borrow_mut()),
			Data::Shared(ref shared) => {
				let mut owned = T::to_owned(shared);
				let ret = func(owned.borrow_mut());
				*lock = Data::Owned(owned);
				ret
			}
		}
	}
}

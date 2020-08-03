use std::sync::Arc;
use std::fmt::{self, Debug, Formatter};
use std::borrow::{Borrow, BorrowMut};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::ops::{Deref, DerefMut};

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
			Data::Owned(ref owned) => Debug::fmt(owned, f),
			Data::Shared(ref shared) => Debug::fmt(shared, f),
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
			None => unreachable!("we set it to `Some` either way?")
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

	pub fn read<'a>(&'a self) -> impl Deref<Target=T> + 'a {
		struct Reader<'a, T: Sharable + ?Sized>(RwLockReadGuard<'a, Data<T>>);

		impl<'a, T: Sharable + ?Sized> Deref for Reader<'a, T> {
			type Target = T;
			fn deref(&self) -> &Self::Target {
				match *self.0 {
					Data::Shared(ref shared) => shared.borrow(),
					Data::Owned(ref owned) => owned.borrow(),
				}
			}
		}

		Reader(self.0.read())
	}

	pub fn write<'a>(&'a self) -> impl DerefMut<Target=T> + 'a {
		struct Writer<'a, T: Sharable + ?Sized>(RwLockWriteGuard<'a, Data<T>>);

		impl<'a, T: Sharable + ?Sized> Deref for Writer<'a, T> {
			type Target = T;
			fn deref(&self) -> &Self::Target {
				match *self.0 {
					Data::Shared(ref shared) => shared.borrow(),
					Data::Owned(ref owned) => owned.borrow(),
				}
			}
		}

		impl<'a, T: Sharable + ?Sized> DerefMut for Writer<'a, T> {
			fn deref_mut(&mut self) -> &mut Self::Target {
				if let Data::Shared(ref shared) = *self.0 {
					*self.0 = Data::Owned(T::to_owned(shared));
				}

				match *self.0 {
					Data::Owned(ref mut owned) => owned.borrow_mut(),
					Data::Shared(_) => unreachable!("we already converted shared to owned?")
				}
			}
		}

		Writer(self.0.write())
	}
}

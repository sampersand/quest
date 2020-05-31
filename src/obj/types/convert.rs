use crate::obj::{Object, Result, Args, types::rustfn::Binding};
use std::any::Any;
use std::sync::{RwLockWriteGuard, RwLockReadGuard};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};


pub trait Convertible : Any + Sized + Clone {
	const CONVERT_FUNC: &'static str;
}

impl Object {
	pub fn downcast_convert<T: Convertible>(&self) -> Result<Self> {
		self.call_attr(T::CONVERT_FUNC, Args::default())
	}

	pub fn downcast_call<T: Convertible>(&self) -> Result<T> {
		self.downcast_convert::<T>().and_then(|o| o.try_downcast_clone())
	}
}
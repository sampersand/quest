use crate::obj::{Object, Result, Args, types::rustfn::Binding};
use std::any::Any;
use std::sync::{RwLockWriteGuard, RwLockReadGuard};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};


pub trait Convertible : Any + Sized + Clone {
	const CONVERT_FUNC: &'static str;
}

impl Object {
	pub fn downcast_call<T: Convertible>(&self, binding: &Binding) -> Result<T> {
		self.call(T::CONVERT_FUNC, Args::new_slice(&[], binding.clone()))
			.and_then(|o| o.try_downcast_clone())
	}
}
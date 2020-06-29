use crate::{Object, Result};
use std::any::Any;

pub trait Convertible : Any + Sized + Clone {
	const CONVERT_FUNC: &'static str;
}

impl Object {
	pub fn downcast_call<T: Convertible>(&self) -> Result<T> {
		self.call_attr(T::CONVERT_FUNC, crate::Args::default())
			.and_then(|o| o.try_downcast_clone())
	}
}
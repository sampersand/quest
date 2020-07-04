use crate::Object;
use crate::literals::Literal;
use std::any::{Any, type_name};

pub trait Convertible : Any + Sized + Clone {
	const CONVERT_FUNC: Literal;
}

impl Object {
	#[inline]
	pub fn call_downcast_map<T, O, F>(&self, f: F) -> crate::Result<O>
	where
		T: Convertible + Any,
		F: FnOnce(&T) -> O
	{
		self.call_downcast_and_then::<T, O, !, _>(|x| Ok(f(x)))
	}

	pub fn call_downcast_and_then<T, O, E, F>(&self, f: F) -> crate::Result<O>
	where
		T: Convertible + Any,
		E: Into<crate::Error>,
		F: FnOnce(&T) -> Result<O, E>,
	{
		if self.is_a::<T>() {
			unsafe {
				self.downcast_unchecked_and_then(f).map_err(Into::into)
			}
		} else {
			self.call_attr_lit(T::CONVERT_FUNC, &[]).and_then(|obj| {
				if obj.is_a::<T>() {
					unsafe {
						obj.downcast_unchecked_and_then(f).map_err(Into::into)
					}
				} else {
					Err(crate::error::TypeError::ConversionReturnedBadType {
						func: T::CONVERT_FUNC,
						expected: type_name::<T>(),
						got: obj.typename()
					}.into())
				}
			})
		}
	}
}




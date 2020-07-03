use crate::Object;
use crate::literals::Literal;
use std::any::{Any, type_name};

pub trait Convertible : Any + Sized + Clone {
	const CONVERT_FUNC: Literal;
}

impl Object {
	pub fn downcast_call<T: Convertible>(&self) -> crate::Result<T> {
		self.call_attr_lit(T::CONVERT_FUNC, crate::Args::default())
			.and_then(|o| o.try_downcast_clone())
	}

	pub fn with_ref_call<T, O, E, F>(&self, f: F) -> crate::Result<O>
	where
		T: Convertible + Any,
		E: Into<crate::Error>,
		F: FnOnce(&T) -> Result<O, E>,
	{
		if self.is_a::<T>() {
			unsafe {
				self.with_ref_unchecked(f).map_err(Into::into)
			}
		} else {
			self.call_attr_lit(T::CONVERT_FUNC, &[]).and_then(|obj| {
				if self.is_a::<T>() {
					unsafe {
						self.with_ref_unchecked(f).map_err(Into::into)
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




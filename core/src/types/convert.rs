use crate::Object;
use crate::error::TypeError;
use crate::literal::Literal;
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
		self.call_downcast_and_then(|x| Ok(f(x)))
	}

/*
	pub fn call_downcast<'a, T>(&'a self) -> crate::Result<impl AsRef<T> + 'a>
	where
		T: Convertible + Any
	{
		use std::marker::PhantomData;

		struct Convert<'a, T>(Object, PhantomData<&'a T>);
		
		impl<'a, T: 'static> AsRef<T> for Convert<'a, T> {
			fn as_ref<'b>(&'b self) -> &'b T {
				Object::downcast::<'b, T>(self).unwrap()
			}
		}
		// impl<'a, T: 'a> std::ops::Deref for Convert<'a, T> {
		// 	type Target = &'a T;
		// 	fn deref(&self) -> &'a Self::Target {
		// 		self.0.downcast().unwrap()
		// 	}
		// }

		Ok(Convert(self.call_attr_lit(T::CONVERT_FUNC, &[])?, PhantomData))
	}*/


/*
	pub fn call_downcast<'a, T>(&'a self) -> crate::Result<impl std::ops::Deref<Target=T> + 'a>
	where
		T: Convertible + Any
	{
		use std::marker::PhantomData;

		enum CalledReader<T, D> {
			Original(D, std::marker::PhantomData<T>),
			Converted(Object)
		}

		use std::ops::Deref;

		impl<T: 'static, D: std::ops::Deref<Target=T>> std::ops::Deref for CalledReader<T, D> {
			type Target = T;
			fn deref(&self) -> &Self::Target {
				match self {
					Self::Original(orig, _) => &orig,
					Self::Converted(obj) => obj.downcast::<T>().expect("bad downcast").deref()
				}
			}
		}

		if let Some(this) = self.downcast::<T>() {
			return Ok(CalledReader::Original(this, PhantomData));
		}

		let converted = self.call_attr_lit(T::CONVERT_FUNC, &[])?;
		if converted.is_a::<T>() {
			Ok(CalledReader::Converted(converted))
		} else {
			Err(TypeError::ConversionReturnedBadType {
				func: T::CONVERT_FUNC,
				expected: type_name::<T>(),
				got: converted.typename()
			}.into())
		}
	}*/

	pub fn call_downcast_and_then<T, O, F>(&self, f: F) -> crate::Result<O>
	where
		T: Convertible + Any,
		F: FnOnce(&T) -> crate::Result<O>,
	{
		if self.is_a::<T>() {
			self.downcast().map(|d| f(&d)).unwrap().map_err(Into::into)
		} else {
			self.call_attr_lit(T::CONVERT_FUNC, &[]).and_then(|obj| {
				if obj.is_a::<T>() {
					obj.downcast().map(|d| f(&d)).unwrap()
				} else {
					Err(TypeError::ConversionReturnedBadType {
						func: T::CONVERT_FUNC,
						expected: type_name::<T>(),
						got: obj.typename()
					}.into())
				}
			})
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Object;

	#[derive(Debug, Clone)]
	struct Dummy;
	impl Convertible for Dummy { const CONVERT_FUNC: Literal = "@dummy"; }
	impl_object_type! { for Dummy [(parents crate::types::Basic)]:
		"@dummy2" => function |_, _| Ok(Object::from(Dummy2)),
		"@dummy3" => function |_, _| Ok(Object::from(Dummy))
	}

	#[derive(Debug, Clone)]
	struct Dummy2;
	impl Convertible for Dummy2 { const CONVERT_FUNC: Literal = "@dummy2"; }
	impl_object_type! { for Dummy2 [(parents crate::types::Basic) (setup IS_SETUP2)]: }

	#[derive(Debug, Clone)]
	struct Dummy3;
	impl Convertible for Dummy3 { const CONVERT_FUNC: Literal = "@dummy3"; }
	impl_object_type! { for Dummy3 [(parents crate::types::Basic) (setup IS_SETUP3)]: }


	#[test]
	fn call_downcast_map() {
		use crate::{Error, error::KeyError};
		<Dummy as crate::types::ObjectType>::initialize().unwrap();
		<Dummy2 as crate::types::ObjectType>::initialize().unwrap();
		<Dummy3 as crate::types::ObjectType>::initialize().unwrap();

		Object::from(Dummy).call_downcast_map(|_: &Dummy| {}).unwrap();
		Object::from(Dummy).call_downcast_map(|_: &Dummy2| {}).unwrap();

		assert_matches!(
			Object::from(Dummy).call_downcast_map(|_: &Dummy3| {}).unwrap_err(),
			Error::TypeError(TypeError::ConversionReturnedBadType {
				func: Dummy3::CONVERT_FUNC,
				expected, got }) if expected == type_name::<Dummy3>() && got == type_name::<Dummy>()
		);

		assert_matches!(
			Object::from(Dummy2).call_downcast_map(|_: &Dummy| {}).unwrap_err(),
			Error::KeyError(KeyError::DoesntExist { ref attr, .. })
				if attr.eq_obj(&Dummy::CONVERT_FUNC.into()).unwrap_or(false)
		)
	}
}





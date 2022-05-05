use crate::{Object, Literal, types::ObjectType, error::TypeError};
use std::any::{Any, type_name};
use std::ops::Deref;
use std::marker::PhantomData;


/// A trait that's used to represent the ability for an object to be converted to within Quest, e.g.
/// via `@text`.
///
/// Not every object needs to implement this. For example, it doesn't make much sense to have a `@rustfn`, as there's no
/// way to convert to [`RustFn`]s(crate::types::RustFn).
pub trait Convertible : Any + Sized + Clone + ObjectType {
	/// The function that does the actual conversion.
	const CONVERT_FUNC: Literal;
}

impl Object {
	/// Attempts to convert `self` to the given type, calling [`T::CONVERT_FUNC`](Convertible::CONVERT_FUNC) if `self` is
	/// not the correct type.
	pub fn call_downcast<'a, T: Convertible>(&'a self) -> crate::Result<impl Deref<Target=T> + 'a> {
		// We either have an `Original` reference or a `Converted` Object.
		enum CalledReader<T, D> {
			Original(D, PhantomData<T>),
			Converted(Object, D)
		}

		impl<T, D> Deref for CalledReader<T, D>
		where
			T: ObjectType,
			D: Deref<Target=T>,
		{
			type Target = T;
			fn deref(&self) -> &Self::Target {
				match self {
					Self::Original(orig, _) => &orig,
					Self::Converted(_, data) => &data
				}
			}
		}

		if let Some(this) = self.downcast::<T>() {
			return Ok(CalledReader::Original(this, PhantomData));
		}

		let converted = self.call_attr_lit(&T::CONVERT_FUNC, &[])?;

		if converted.is_a::<T>() {
			// SAFETY: Since we know the `converted` is a `T` already, then we're able to transmute.
			// This is needed because the compiler isn't smart enough to know that the two opaque `impl` types are actually
			// the same.
			let dc = unsafe {
				let cdc = converted.downcast::<T>().expect("bad downcast");
				let dc = std::mem::transmute_copy(&cdc);
				std::mem::forget(cdc);
				dc
			};

			Ok(CalledReader::Converted(converted, dc))
		} else {
			Err(TypeError::ConversionReturnedBadType {
				func: T::CONVERT_FUNC,
				expected: type_name::<T>(),
				got: converted.typename()
			}.into())
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Object;

	#[derive(Debug, Clone)]
	struct Dummy;
	impl Convertible for Dummy { const CONVERT_FUNC: Literal = Literal::new("@dummy"); }
	impl_object_type! { for Dummy [(parents crate::types::Basic)]:
		"@dummy2" => method |_, _| Ok(Object::from(Dummy2)),
		"@dummy3" => method |_, _| Ok(Object::from(Dummy))
	}

	#[derive(Debug, Clone)]
	struct Dummy2;
	impl Convertible for Dummy2 { const CONVERT_FUNC: Literal = Literal::new("@dummy2"); }
	impl_object_type! { for Dummy2 [(parents crate::types::Basic) (setup IS_SETUP2)]: }

	#[derive(Debug, Clone)]
	struct Dummy3;
	impl Convertible for Dummy3 { const CONVERT_FUNC: Literal = Literal::new("@dummy3"); }
	impl_object_type! { for Dummy3 [(parents crate::types::Basic) (setup IS_SETUP3)]: }


	#[test]
	fn call_downcast() {
		use crate::{Error, error::KeyError};
		<Dummy as crate::types::ObjectType>::initialize().unwrap();
		<Dummy2 as crate::types::ObjectType>::initialize().unwrap();
		<Dummy3 as crate::types::ObjectType>::initialize().unwrap();

		Object::from(Dummy).call_downcast::<Dummy>().unwrap();
		Object::from(Dummy).call_downcast::<Dummy2>().unwrap();

		assert_matches!(
			Object::from(Dummy).call_downcast::<Dummy3>().map(|x| x.clone()).unwrap_err(),
			Error::TypeError(TypeError::ConversionReturnedBadType {
				func: Dummy3::CONVERT_FUNC,
				expected, got }) if expected == type_name::<Dummy3>() && got == type_name::<Dummy>()
		);

		assert_matches!(
			Object::from(Dummy2).call_downcast::<Dummy>().map(|x| x.clone()).unwrap_err(),
			Error::KeyError(KeyError::DoesntExist { ref attr, .. })
				if attr.eq_obj(&Dummy::CONVERT_FUNC.into()).unwrap_or(false)
		)
	}
}





macro_rules! impl_object_type {
	(for $obj:ty, $parent:ty; $($name:expr => $fn:expr),* $(,)?) => {
		impl_object_type!(for $obj, $parent, @$parent; $($name => $fn),*);
	};

	(for $obj:ty, $parent:ty, $( @ $init_parent:ty)?; $($name:expr => $fn:expr),* $(,)?) => {
		impl $crate::obj::types::ObjectType for $obj {
			fn mapping() -> $crate::obj::Object {
				use std::mem::{self, MaybeUninit};
				use std::sync::{Once, Arc, RwLock};
				use $crate::obj::{Object, Mapping};

				static mut CLASS_OBJECT: MaybeUninit<Object> = MaybeUninit::uninit();
				static mut HAS_SETUP_HAPPENED: bool = false;
				static mut HAS_CREATE_HAPPENED: bool = false; // todo: make sync-safe

				unsafe {
					if !HAS_CREATE_HAPPENED {
						HAS_CREATE_HAPPENED = true;
						// todo: change data type to be parent's
						CLASS_OBJECT.as_mut_ptr().write(Object::new_with_parent(<$parent as Default>::default(), None));
					}
				}

				unsafe {
					if !HAS_SETUP_HAPPENED {
						HAS_SETUP_HAPPENED = true;

						let class = (*CLASS_OBJECT.as_ptr()).clone();
						use $crate::obj::{Object, types::*};
						$(
							class.set_attr("__parent__".into(), <$init_parent as $crate::obj::types::ObjectType>::mapping());
						)?
						class.set_attr("name".into(), stringify!($obj).into());
						$({
							class.set_attr($name.into(), $crate::obj::types::RustFn::new($name, $fn).into());
						})*
					}
				}

				unsafe {
					(*CLASS_OBJECT.as_ptr()).clone()
				}
			}
		}
	};
}
// macro_rules! impl_object_conversions {
// 	($enum:ident $conv_func:literal $as_obj:ident($ty:ty) $as:ident
// 	 $into:ident $try_as:ident $try_into:ident $call_into:ident $ret:ty
// 	) => {
// 		impl $crate::obj::Object {
// 			pub fn $call_into(&self) -> ::std::result::Result<$ret, $crate::obj::Object> {
// 				self.call($conv_func, &[])?.$try_into()
// 			}

// 			pub fn $try_into(&self) -> ::std::result::Result<$ret, $crate::obj::Object> {
// 				self.$try_as().map(Clone::clone)
// 			}

// 			pub fn $into(&self) -> Option<$ret> {
// 				self.$as().map(Clone::clone)
// 			}

// 			pub fn $try_as(&self) -> ::std::result::Result<&$ret, $crate::obj::Object> {
// 				self.$as().ok_or_else(|| concat!("not a ", stringify!($ret)).into())
// 			}

// 			pub fn $as(&self) -> Option<&$ret> {
// 				if let Some(t) = self.downcast_ref::<$ret>() {
// 				if let $crate::obj::DataEnum::$enum(ref t) = self.0.data_ {
// 					Some(AsRef::<$ret>::as_ref(t))
// 				} else {
// 					None
// 				}
// 			}

// 			pub fn $as_obj(&self) -> Option<&$ty> {
// 				if let $crate::obj::DataEnum::$enum(ref t) = self.0.data_ {
// 					Some(t)
// 				} else {
// 					None
// 				}
// 			}
// 		}
// 	}
// }

macro_rules! impl_object_type {
	(for $ty:ty $(, $parent:ty)?; $($name:expr => $fn:expr),* $(,)?) => {
	// 	impl_object_type!(;
	// 		$ty,
	// 		$crate::obj::Object::new_with_parent::<$parent>(
	// 			Default::default(),
	// 		),
	// 			Some(<$parent as $crate::obj::types::ObjectType>::mapping());
	// 		$($name => $fn),*
	// 	);
	// };
	// (for $ty:ty; $($name:expr => $fn:expr),*  $(,)?) => {
	// 	impl_object_type!(;
	// 		$ty,
	// 		$crate::obj::Object::new_with_parent($crate::obj::DataEnum::Empty, None);
	// 		$($name => $fn),*
	// 	);
	// };

	// (; $ty:ty, $obj_init:expr, $parent_t:expr; $($name:expr => $fn:expr),*) => {
		impl $crate::obj::types::ObjectType for $ty {
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
						CLASS_OBJECT.as_mut_ptr().write(Object::new_with_parent(
							$crate::obj::DataEnum::Empty, None
						));
					}
				}

				// static mut N: isize = 0;
				// unsafe {
				// 	N += 1;
				// 	if N > 500 {
				// 		panic!("N > 500 for: {:?}", stringify!($ty));
				// 	}
				// }

				unsafe {
					if !HAS_SETUP_HAPPENED {
						HAS_SETUP_HAPPENED = true;

						let class = (*CLASS_OBJECT.as_ptr()).clone();
						use $crate::obj::{Object, types::*};
						$(
							class.set_attr("__parent__".into(), <$parent as $crate::obj::types::ObjectType>::mapping());
						)?
						class.set_attr("name".into(), stringify!($ty).into());
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

// macro_rules! impl_object_type_ {
// 	(for $ty:ty, $parent:expr; $(fn $name:literal $fn:tt)*) => {
// 		impl $crate::obj::types::ObjectType for $ty {
// 			fn mapping() -> $crate::obj::Object {
// 				use std::mem::{self, MaybeUninit};
// 				use std::sync::{Once, Arc, RwLock};
// 				use $crate::obj::{Object, Mapping};

// 				static mut CLASS_OBJECT: MaybeUninit<Object> = MaybeUninit::uninit();
// 				static CREATE_MAPPING: Once = Once::new();
// 				static mut HAS_SETUP_HAPPENED: bool = false;
// 				// println!("{:?}::mapping called", stringify!($ty));

// 				CREATE_MAPPING.call_once(|| unsafe {
// 					CLASS_OBJECT.as_mut_ptr().write(Object::new_with_parent($crate::obj::types::Class, $parent));
// 				});

// 				static mut N: i32 = 0;
// 				unsafe {
// 					if N > 1000 {
// 						panic!("too much: {:?}", stringify!($ty));
// 					}
// 					N += 1;
// 				}

// 				unsafe {
// 					if !HAS_SETUP_HAPPENED {
// 				// println!("setup hasn't happened for: {:?}", stringify!($ty));
// 						HAS_SETUP_HAPPENED = true;

// 						let class = (*CLASS_OBJECT.as_ptr()).clone();
// 						use $crate::obj::{Object, types::*};
// 						// , Mapping::new($parent)
// 						class.set_attr("name".into(), stringify!($ty).into());
// 						$({
// 							class.set_attr($name.into(), $crate::obj::types::RustFn::new($name, $fn).into());
// 						})*
// 					}
// 				}

// 				unsafe {
// 					(*CLASS_OBJECT.as_ptr()).clone()
// 				}
// 			}
// 		}
// 	};
// }
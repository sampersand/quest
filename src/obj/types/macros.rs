macro_rules! impl_trait {
	(IntoInner<Inner=$inner:ty> for $obj:ty) => {
		impl $crate::util::IntoInner for Boolean {
			type Inner = $inner;

			fn into_inner(self) -> $inner { 
				<$inner>::from(self)
			}
		}
	};

	(From<$inner:ty> for $obj:ty) => {
		impl From<$inner> for $obj {
			fn from(t: $inner) -> Self {
				Self::new(t)
			}
		}
	};

	(From<$obj:ty, $inner:ty> for Object) => {
		impl From<$inner> for $crate::obj::Object {
			fn from(t: $inner) -> Self {
				<$obj>::from(t).into()
			}
		}
	};

	(Into<$inner:ty> for $obj:ty) => {
		impl From<$obj> for $inner {
			fn from(t: $obj) -> Self {
				t.0
			}
		}
	};

	(AsRef<$inner:ty> for $obj:ty) => {
		impl AsRef<$inner> for $obj {
			fn as_ref(&self) -> &$inner {
				&self.0
			}
		}
	};

	(ObjectType<parent=$parent:ty$(, @ $init_parent:ty)?> for $obj:ty { $($name:expr => $fn:expr),* $(,)? }) => {
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
	}
}

macro_rules! impl_object_type {
	(for $ty:ty, $parent:ty; $($name:expr => $fn:expr),* $(,)?) => {
		impl_object_type!(for $ty, $parent, @$parent; $($name => $fn),*);
	};

	(for $ty:ty, $parent:ty, $( @ $init_parent:ty)?; $($name:expr => $fn:expr),* $(,)?) => {
		impl_trait!(ObjectType<parent=$parent $(, @ $init_parent)?> for $ty { $($name => $fn),* });
		// impl $crate::obj::types::ObjectType for $ty {
		// 	fn mapping() -> $crate::obj::Object {
		// 		use std::mem::{self, MaybeUninit};
		// 		use std::sync::{Once, Arc, RwLock};
		// 		use $crate::obj::{Object, Mapping};

		// 		static mut CLASS_OBJECT: MaybeUninit<Object> = MaybeUninit::uninit();
		// 		static mut HAS_SETUP_HAPPENED: bool = false;
		// 		static mut HAS_CREATE_HAPPENED: bool = false; // todo: make sync-safe

		// 		unsafe {
		// 			if !HAS_CREATE_HAPPENED {
		// 				HAS_CREATE_HAPPENED = true;
		// 				// todo: change data type to be parent's
		// 				CLASS_OBJECT.as_mut_ptr().write(Object::new_with_parent(<$parent as Default>::default(), None));
		// 			}
		// 		}

		// 		unsafe {
		// 			if !HAS_SETUP_HAPPENED {
		// 				HAS_SETUP_HAPPENED = true;

		// 				let class = (*CLASS_OBJECT.as_ptr()).clone();
		// 				use $crate::obj::{Object, types::*};
		// 				$(
		// 					println!("setting parent for {:?}", stringify!($ty));
		// 					class.set_attr("__parent__".into(), <$init_parent as $crate::obj::types::ObjectType>::mapping());
		// 				)?
		// 				class.set_attr("name".into(), stringify!($ty).into());
		// 				$({
		// 					class.set_attr($name.into(), $crate::obj::types::RustFn::new($name, $fn).into());
		// 				})*
		// 			}
		// 		}

		// 		unsafe {
		// 			(*CLASS_OBJECT.as_ptr()).clone()
		// 		}
		// 	}
		// }
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
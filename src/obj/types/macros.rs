macro_rules! getarg {
	($what:tt; $args:expr) => {
		getarg!($what; $args, 1)
	};

	(Number; $args:expr, $pos:expr) => {
		$args.get($pos)?.call("@num", $args.new_same_binding(&[] as &[_]))?.try_downcast_ref::<$crate::obj::types::Number>()?
	};
	
	(Boolean; $args:expr, $pos:expr) => {
		$args.get($pos)?.call("@bool", $args.new_same_binding(&[] as &[_]))?.try_downcast_ref::<$crate::obj::types::Boolean>()?
	};

	(Text; $args:expr, $pos:expr) => {
		$args.get($pos)?.call("@text", $args.new_same_binding(&[] as &[_]))?.try_downcast_ref::<$crate::obj::types::Text>()?
	};

	($other:tt $args:expr, $pos:expr) => {
		error!(concat!("unknown type `", stringify!($other), "`"));
	}
}


macro_rules! impl_object_type {
	(for $obj:ty, $parent:ty; $($args:tt)*) => {
		impl_object_type!(SETUP $obj; $parent; $parent; $($args)*);
	};

	(for $obj:ty, $parent:ty, $($init_parent:ty)?; $($args:tt)*) => {
		impl_object_type!(SETUP $obj; $parent; $($init_parent)?; $($args)*);
	};

	(SET_ATTR $class:ident $attr:literal, (expr $val:expr)) => {
		$class.set_attr($attr.into(), $val.into(), &Object::default()); // TODO: binding
	};

	(SET_ATTR $class:ident $attr:literal, ($val:expr)) => {
		$class.set_attr($attr.into(), $crate::obj::types::RustFn::new($attr, $val).into(), &Object::default()); // TODO: binding
	};

	(SET_ATTR $_class:ident $attr:expr, $val:expr) => {
		compile_error!(concat!("Bad attr (", stringify!($attr), ") or val (", stringify!($val), ")"))
	};
	(SETUP $obj:ty; $parent:ty; $($init_parent:ty)?; $($attr:expr => ($($val:tt)*)),* $(,)?) => {
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
							class.set_attr(
								"__parent__".into(),
								<$init_parent as $crate::obj::types::ObjectType>::mapping(),
								&Object::default() // TODO: what binding should be used when setting parent?
							);
						)?
						class.set_attr(
							"name".into(),
							stringify!($obj).into(),
							&Object::default() // TODO: what binding should be used when setting name?
						);
						$({
							impl_object_type!(SET_ATTR class $attr, ($($val)*));
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
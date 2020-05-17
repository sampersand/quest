#[cfg(test)]
macro_rules! assert_call_eq {
	(for $ty:ty; $($rhs:expr, $lhs:ident($this:expr $(,$args:expr)*) -> $ret:ty),* $(,)?) => {{
		use crate::obj::types::{self, *, rustfn::Args};

		<$ty>::wait_for_setup_to_finish();
		$(
			assert_eq!(*impls::$lhs({
				let mut args = Args::new(vec![$($args.into()),*], Default::default());
				args.add_this($this.into());
				args
			}).unwrap().downcast_ref::<$ret>().unwrap(), $rhs);
		)*
	}};
}

macro_rules! getarg {
	($what:tt; $args:expr) => {
		getarg!($what; $args, 1)
	};

	(Number; $args:expr, $pos:expr) => {
		getarg!(Object; $args, $pos).call("@num", $args.new_args_slice(&[]))?.try_downcast_ref::<$crate::obj::types::Number>()?
	};
	
	(Boolean; $args:expr, $pos:expr) => {
		getarg!(Object; $args, $pos).call("@bool", $args.new_args_slice(&[]))?.try_downcast_ref::<$crate::obj::types::Boolean>()?
	};

	(Text; $args:expr, $pos:expr) => {
		getarg!(Object; $args, $pos).call("@text", $args.new_args_slice(&[]))?.try_downcast_ref::<$crate::obj::types::Text>()?
	};

	(Object; $args:expr, $pos:expr) => {
		$args.get($pos)?
	};

	($other:tt $args:expr, $pos:expr) => {
		error!(concat!("unknown type `", stringify!($other), "`"));
	}
}


macro_rules! impl_object_type {
	(for;$convert_func:literal; $obj:ty, $parent:ty; $($args:tt)*) => {
		impl_object_type!(SETUP $obj; $parent; $convert_func; $parent; $($args)*);
	};
	(for $obj:ty, $parent:ty; $($args:tt)*) => {
		impl_object_type!(SETUP $obj; $parent;; $parent; $($args)*);
	};

	(for $obj:ty $([$convert_func:literal])?, $parent:ty, $($init_parent:ty)?; $($args:tt)*) => {
		impl_object_type!(SETUP $obj; $parent; $($convert_func)?; $($init_parent)?; $($args)*);
	};

	(SET_ATTR $class:ident $attr:literal, (expr $val:expr)) => {
		$class.set_attr($attr.into(), $val.into(), &Default::default());
	};

	(SET_ATTR $class:ident $attr:literal, ($val:expr)) => {
		$class.set_attr($attr.into(), $crate::obj::types::RustFn::new($attr, $val).into(), &Default::default());
	};

	(SET_ATTR $_class:ident $attr:expr, $val:expr) => {
		compile_error!(concat!("Bad attr (", stringify!($attr), ") or val (", stringify!($val), ")"))
	};
	(SETUP $obj:ty; $parent:ty; $($convert_func:literal)?; $($init_parent:ty)?; $($attr:expr => ($($val:tt)*)),* $(,)?) => {
		$(
			impl $crate::obj::types::Convertible for $obj {
				const CONVERT_FUNC: &'static str = $convert_func;
			}
		)?

		static mut IS_SETUP: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
		impl $crate::obj::types::ObjectType for $obj {
			fn wait_for_setup_to_finish() {
				Self::mapping();
				while unsafe { IS_SETUP.load(std::sync::atomic::Ordering::SeqCst) == false } {
					std::thread::yield_now();
				}
			}

			fn mapping() -> $crate::obj::Object {
				use std::mem::{self, MaybeUninit};
				use std::sync::{Once, Arc, RwLock, atomic::{AtomicU8, Ordering}};
				use $crate::obj::{Object, Mapping};

				static mut CLASS_OBJECT: MaybeUninit<Object> = MaybeUninit::uninit();
				static mut HAS_SETUP_HAPPENED: AtomicU8 = AtomicU8::new(0);
				static HAS_CREATE_HAPPENED: Once = Once::new();

				HAS_CREATE_HAPPENED.call_once(|| unsafe {
					CLASS_OBJECT.as_mut_ptr().write(Object::new_with_parent(
						<$parent as Default>::default(),
						None
					));
				});

				let class = unsafe { (*CLASS_OBJECT.as_ptr()).clone() };
				
				if unsafe { HAS_SETUP_HAPPENED.compare_and_swap(0, 1, Ordering::SeqCst) } == 0 {
					use $crate::obj::{Object, types::*};
					$(
						class.set_attr(
							"__parent__".into(),
							<$init_parent as $crate::obj::types::ObjectType>::mapping(),
							&Default::default() // TODO: actual binding everywhere
						);
					)?

					class.set_attr("name".into(), stringify!($obj).into(), &Default::default());
					$({
						impl_object_type!(SET_ATTR class $attr, ($($val)*));
					})*

 					unsafe {
 						IS_SETUP.store(true, std::sync::atomic::Ordering::SeqCst);
 					}
				} else {
					// while unsafe { HAS_SETUP_HAPPENED.load(Ordering::SeqCst) == 1 } {

					// }
				}

				class
			}
		}
	};
}
#[cfg(test)]
macro_rules! dummy_object {
	($vis:vis struct $obj:ident $(($($types:ty),*))?;) =>{
		dummy_object!($vis struct $obj $(($($types),*))?; $crate::obj::types::Basic {});
	};

	($vis:vis struct $obj:ident $(($($types:ty),*))?; $parent:ty) =>{
		dummy_object!($vis struct $obj $(($($types),*))?; $parent {});
	};

	($vis:vis struct $obj:ident $(($($types:ty),*))?; { $($args:tt)* }) =>{
		dummy_object!($vis struct $obj $(($($types),*))?; $crate::obj::types::Basic { $($args)* });
	};

	($vis:vis struct $obj:ident $(($($types:ty),*))?; $parent:path { $($args:tt)* }) =>{
		#[derive(Debug, Clone)]
		$vis struct $obj$(($($types),*))?;
		impl_object_type!(for $obj [(parent $parent)]: $($args)* );
	};
}

#[cfg(test)]
macro_rules! call_impl {
	($fnc:ident($this:expr $(,$args:expr)*) -> $ret:ty) => {{
		use crate::obj::types::{self, *, rustfn::Args};
		impls::$fnc({
			let mut args = Args::new(vec![$($args.into()),*], Default::default());
			args.add_this($this.into());
			args
		}).unwrap().downcast_ref::<$ret>().unwrap()
	}};
}
#[cfg(test)]
macro_rules! assert_call_eq {
	(for $ty:ty; $($lhs:expr, $rhs:ident($this:expr $(,$args:expr)*) -> $ret:ty),* $(,)?) => {{
		use crate::obj::types::{self, *, rustfn::Args};
		#[cfg(test)]
		<$ty>::_wait_for_setup_to_finish();
		let mut which = 1;
		$(
			assert_eq!($lhs, *call_impl!($rhs($this $(,$args)*) -> $ret), "Bad test #{}", which);
			which += 1;
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
	(@CONVERTIBLE $_obj:ty;) => {};
	(@CONVERTIBLE $obj:ty; (convert $convert_func:expr) $($_rest:tt)*) => {
		impl $crate::obj::types::Convertible for $obj {
			const CONVERT_FUNC: &'static str = $convert_func;
		}
	};
	(@CONVERTIBLE $obj:ty; $_b:tt $($rest:tt)*) => {
		impl_object_type!(@CONVERTIBLE $obj; $($rest)*);
	};
	(@CONVERTIBLE $($tt:tt)*) => {
		compile_error!(concat!("bad CONVERTIBLE: ", stringify!($($tt)*)))
	};

	(@SETUP) => { IS_SETUP };
	(@SETUP (setup $name:ident) $($_rest:tt)*) => { $name };
	(@SETUP $_b:tt $($rest:tt)*) => {
		impl_object_type!(@SETUP $($rest)*);
	};

	(@SETUP_INIT) => { impl_object_type!(@SETUP_INIT (setup IS_SETUP)); };
	(@SETUP_INIT (setup $name:ident) $($_rest:tt)*) => {
		static mut $name: std::sync::atomic::AtomicBool
			= std::sync::atomic::AtomicBool::new(false);
	};
	(@SETUP_INIT $_b:tt $($rest:tt)*) => {
		impl_object_type!(@SETUP_INIT $($rest)*);
	};

	(@PARENT_DEFAULT) => {
		compile_error!("A parent is needed to create an object");
	};
	(@PARENT_DEFAULT (parent $parent:path) $($_rest:tt)*) => {
		<$parent as Default>::default()
	};
	(@PARENT_DEFAULT $_b:tt $($rest:tt)*) => {
		impl_object_type!(@PARENT_DEFAULT $($rest)*);
	};

	(@SET_PARENT $class:ident) => {
		compile_error!("parent should have been checked for earlier");
	};
	(@SET_PARENT $class:ident (init_parent $($init_parent:path)?) $($_rest:tt)*) => {
		$(
			$class.set_attr(
				"__parent__".into(),
				<$init_parent as $crate::obj::types::ObjectType>::mapping(),
				&Default::default() // TODO: actual binding everywhere
			);
		)?
	};
	(@SET_PARENT $class:ident (parent $parent:path) $($_rest:tt)*) => {
		impl_object_type!(@SET_PARENT $class (init_parent $parent));
	};

	(@SET_PARENT $class:ident $_b:tt $($rest:tt)*) => {
		impl_object_type!(@SET_PARENT $class $($rest)*)
	};

	(@SET_ATTRS $class:ident) => {};
	(@SET_ATTRS $class:ident $attr:literal => const $val:expr $(, $($args:tt)*)?) => {{
		$class.set_attr($attr.into(), $val.into(), &Default::default());
		impl_object_type!(@SET_ATTRS $class $($($args)*)?);
	}};

	(@SET_ATTRS $class:ident $attr:literal => $val:expr $(, $($args:tt)*)?) => {{
		$class.set_attr(
			$attr.into(),
			$crate::obj::types::RustFn::new($attr, $val).into(),
			&Default::default()
		);
		impl_object_type!(@SET_ATTRS $class $($($args)*)?);
	}};

	(@SET_ATTRS $_class:ident $($tt:tt)*) => {
		compile_error!(concat!("Bad attrs given:", stringify!($($tt)*)));
	};


	(for $obj:ty [ $($args:tt)* ]: $($body:tt)*/*$($attr:expr => ($($attr_val:tt)*)),* $(,)?*/) => {
		impl_object_type!(@CONVERTIBLE $obj; $($args)* );

		#[cfg(test)]
		impl_object_type!(@SETUP_INIT $($args)*);

		impl $crate::obj::types::ObjectType for $obj {
			#[cfg(test)]
			fn _wait_for_setup_to_finish() {
				Self::mapping();
				while unsafe {
					impl_object_type!(@SETUP $($args)*)
						.load(std::sync::atomic::Ordering::SeqCst) == false
				} {
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
						impl_object_type!(@PARENT_DEFAULT $($args)*),
						None
					));
				});

				let class = unsafe { (*CLASS_OBJECT.as_ptr()).clone() };
				
				if unsafe { HAS_SETUP_HAPPENED.compare_and_swap(0, 1, Ordering::SeqCst) } == 0 {
					use $crate::obj::{Object, types::*};
					impl_object_type!(@SET_PARENT class $($args)*);

					class.set_attr("name".into(), stringify!($obj).into(), &Default::default());
					impl_object_type!(@SET_ATTRS class $($body)*);

					#[cfg(test)]
 					unsafe {
						impl_object_type!(@SETUP $($args)*)
 							.store(true, std::sync::atomic::Ordering::SeqCst);
 					}
				}
				class
			}
		}
	};
}

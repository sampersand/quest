#[cfg(test)]
macro_rules! assert_downcast_eq {
	($lhs:expr, $rhs:expr) => {
		$lhs.unwrap().downcast_and_then(|lhs| assert_eq!(*lhs, $rhs)).unwrap()
	};
	($ty:ty; $lhs:expr, $rhs:expr) => {

		$lhs.unwrap().downcast_and_then::<$ty, _, _>(|lhs| assert_eq!(*lhs, $rhs)).unwrap()
	};
}

#[cfg(test)]
macro_rules! args {
	() => { $crate::Args::default() };
	($($args:expr),+) => {
		$crate::Args::new(vec![$(&$args.into()),*])
	};
}


#[cfg(test)]
#[deprecated]
macro_rules! dummy_object_old {
	($vis:vis struct $obj:ident $(($($types:ty),*))?;) =>{
		dummy_object_old!($vis struct $obj $(($($types),*))?; $crate::types::Basic {});
	};

	($vis:vis struct $obj:ident $(($($types:ty),*))?; $parent:ty) =>{
		dummy_object_old!($vis struct $obj $(($($types),*))?; $parent {});
	};

	($vis:vis struct $obj:ident $(($($types:ty),*))?; { $($args:tt)* }) =>{
		dummy_object_old!($vis struct $obj $(($($types),*))?; $crate::types::Basic { $($args)* });
	};

	($vis:vis struct $obj:ident $(($($types:ty),*))?; $parent:path { $($args:tt)* }) =>{
		#[derive(Debug, Clone, Hash)]
		$vis struct $obj$(($($types),*))?;
		impl_object_type!(for $obj [(parents $parent)]: $($args)* );
	};
}

#[macro_export]
macro_rules! impl_object_type {
	(@CONVERTIBLE $_obj:ty;) => {};
	(@CONVERTIBLE $obj:ty; (convert $convert_func:expr) $($_rest:tt)*) => {
		impl $crate::types::Convertible for $obj {
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
	(@PARENT_DEFAULT (parents $parent:path) $($_rest:tt)*) => {
		<$parent as Default>::default()
	};
	(@PARENT_DEFAULT $_b:tt $($rest:tt)*) => {
		impl_object_type!(@PARENT_DEFAULT $($rest)*);
	};

	(@SET_PARENT $class:ident) => {
		compile_error!("parent should have been checked for earlier");
	};
	(@SET_PARENT $class:ident (init_parent) $($_rest:tt)*) => {};
	(@SET_PARENT $class:ident (init_parent $($init_parent:path)+) $($_rest:tt)*) => {
		$class.set_attr_lit(
			"__parents__",
			Object::from(vec![
				$(<$init_parent as $crate::types::ObjectType>::mapping()),+
			])
		);
	};
	(@SET_PARENT $class:ident (parents $parent:path) $($_rest:tt)*) => {
		impl_object_type!(@SET_PARENT $class (init_parent $parent));
	};

	(@SET_PARENT $class:ident $_b:tt $($rest:tt)*) => {
		impl_object_type!(@SET_PARENT $class $($rest)*)
	};

	(@SET_ATTRS $class:ident $obj:ty;) => {};
	(@SET_ATTRS $class:ident $obj:ty; $attr:expr => const $val:expr $(, $($args:tt)*)?) => {{
		$class.set_attr_lit($attr, Object::from($val));
		impl_object_type!(@SET_ATTRS $class $obj; $($($args)*)?);
	}};

	(@SET_ATTRS $class:ident $obj:ty; $attr:expr => function $val:expr $(, $($args:tt)*)?) => {{
		$class.set_attr_lit($attr, $crate::types::RustFn::new(
			concat!(stringify!($obj), "::", $attr), |this, args| {
				$val(this, args).map(Object::from).map_err(From::from)
			})
		);
		impl_object_type!(@SET_ATTRS $class $obj; $($($args)*)?);
	}};

	(@SET_ATTRS $class:ident $obj:ty; $attr:expr => method $val:expr $(, $($args:tt)*)?) => {{
		$class.set_attr_lit($attr, $crate::types::RustFn::new(
			concat!(stringify!($obj), "::", $attr),
			|this, args| {
				this.try_downcast_and_then::<$obj, _, _, _>(|this_data|
					$val(this_data, args, this)
						.map(Object::from)
						.map_err(From::from)
				)
			}
		));
		impl_object_type!(@SET_ATTRS $class $obj; $($($args)*)?);
	}};

	(@SET_ATTRS $class:ident $obj:ty; $attr:expr => method_old $val:expr $(, $($args:tt)*)?) => {{
		$class.set_attr_lit($attr, $crate::types::RustFn::new(
			concat!(stringify!($obj), "::", $attr),
			|this, args| {
				this.try_downcast_and_then(|this| $val(this, args)
					.map(Object::from)
					.map_err($crate::Error::from))
			}
		));
		impl_object_type!(@SET_ATTRS $class $obj; $($($args)*)?);
	}};


	(@SET_ATTRS $class:ident $obj:ty; $attr:expr => method_old_mut $val:expr $(, $($args:tt)*)?) => {{
		$class.set_attr_lit($attr, $crate::types::RustFn::new(
			concat!(stringify!($obj), "::", $attr),
			|this, args| {
				this.try_downcast_mut_and_then(|data| $val(data, args).map(Object::from).map_err($crate::Error::from))
			}
		));
		impl_object_type!(@SET_ATTRS $class $obj; $($($args)*)?);
	}};

	(@SET_ATTRS $_class:ident $_obj:ty; $($tt:tt)*) => {
		compile_error!(concat!("Bad attrs given:", stringify!($($tt)*)));
	};


	(for $obj:ty $({$new_object:item})? [ $($args:tt)* ]: $($body:tt)*/*$($attr:expr => ($($attr_val:tt)*)),* $(,)?*/) => {
		impl_object_type!(@CONVERTIBLE $obj; $($args)* );

		#[cfg(test)]
		impl_object_type!(@SETUP_INIT $($args)*);

		// don't fix any unsafety here 
		impl $crate::types::ObjectType for $obj {
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

			fn mapping() -> $crate::Object {
				use std::mem::MaybeUninit;
				use std::sync::{Once, atomic::{AtomicU8, Ordering}};
				#[allow(unused)]
				use $crate::{Object, literals};

				static mut CLASS_OBJECT: MaybeUninit<Object> = MaybeUninit::uninit();
				static mut HAS_SETUP_HAPPENED: AtomicU8 = AtomicU8::new(0);
				static HAS_CREATE_HAPPENED: Once = Once::new();

				HAS_CREATE_HAPPENED.call_once(|| unsafe {
					CLASS_OBJECT.as_mut_ptr().write(Object::new_with_parent(
						impl_object_type!(@PARENT_DEFAULT $($args)*),
						()
					));
				});

				let class = unsafe { (*CLASS_OBJECT.as_ptr()).clone() };
				
				if unsafe { HAS_SETUP_HAPPENED.compare_and_swap(0, 1, Ordering::SeqCst) } == 0 {
					#[allow(unused)]
					use $crate::{Object, types::*};
					impl_object_type!(@SET_PARENT class $($args)*);

					class.set_attr_lit("name", Object::from(stringify!($obj)));

					impl_object_type!(@SET_ATTRS class $obj; $($body)*);

					#[cfg(test)]
 					unsafe {
						impl_object_type!(@SETUP $($args)*)
 							.store(true, std::sync::atomic::Ordering::SeqCst);
 					}
				}
				class
			}

			$($new_object)?
		}
	};
}

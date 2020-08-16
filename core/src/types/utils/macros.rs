#[cfg(test)]
macro_rules! assert_call_idempotent {
	(@INTO $ty:ident) => { $ty };
	(@INTO $_ty:ident $into:ty) => { $into };

	($ty:ident::$fn:ident($this:expr $(, $args:expr)*) $( $(-> $into:ty)?, $expected:expr)?) => {
		crate::initialize();

		let old = Object::from($this);
		let new = $ty::$fn(&old, args!($($args),*)).unwrap();
		assert!(!old.is_identical(&new));
		$(
			assert_eq!(
				*old.downcast::<assert_call_idempotent!(@INTO $ty $($into)?)>().unwrap(),
				$expected)
		)?
	};
}

#[cfg(test)]
macro_rules! assert_call_non_idempotent {
	($ty:ident::$fn:ident($this:expr $(, $args:expr)*) $( $(-> $into:ty)?, $expected:expr)?) => {{
		crate::initialize();

		let old = Object::from($this);
		let new = $ty::$fn(&old, args!($($args),*)).unwrap();
		assert!(old.is_identical(&new));
		$(
			assert_eq!(
				*old.downcast::<assert_call_idempotent!(@INTO $ty $($into)?)>().unwrap(),
				$expected
			);
		)?
	}};
}

#[cfg(test)]
macro_rules! call_unwrap {
	($ty:ident::$fn:ident($this:expr $(, $args:expr)*) $(-> $ret:ty)?; $block:expr) => {{
		crate::initialize();
		#[allow(unused_imports)]
		use crate::types::*;

		$ty::$fn(&$this.into(), args!($($args),*)).unwrap()
			.downcast::<$($ret)?>().map($block)
			.unwrap()
	}};
}

#[cfg(test)]
macro_rules! call_unwrap_err {
	($ty:ident::$fn:ident($this:expr $(, $args:expr)*)) => {{
		crate::initialize();
		#[allow(unused_imports)]
		use crate::types::*;

		$ty::$fn(&$this.into(), args!($($args),*)).unwrap_err()
	}};
}

#[cfg(test)]
macro_rules! assert_call {
	($ty:ident::$fn:ident($this:expr $(, $args:expr)*) $(-> $ret:ty)?; $block:expr) => {
		assert!(call_unwrap!($ty::$fn($this $(, $args)*) $(-> $ret)?; $block));
	};
}

#[cfg(test)]
macro_rules! assert_call_err {
	($ty:ident::$fn:ident($this:expr $(, $args:expr)*), $($tt:tt)*) => {{
		assert_matches!(call_unwrap_err!($ty::$fn($this $(, $args)*)), $($tt)*)
	}};
}

#[cfg(test)]
macro_rules! assert_call_eq {
	($ty:ident::$fn:ident($this:expr $(, $args:expr)*) $(-> $ret:ty)?, $rhs:expr) => {{
		call_unwrap!($ty::$fn($this $(, $args)*) $(-> $ret)?; |lhs| {
			assert_eq!(*lhs, $rhs)
		})
	}};
}

#[cfg(test)]
macro_rules! assert_call_missing_parameter {
	($ty:ident::$fn:ident($this:expr $(, $args:expr)*), $index:expr) => {{
		crate::initialize();

		assert_matches!(
			$ty::$fn(&$this.into(), args!($($args),*)),
				Err($crate::Error::KeyError($crate::error::KeyError::MissingArgument { index: $index }))
		);
	}};
}

#[cfg(test)]
macro_rules! assert_matches {
	($lhs:expr, $($rest:tt)*) => {{
		let lhs = $lhs;
		assert!(
			matches!(lhs, $($rest)*),
			concat!("values don't match\nlhs: {:?}\npat: {}"),
			lhs,
			stringify!($($rest)*)
		);
	}};
}

#[cfg(test)]
macro_rules! args {
	() => { $crate::Args::default() };
	($($args:expr),+) => {
		$crate::Args::new(vec![$(&$args.into()),*])
	};
}

#[macro_export]
/// Create a new object type.
///
/// This is soft-deprecated.
macro_rules! impl_object_type {
	(@CONVERTIBLE $obj:ty;) => { /* TODO */ };

	// (@CONVERTIBLE $_obj:ty; (no_convert) $($_ret:tt)*) => {};
	(@CONVERTIBLE $obj:ty; (convert $convert_func:expr) $($rest:tt)*) => {
		impl $crate::types::Convertible for $obj {
			const CONVERT_FUNC: &'static str = $convert_func;
		}
		impl_object_type!(@CONVERTIBLE $obj; $($rest)*);
	};
	(@CONVERTIBLE $obj:ty; $_b:tt $($rest:tt)*) => {
		impl_object_type!(@CONVERTIBLE $obj; $($rest)*);
	};
	(@CONVERTIBLE $($tt:tt)*) => {
		compile_error!(concat!("bad CONVERTIBLE: ", stringify!($($tt)*)))
	};

	(@PARENT_DEFAULT) => { compile_error!("A parent is needed to create an object"); };
	(@PARENT_DEFAULT (parents $parent:path) $($_rest:tt)*) => { <$parent as Default>::default() };
	(@PARENT_DEFAULT $_b:tt $($rest:tt)*) => { impl_object_type!(@PARENT_DEFAULT $($rest)*); };

	(@SET_PARENT $class:ident) => { () };
	(@SET_PARENT $class:ident (init_parent) $($_rest:tt)*) => { () };
	(@SET_PARENT $class:ident (init_parent $($init_parent:path)+) $($_rest:tt)*) => {
		vec![$(<$init_parent as $crate::types::ObjectType>::mapping().clone()),+]
	};
	(@SET_PARENT $class:ident (parents $parent:path) $($_rest:tt)*) => {
		impl_object_type!(@SET_PARENT $class (init_parent $parent));
	};

	(@SET_PARENT $class:ident $_b:tt $($rest:tt)*) => {
		impl_object_type!(@SET_PARENT $class $($rest)*)
	};

	(@SET_ATTRS $class:ident $obj:ty;) => {};
	(@SET_ATTRS $class:ident $obj:ty; $attr:expr => const $val:expr $(, $($args:tt)*)?) => {{
		$class.set_attr_lit($attr, Object::from($val))?;

		impl_object_type!(@SET_ATTRS $class $obj; $($($args)*)?);
	}};

	(@SET_ATTRS $class:ident $obj:ty; $attr:expr => function $val:expr $(, $($args:tt)*)?) => {{
		$class.set_value_lit($attr, $crate::types::RustFn::new(
			concat!(stringify!($obj), "::", $attr), $val)
		)?;
		impl_object_type!(@SET_ATTRS $class $obj; $($($args)*)?);
	}};

	(@SET_ATTRS $_class:ident $_obj:ty; $($tt:tt)*) => {
		compile_error!(concat!("Bad attrs given:", stringify!($($tt)*)));
	};


	(for $obj:ty $({$new_object:item})? [ $($args:tt)* ]: $($body:tt)*) => {
		impl_object_type!(@CONVERTIBLE $obj; $($args)* );

		impl $crate::types::ObjectType for $obj {
			fn initialize() -> $crate::Result<()> {
				// `Once` wouldn't allow for returning an error.
				use ::std::sync::atomic::{AtomicBool, Ordering};

				const UNINIT: bool = false;
				const INIT: bool = true;
				static INITIALIZE: AtomicBool = AtomicBool::new(UNINIT);

				if INITIALIZE.compare_and_swap(UNINIT, INIT, Ordering::SeqCst) == INIT {
					return Ok(());
				}

				let class = Self::mapping();
				class.set_attr_lit("name", stringify!($obj).into())?;

				impl_object_type!(@SET_ATTRS class $obj; $($body)*);
				Ok(())
			}

			fn mapping() -> &'static $crate::Object {
				lazy_static::lazy_static! {
					static ref CLASS: $crate::Object = $crate::Object::new_with_parent(
						$crate::types::Class::new(stringify!($obj)),
						impl_object_type!(@SET_PARENT class $($args)*)
					);
				}

				&CLASS
			}			

			$($new_object)?
		}
	};
}

macro_rules! define_attrs {
	(for $class:ident; $($tt:tt)*) => {
		define_attrs!(for $class, attrs=__ATTRIBUTES, debug=__ATTRIBUTES_INITIALIZED; $($tt)*);
	};

	(for $class:ident, attrs=$attrs:ident, debug=$initialized:ident; $($funcs:tt)*) => {
		static mut $attrs: std::mem::MaybeUninit<$crate::value::Attributes> = std::mem::MaybeUninit::uninit();

		#[cfg(debug_assertions)]
		static $initialized: std::sync::atomic::AtomicBoolean = std::sync::atomic::AtomicBoolean::new(false);

		impl $crate::value::UnboxedValue for $class  {

		}
	}
}

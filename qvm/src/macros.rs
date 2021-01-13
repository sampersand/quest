macro_rules! rustfn_method {
	($type:ty, $name:literal, ) => {};
}
macro_rules! raise {
	($type:ident, $($fmt_args:tt)+) => {
		return ::std::result::Result::Err($crate::Error::$type(format!($($fmt_args)+)))
	};
}

macro_rules! try_downcast {
	($value:expr, $type:ty, $prefix:literal $($extra:tt)*) => {
		if let Some(inner) = $value.downcast::<$ty>() {
			inner
		} else {
			raise!(
				TypeError
				concat!(
					$prefix,
					concat!("expected '", stringify!($type), "', got {:?}."),
			));
		}
	};
}

macro_rules! expect_is_a {
	($value:expr, $type:ty $(,)?) => {{
		let value = $value;
		expect_is_a!(
			value,
			$type,
			concat!("expected a '", stringify!($type), "', got a {:?} instead."),
			value);
	}};

	($value:expr, $type:ty, $($fmt_args:tt)+) => {{
		let value = $value;
		if !value.is_a::<$type>() {
			raise!(TypeError, $($fmt_args)+);
		}
	}};
}

macro_rules! strict_arguments_check {
	($self:ident: $Self:ty) => {
		expect_is_a!($self, $Self, "invalid `self`")
		strict_arguments_check!($self: $Self)
	};
}

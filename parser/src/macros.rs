macro_rules! parse_error {
	(context=$context:expr, $type:ident $($tt:tt)*) => {
		$crate::Error::new($context, $crate::ErrorType::$type$($tt)*)
	};

	($stream:expr, $type:ident $($tt:tt)*) => {
		parse_error!(context=$crate::stream::Contexted::context($stream).clone(), $type$($tt)*)
	};
}
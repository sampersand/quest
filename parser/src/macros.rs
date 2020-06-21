macro_rules! parse_error {
	(context=$context:expr, $type:ident $($tt:tt)*) => {
		$crate::Error::new($context, $crate::ErrorType::$type$($tt)*)
	};

	($stream:expr, $type:ident $($tt:tt)*) => {
		parse_error!(context=$crate::stream::Contexted::context($stream).clone(), $type$($tt)*)
	};
}

macro_rules! try_seek {
	($stream:expr, $where:ident($val:expr)) => {{
		std::io::Seek::seek($stream, std::io::SeekFrom::$where($val))
			.map_err(|err| parse_error!($stream, CantReadStream(err)))?;
	}};
	($stream:expr, $val:expr) => {
		try_seek!($stream, Current($val));
	};
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Basic;

impl_object_type!{for Basic, super::Kernel;
	"==" => (|args| {
		args.this_any()?
			.call("__id__", args.new_args_slice(&[]))?
			.call("==", args.new_args_slice(&[
				args.get(1)?.call("__id__", args.new_args_slice(&[]))?
			]))
	}),

	"!=" => (|args| {
		args.this_any()?
			.call("==", args.get_rng(1..)?)?
			.call("!", args.new_args_slice(&[]))
	}),

	"@bool" => (|_args| {
		Ok(boolean::TRUE.into())
	}),

	"!" => (|args| {
		args.this_any()?
			.call("@bool", args.get_rng(1..)?)?
			.call("!", args.new_args_slice(&[]))
	}),

	"@text" => (|args| {
		let this = args.this_any()?;
		Ok(format!("<{}:{}>",
			this.get_attr(&"__parent__".into())?
				.get_attr(&"name".into())
				.and_then(|x| x.call("@text", args.new_args_slice(&[])))
				.unwrap_or_else(|_| "<unknown name>".into())
				.try_downcast_ref::<Text>()?
				.as_ref(),
			this.id()
		).into())
	}),

	"ancestors" => (|args| {
		todo!()
	})
}
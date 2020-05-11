#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Basic;

impl_object_type!{for Basic, super::Pristine;
	"==" => (|args| {
		args.this_any()?.call("__id__", &[])?.call("==", &[&args.get(1)?.call("__id__", &[])?])
	}),

	"!=" => (|args| {
		args.this_any()?.call("==", &args.get(1..)?)?.call("!", &[])
	}),

	"@bool" => (|_args| {
		Ok(true.into())
	}),

	"!" => (|args| {
		args.this_any()?.call("@bool", &args.get(1..)?)?.call("!", &[])
	}),

	"@text" => (|args| {
		let this = args.this_any()?;
		Ok(format!("<{}:{}>",
			this.get_attr(&"__parent__".into())?
				.get_attr(&"name".into())
				.and_then(|x| x.call("@text", &[]))
				.unwrap_or_else(|_| "<unknown name>".into())
				.try_downcast_ref::<Text>()?
				.as_ref(),
			this.0.id
		).into())
	}),

	"ancestors" => (|args| {
		todo!()
	})
}
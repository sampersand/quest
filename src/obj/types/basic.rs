#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Basic;

mod impls {
	use super::Basic;
	use crate::obj::{Object, Result, Args, types};

	pub fn at_bool(_: Args) -> Result<Object> {
		Ok(types::boolean::TRUE.into())
	}

	pub fn at_text(args: Args) -> Result<Object> {
		let this = args._this()?;
		Ok(format!("<{}:{}>",
			this.get_attr(&"__parent__".into(), args.binding())?
				.get_attr(&"name".into(), args.binding())
				.and_then(|x| x.call("@text", args.new_args_slice(&[])))
				.unwrap_or_else(|_| "<unknown name>".into())
				.try_downcast_ref::<types::Text>()?
				.as_ref(),
			this.id()
		).into())
	}

	pub fn eql(args: Args) -> Result<Object> {
		let lhs_id = args._this()?.call("__id__", args.new_args_slice(&[]))?;
		let rhs_id = args.get(1)?.call("__id__", args.new_args_slice(&[]))?;
		lhs_id.call("==", args.new_args_slice(&[rhs_id]))
	}

	pub fn neq(args: Args) -> Result<Object> {
		args._this()?
			.call("==", args.get_rng(1..)?)?
			.call("!", args.new_args_slice(&[]))
	}

	pub fn not(args: Args) -> Result<Object> {
		args._this()?
			.call("@bool", args.get_rng(1..)?)?
			.call("!", args.new_args_slice(&[]))
	}

	pub fn is_identical(args: Args) -> Result<Object> {
		// TODO: do we want the `id` here to be overridable?
		Ok((args._this()?.id() == args.get(1)?.id()).into())
	}
}

impl_object_type!{for Basic, super::Pristine;
	"@bool" => (impls::at_bool),
	"@text" => (impls::at_text),
	"=="    => (impls::eql),
	"!="    => (impls::neq),
	"!"     => (impls::not),
	"is_identical" => (impls::is_identical),
	"ancestors" => (|args| todo!()) // this is just a reminder to update `__parent__`...
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pristine;

mod impls {
	use super::Pristine;
	use crate::obj::{Object, Result, Args, types};

	pub fn __id__(args: Args) -> Result<Object> {
		Ok(types::Number::from(args.get(0)?.id()).into())
	}

	pub fn __call_attr__(args: Args) -> Result<Object> {
		args.this()?.call_attr(args.arg(0)?, args.args(1..).unwrap_or_default())
	}

	pub fn __get_attr__(args: Args) -> Result<Object> {
		args.this()?.get_attr(args.arg(0)?, args.binding())
	}

	pub fn __set_attr__(args: Args) -> Result<Object> {
		args.this()?.set_attr(args.arg(0)?.clone(), args.arg(1)?.clone(), args.binding())
	}

	pub fn __del_attr__(args: Args) -> Result<Object> {
		args.this()?.del_attr(args.arg(0)?, args.binding())
	}
}

impl_object_type!{for Pristine, Pristine,; // trailing comma here is required 
	"__id__" => (impls::__id__),
	"__call_attr__" => (impls::__call_attr__),
	"__get_attr__" => (impls::__get_attr__),
	"__set_attr__" => (impls::__set_attr__),
	"__del_attr__" => (impls::__del_attr__),
	"::" => (impls::__get_attr__),
	"." => (impls::__get_attr__),/*|args| {
		args.get(0)?.call("__get_attr__", args.get_rng(1..)?)
	}),*/
	".=" => (impls::__set_attr__)/*(|args| {
		args.get(0)?.call("__set_attr__", args.get_rng(1..)?)
	}),*/
}
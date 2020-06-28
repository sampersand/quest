#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoundFunction;

pub mod impls {
	use crate::{Object, Result, ArgsOld};

	fn parent_call_attr_old(args: ArgsOld, attr: &'static str) -> Result<Object> {
		let this = args.this()?;
		this.get_attr("__bound_object__")?.call_attr_old(attr, args.args(..)?)
	}

	pub fn get_attr(args: ArgsOld) -> Result<Object> {
		parent_call_attr_old(args, "__get_attr__")
	}

	pub fn set_attr(args: ArgsOld) -> Result<Object> {
		parent_call_attr_old(args, "__set_attr__")
	}

	pub fn del_attr(args: ArgsOld) -> Result<Object> {
		parent_call_attr_old(args, "__del_attr__")
	}

	pub fn call_attr_old(args: ArgsOld) -> Result<Object> {
		parent_call_attr_old(args, "__call_attr_old__")
	}

	pub fn call(args: ArgsOld) -> Result<Object> {
		let this = args.this()?.clone();
		let mut args = args.args(..)?;

		args.add_this(this.get_attr("__bound_object_owner__")?);

		this.get_attr("__bound_object__")?.call_attr_old("()", args)
	}
}

impl_object_type!{
for BoundFunction [(parents super::Basic)]:
	"__get_attr__" => impls::get_attr,
	"__set_attr__" => impls::set_attr,
	"__del_attr__" => impls::del_attr,
	"__call_attr_old__" => impls::call_attr_old,
	"()" => impls::call,
}






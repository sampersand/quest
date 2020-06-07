#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoundFunction;

pub mod impls {
	use super::BoundFunction;
	use crate::{Object, Result, Args, types, literals};

	fn parent_call_attr(args: Args, attr: &'static str) -> Result<Object> {
		let this = args.this()?;
		this.get_attr("__bound_object__")?.call_attr(attr, args.args(..)?)
	}

	pub fn get_attr(args: Args) -> Result<Object> {
		parent_call_attr(args, "__get_attr__")
	}

	pub fn set_attr(args: Args) -> Result<Object> {
		parent_call_attr(args, "__set_attr__")
	}

	pub fn del_attr(args: Args) -> Result<Object> {
		parent_call_attr(args, "__del_attr__")
	}

	pub fn call_attr(args: Args) -> Result<Object> {
		parent_call_attr(args, "__call_attr__")
	}

	pub fn call(args: Args) -> Result<Object> {
		let this = args.this()?.clone();
		let bound_object_owner = this.get_attr("__bound_object_owner__")?;
		let bound_object = this.get_attr("__bound_object__")?;
		let mut args = Vec::from(args.args(..)?);
		// print!("BoundObject::call(this={:?}, __bound_object_owner__={:?}", this.id(), bound_object_owner.id());
		args.insert(0, bound_object_owner);
		// println!(", args={:?}, __bound_object__={:?})", args, this.get_attr("__bound_object__")?.id());
		// args.insert(0, bound_object.clone());
		// bound_object.get_attr("()")?
			
		// bound_object.get_attr("()")?.call_attr("()", args)
		this.get_attr("__bound_object__")?
			.call_attr("()", args)
	}
}

impl_object_type!{
for BoundFunction [(parents super::Basic)]:
	"__get_attr__" => impls::get_attr,
	"__set_attr__" => impls::set_attr,
	"__del_attr__" => impls::del_attr,
	"__call_attr__" => impls::call_attr,
	"()" => impls::call,
}






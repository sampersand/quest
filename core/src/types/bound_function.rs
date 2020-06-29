#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoundFunction;

pub mod impls {
	use crate::{Object, Result, Args};

	fn parent_call_attr(this: &Object, args: Args, attr: &'static str) -> Result<Object> {
		this.get_attr("__bound_object__")?.call_attr(attr, args)
	}

	pub fn get_attr(this: &Object, args: Args) -> Result<Object> {
		parent_call_attr(this, args, "__get_attr__")
	}

	pub fn set_attr(this: &Object, args: Args) -> Result<Object> {
		parent_call_attr(this, args, "__set_attr__")
	}

	pub fn del_attr(this: &Object, args: Args) -> Result<Object> {
		parent_call_attr(this, args, "__del_attr__")
	}

	pub fn call_attr(this: &Object, args: Args) -> Result<Object> {
		parent_call_attr(this, args, "__call_attr__")
	}


	/*
	pub fn call(this: &Object, args: Args) -> Result<Object> {
		let owner = this.get_attr("__bound_object_owner__").unwrap();
		let bound_obj = this.get_attr("__bound_object__").unwrap();

		println!("owner: {:?}", owner);
		println!("bound_obj: {:?}", bound_obj);

		let x = Object::downcast_ref::<crate::types::RustFn>(&bound_obj);

		if let Some(rustfn) = x {
			rustfn.call(&owner, args)
		} else {
			drop(x);
			bound_obj.get_value("()")?.call(&owner, args)
		}
		// match bound_obj {
		// 	crate::obj::Value::RustFn(rustfn) => rustfn.call(&owner, args),
		// 	crate::obj::Value::Object(_) => bound_obj.call(&owner, args)
		// }


		// let bound_obj = this.get_attr("__bound_object__").unwrap();
		// println!("{:?}", bound_obj);
		// println!("{:?}", bound_obj.is_a::<crate::types::RustFn>());
		// let bound_call = bound_obj.get_value("()").unwrap();
		// println!("{:?}", bound_call);
		// let res = bound_call.call(&owner, args);
		// println!("{:?}", res);
		// res
			// .get_value("()")?
	}*/

	// pub fn call(args: crate::ArgsOld) -> Result<Object> {
	// 	let this = args.this()?.clone();
	// 	let mut args = args.args(..)?;

	// 	args.add_this(this.get_attr("__bound_object_owner__")?);

	// 	this.get_attr("__bound_object__")?.call_attr_old("()", args)
	// }


	pub fn call(this: &Object, args: Args) -> Result<Object> {
		let bound_owner = this.get_attr("__bound_object_owner__")?;
		let mut args = crate::ArgsOld::from(args);
		args.add_this(bound_owner);

		this.get_attr("__bound_object__")?.call_attr_old("()", args)
	}
}

impl_object_type!{
for BoundFunction [(parents super::Basic)]:
	"__get_attr__" => function impls::get_attr,
	"__set_attr__" => function impls::set_attr,
	"__del_attr__" => function impls::del_attr,
	"__call_attr__" => function impls::call_attr,
	"()" => function impls::call,
}






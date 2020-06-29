mod args;
mod args_old;
mod binding;

pub use args::Args;
pub use args_old::ArgsOld;
pub use binding::Binding;

use crate::Object;
use crate::types::Text;
use std::fmt::{self, Debug, Formatter};
// use std::any::Any;

#[derive(Clone, Copy)]
enum FnType {
	Old(fn(ArgsOld) -> crate::Result<Object>),
	New(fn(&Object, Args) -> crate::Result<Object>)
}

#[derive(Clone, Copy)]
pub struct RustFn(&'static str, FnType);

impl Debug for RustFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_tuple("RustFn").field(&self.0).finish()
	}
}

impl Eq for RustFn {}
impl PartialEq for RustFn {
	fn eq(&self, rhs: &RustFn) -> bool {
		self.0 == rhs.0
		// let eql = (self.1 as usize) == (rhs.1 as usize);
		// assert_eq!(eql, self.0 == rhs.0);
		// eql
	}
}


impl RustFn {
	#[inline]
	pub fn new(name: &'static str, func: fn(ArgsOld) -> crate::Result<Object>) -> Self {
		RustFn(name, FnType::Old(func))
	}

	pub fn method(name: &'static str, func: fn(&Object, Args) -> crate::Result<Object>) -> Self {
		RustFn(name, FnType::New(func))
	}

	#[inline]
	// eventually, we'll remove the `generic` thing.
	pub fn call(&self, obj: &Object, args: Args) -> crate::Result<Object> {
		match self.1 {
			FnType::Old(generic) => {
				let mut args = ArgsOld::from(args);
				args.add_this(obj.clone());
				generic(args)
			},
			FnType::New(method) => method(obj, args)
		}
	}

	#[inline]
	pub fn call_old(&self, args: ArgsOld) -> crate::Result<Object> {
		match self.1 {
			FnType::Old(generic) => generic(args),
			FnType::New(method) => method(args.this()?, args.args(..)?.as_ref().iter().collect())
		}
	}
}


impl From<RustFn> for Text {
	fn from(rustfn: RustFn) -> Self {
		Text::new_static(rustfn.0)
	}
}

impl RustFn {
	#[allow(non_snake_case)]
	pub fn qs___inspect__(&self, _: Args) -> Result<Text, !> {
		Ok(format!("{:?}", self).into())
	}
}

mod impls {
	use super::*;
	use crate::{Object, Result, ArgsOld};

	pub fn call(args: ArgsOld) -> Result<Object> {
		let this = args.this()?.try_downcast_ref::<RustFn>()?;
		this.call_old(args.args(..)?)
	}

	pub fn at_text(args: ArgsOld) -> Result<Object> {
		let this = args.this()?.try_downcast_ref::<RustFn>()?;
		Ok(this.0.into())
	}
}

impl_object_type!{
for RustFn [(parents super::Function)]:
	"@text" => impls::at_text,
	"__inspect__" => method RustFn::qs___inspect__,
	"()" => impls::call,
}
mod args;
mod args_old;
mod binding;

pub use args::Args;
pub use args_old::ArgsOld;
pub use binding::Binding;

use crate::Object;
use crate::types::Text;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
// use std::any::Any;

#[derive(Clone, Copy)]
pub struct RustFn(&'static str, fn(&Object, Args) -> crate::Result<Object>);

impl Debug for RustFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_tuple("RustFn").field(&self.0).finish()
	}
}

impl Eq for RustFn {}
impl PartialEq for RustFn {
	#[inline]
	fn eq(&self, rhs: &RustFn) -> bool {
		let eql = (self.1 as usize) == (rhs.1 as usize);
		debug_assert_eq!(eql, self.0 == rhs.0);
		eql
	}
}

impl Hash for RustFn {
	#[inline]
	fn hash<H: Hasher>(&self, h: &mut H) {
		(self.1 as usize).hash(h)
	}
}

impl RustFn {
	#[inline]
	pub fn new(name: &'static str, func: fn(&Object, Args) -> crate::Result<Object>) -> Self {
		RustFn(name, func)
	}

	#[inline]
	// eventually, we'll remove the `generic` thing.
	pub fn call(&self, obj: &Object, args: Args) -> crate::Result<Object> {
		(self.1)(obj, args)
	}

	#[inline]
	pub fn call_old(&self, args: ArgsOld) -> crate::Result<Object> {
		(self.1)(args.this()?, args.args(..)?.as_ref().iter().collect())
	}
}


impl From<RustFn> for Text {
	#[inline]
	fn from(rustfn: RustFn) -> Self {
		Text::new_static(rustfn.0)
	}
}

impl RustFn {
	#[allow(non_snake_case)]
	#[inline]
	pub fn qs_inspect(&self, _: Args) -> Result<Text, !> {
		Ok(format!("{:?}", self).into())
	}

	#[inline]
	pub fn qs_at_text(&self, _: Args) -> Result<Text, !> {
		Ok(Text::from(*self))
	}

	#[inline]
	pub fn qs_call(&self, args: Args) -> crate::Result<Object> {
		let caller = args.arg(0)?;
		let args = args.args(1..).unwrap_or_default();

		self.call(caller, args)
	}
}

impl_object_type!{
for RustFn [(parents super::Function)]:
	"inspect" => method_old RustFn::qs_inspect,
	"@text" => method_old RustFn::qs_at_text,
	"()" => method_old RustFn::qs_call,
}
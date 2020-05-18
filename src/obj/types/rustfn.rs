mod args;
pub use self::args::{Args, Binding};

use crate::obj::{self, Mapping, Object, types};
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};

type FnType = fn(Args) -> obj::Result<Object>;

#[derive(Clone, Copy)]
pub struct RustFn(&'static str, FnType);

impl Debug for RustFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_tuple("RustFn")
			.field(&self.0)
			.field(&(self.1 as usize as *const ()))
			.finish()
	}
}

impl Eq for RustFn {}
impl PartialEq for RustFn {
	fn eq(&self, rhs: &RustFn) -> bool {
		self.0 == rhs.0 && (self.1 as usize) == (rhs.1 as usize)
	}
}


impl RustFn {
	pub fn new(name: &'static str, n: FnType) -> Self {
		RustFn(name, n.into())
	}

	pub fn call(&self, args: Args) -> obj::Result<Object> {
		(self.1)(args)
	}
}


impl AsRef<FnType> for RustFn {
	fn as_ref(&self) -> &FnType {
		&self.1
	}
}

impl From<RustFn> for types::Text {
	fn from(rustfn: RustFn) -> Self {
		types::Text::new_static(rustfn.0)
	}
}

mod impls {
	use super::*;
	use crate::obj::{Object, Result, Args};

	pub fn call(mut args: Args) -> Result<Object> {
		let this = args.this_downcast::<RustFn>()?;
		todo!("do we want args.args(..)?");
		this.call(args)
	}

	pub fn at_text(args: Args) -> Result<Object> {
		Ok(types::Text::from(*args.this_downcast_ref::<RustFn>()?).into())
	}
}

impl_object_type!{
for RustFn [(parent super::Function)]:
	"@text" => impls::at_text,
	"()" => impls::call,
}
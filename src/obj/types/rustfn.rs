mod args;
pub use self::args::{Args, Binding};

use crate::obj::{self, Mapping, Object, types::ObjectType};
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

impl_object_type!{for RustFn, super::Function;
	"()" => (|args| {
		args._this_downcast::<RustFn>()?.call(args.get_rng(1..)?)
	}),

	"@text" => (|args| {
		Ok(args._this_downcast::<RustFn>()?.0.into())
	}),
}
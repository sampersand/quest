mod args;
mod binding;

pub use args::Args;
pub use binding::Binding;

use crate::Object;
use crate::types::Text;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
// use std::any::Any;

type Inner = for<'s, 'o> fn(&'o Object, Args<'s, 'o>) -> crate::Result<Object>;

#[derive(Clone, Copy)]
pub struct RustFn(&'static str, Inner);

impl Debug for RustFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_tuple("RustFn").field(&self.0).finish()
	}
}

impl Eq for RustFn {}
impl PartialEq for RustFn {
	#[inline]
	fn eq(&self, rhs: &Self) -> bool {
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
	pub fn new(name: &'static str, func: Inner) -> Self {
		Self(name, func)
	}

	#[inline]
	pub fn call<'o>(&self, obj: &'o Object, args: Args<'_, 'o>) -> crate::Result<Object> {
		(self.1)(obj, args)
	}
}


impl From<RustFn> for Text {
	#[inline]
	fn from(rustfn: RustFn) -> Self {
		Self::new_static(rustfn.0)
	}
}

impl RustFn {
	pub fn qs_inspect(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(format!("{:?}", *this).into())
	}

	pub fn qs_at_text(this: &Object, _: Args) -> crate::Result<Object> {
		Ok(this.try_downcast::<Self>()?
			.clone()
			.into())
	}

	pub fn qs_call(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;
		let caller = args.arg(0)?;
		let args = args.args(1..).unwrap_or_default();

		this.call(caller, args)
	}
}

impl_object_type!{
for RustFn [(parents super::Function)]:
	"inspect" => function Self::qs_inspect,
	"@text" => function Self::qs_at_text,
	"()" => function Self::qs_call,
}

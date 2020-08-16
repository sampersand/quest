mod args;
mod binding;

pub use args::Args;
pub use binding::Binding;

use crate::Object;
use crate::types::Text;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};

type Inner = for<'s, 'o> fn(&'o Object, Args<'s, 'o>) -> crate::Result<Object>;

#[derive(Clone, Copy)]
pub struct RustFn { 
	name: &'static str,
	func: Inner
}

impl Debug for RustFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_tuple("RustFn")
			.field(&self.name)
			.finish()
	}
}

impl Eq for RustFn {}
impl PartialEq for RustFn {
	#[inline]
	fn eq(&self, rhs: &Self) -> bool {
		let eql = self.func as usize == rhs.func as usize;
		debug_assert_eq!(eql, self.name == rhs.name);
		eql
	}
}

impl Hash for RustFn {
	#[inline]
	fn hash<H: Hasher>(&self, h: &mut H) {
		(self.func as usize).hash(h)
	}
}

impl RustFn {
	#[inline]
	pub fn new(name: &'static str, func: Inner) -> Self {
		Self { name, func }
	}

	#[inline]
	pub fn call<'o>(&self, obj: &'o Object, args: Args<'_, 'o>) -> crate::Result<Object> {
		(self.func)(obj, args)
	}
}


impl From<RustFn> for Text {
	#[inline]
	fn from(rustfn: RustFn) -> Self {
		Self::new_static(rustfn.name)
	}
}

impl RustFn {
	pub fn qs_inspect(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(format!("{:?}", *this).into())
	}

	pub fn qs_at_text(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok((*this).into())
	}

	pub fn qs_call(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;
		let caller = args.try_arg(0)?;
		let args = args.args(1..).unwrap_or_default();

		this.call(caller, args)
	}
}

impl_object_type! {
for RustFn [(parents super::Function)]:
	"inspect" => function Self::qs_inspect,
	"@text" => function Self::qs_at_text,
	"()" => function Self::qs_call,
}

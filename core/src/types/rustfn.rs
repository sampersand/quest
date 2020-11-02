mod args;
mod closure;
mod binding;

pub use closure::RustClosure;
pub use args::Args;
pub use binding::{Binding, StackTrace};

use crate::Object;
use crate::types::Text;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
use tracing::instrument;

#[derive(Clone, Copy)]
enum FuncType {
	Function(for<'s, 'o> fn(Args<'s, 'o>) -> crate::Result<Object>),
	Method(for<'s, 'o> fn(&'o Object, Args<'s, 'o>) -> crate::Result<Object>)
}

#[derive(Clone, Copy)]
pub struct RustFn { 
	name: &'static str,
	func: FuncType
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
		self.name == rhs.name
	}
}

impl Hash for RustFn {
	#[inline]
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.name.hash(h)
	}
}

impl RustFn {
	#[inline]
	pub fn function(name: &'static str, func: for<'s, 'o> fn(Args<'s, 'o>) -> crate::Result<Object>) -> Self {
		Self { name, func: FuncType::Function(func) }
	}

	#[inline]
	pub fn method(name: &'static str, meth: for<'s, 'o> fn(&'o Object, Args<'s, 'o>) -> crate::Result<Object>) -> Self {
		Self { name, func: FuncType::Method(meth) }
	}

	pub fn call_with_owner<'s, 'o>(&self, owner: &'o Object, mut args: Args<'s, 'o>) -> crate::Result<Object> {
		match self.func {
			FuncType::Function(func) => {
				args.prepend(owner);
				func(args)
			},
			FuncType::Method(meth) => meth(owner, args)
		}
	}

	#[inline]
	pub fn call(&self, args: Args) -> crate::Result<Object> {
		match self.func {
			FuncType::Function(func) => func(args),
			FuncType::Method(meth) => {
				let caller = args.try_arg(0)?;
				let args = args.try_args(1..).unwrap_or_default();
				meth(caller, args)
			}
		}
	}
}


impl From<RustFn> for Text {
	#[inline]
	fn from(rustfn: RustFn) -> Self {
		Self::const_new(rustfn.name)
	}
}

impl RustFn {
	#[instrument(name="RustFn::inspect", level="trace", skip(this), fields(self=?this))]
	pub fn qs_inspect(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(format!("{:?}", *this).into())
	}

	#[instrument(name="RustFn::@text", level="trace", skip(this), fields(self=?this))]
	pub fn qs_at_text(this: &Object, _: Args) -> crate::Result<Object> {
		Ok(this.try_downcast::<Self>()?
			.clone()
			.into())
	}

	#[instrument(name="RustFn::()", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_call(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;
		this.call(args)
	}
}

impl_object_type! {
for RustFn [(parents super::Function)]:
	"inspect" => method Self::qs_inspect,
	"@text" => method Self::qs_at_text,
	"()" => method Self::qs_call,
}

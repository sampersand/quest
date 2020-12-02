use crate::{Object, Result, Args, Literal};
use std::sync::Arc;
use crate::types::{RustClosure, List};

use std::fmt::{self, Debug, Formatter};
use tracing::instrument;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Function;

#[derive(Clone)]
pub struct BoundRustFn(Arc<dyn Fn(Args) -> crate::Result<Object> + Send + Sync>);

impl Debug for BoundRustFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_tuple("BoundRustFn").field(&"<fn>").finish()
	}
}

impl_object_type!{
for BoundRustFn [(parents super::Function)]:
	"()" => method |this: &Object, args: Args| {
		let this = this.try_downcast::<Self>()?;
		(this.0)(args)
	}
}

impl BoundRustFn {
	pub fn new<F: 'static>(func: F) -> Self
	where
		F: Fn(Args) -> crate::Result<Object> + Send + Sync
	{
		Self(Arc::new(func))
	}
}

impl Function {
	pub fn curry(this: Object, rhs: Object) -> Object {
		RustClosure::new(move |args| {
			let mut args = args.shorten();
			args.prepend(&rhs);
			this.call_attr_lit(&Literal::CALL, args)
		}).into()
	}
}

impl Function {
	#[instrument(name="Function::<<", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_lsh(this: &Object, args: Args) -> Result<Object> {
		let this = this.clone();
		let rhs = args.try_arg(0)?.clone();

		Ok(Self::curry(this, rhs))
	}

	#[instrument(name="Function::>>", level="trace", skip(this, args), fields(self=?this, args=?args))]
	pub fn qs_rsh(this: &Object, args: Args) -> Result<Object> {
		let this = this.clone();
		let rhs = args.try_arg(0)?.clone();

		Ok(Self::curry(rhs, this))
	}

	#[instrument(name="Function::apply", level="trace", skip(this, args), fields(self=?this, args=?args))]
	pub fn qs_apply(this: &Object, args: Args) -> Result<Object> {
		let this = this.clone();
		let args = args.try_arg(0)?.call_downcast::<List>()?;

		this.call_attr_lit(&Literal::CALL, args.iter().collect::<Args>())
	}
}

impl_object_type!{
for Function [(parents super::Basic)]:
	"<<" => method Self::qs_lsh,
	">>" => method Self::qs_rsh,
	"apply" => method Self::qs_apply,
	"__should_be_bound__" => const true
}

mod tests {
	#[test]
	#[ignore]
	fn lsh() { todo!() }

	#[test]
	#[ignore]
	fn rsh() { todo!() }
}

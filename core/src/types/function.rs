use crate::{Object, Result, Args, Literal, types::RustClosure};
use std::sync::Arc;
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
	"()" => function |this: &Object, args: Args| {
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
}

impl_object_type!{
for Function [(parents super::Basic)]:
	"<<" => function Self::qs_lsh,
	">>" => function Self::qs_rsh,
}

mod tests {
	#[test]
	#[ignore]
	fn lsh() { todo!() }

	#[test]
	#[ignore]
	fn rsh() { todo!() }
}

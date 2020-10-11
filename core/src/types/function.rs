use crate::{Object, Result, Args};
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
	#[instrument(name="Function::<<", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_lsh(this: &Object, args: Args) -> Result<Object> {
		let this = this.clone();
		let args = args.into_inner().into_owned().into_iter().map(Clone::clone).collect::<Vec<_>>();

		Ok(BoundRustFn::new(move |new_args| {
			let mut args = args.clone();
			args.extend(new_args.into_iter().map(Clone::clone));

			this.call_attr_lit(&crate::Literal::CALL, args.iter().collect::<Vec<_>>())
		}).into())
	}

	#[instrument(name="Function::>>", level="trace", skip(_this, _args), fields(self=?_this, args=?_args))]
	pub fn qs_rsh(_this: &Object, _args: Args) -> Result<Object> {
		todo!(">>")
	}

	#[instrument(name="Function::curry", level="trace", skip(_this, _args), fields(self=?_this, args=?_args))]
	pub fn qs_curry(_this: &Object, _args: Args) -> Result<Object> {
		todo!("curry")
	}
}

impl_object_type!{
for Function [(parents super::Basic)]:
	"<<" => function Self::qs_lsh,
	">>" => function Self::qs_rsh,
	"curry" => function Self::qs_curry
}

mod tests {
	#[test]
	#[ignore]
	fn lsh() { todo!() }

	#[test]
	#[ignore]
	fn rsh() { todo!() }

	#[test]
	#[ignore]
	fn curry() { todo!() }
}

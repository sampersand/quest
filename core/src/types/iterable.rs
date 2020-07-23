use crate::{Object, Args};
use std::fmt::{self, Debug, Formatter};
use std::sync::Arc;
use parking_lot::Mutex;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Iterable;

#[derive(Clone)]
pub struct BoundRustFn(Arc<dyn Fn(Object) -> crate::Result<()> + Send + Sync>);

impl Debug for BoundRustFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_tuple("BoundRustFn").field(&"<fn>").finish()
	}
}

impl Eq for BoundRustFn {}
impl PartialEq for BoundRustFn {
	fn eq(&self, rhs: &Self) -> bool {
		Arc::ptr_eq(&self.0, &rhs.0)
	}
}

impl_object_type!{
for BoundRustFn [(parents super::Basic)]:
	"()" => function |this: &Object, args: Args| {
		let arg = args.arg(0)?.clone();

		this.try_downcast_and_then(|this: &Self| (this.0)(arg))
			.map(|_| this.clone())
	}
}

fn foreach<F>(this: &Object, block: Object, f: F) -> crate::Result<()>
where
	F: Fn(Object) + Send + Sync + 'static
{
	let bound =
		BoundRustFn(Arc::new(Box::new(move |obj| {
			f(block.call_attr_lit("()", &[&obj])?);
			Ok(())
		})));

	let each = this.get_attr_lit("each")?;
	each.call_attr_lit("()", &[this, &Object::from(bound)])?;

	Ok(())
}

impl Iterable {
	pub fn qs_map(this: &Object, args: Args) -> crate::Result<Object> {
		let block = args.arg(0)?;

		let ret = Arc::new(Mutex::new(vec![]));
		let ret2 = ret.clone();
		foreach(this, block.clone(), move |obj| ret2.lock().push(obj))?;

		match Arc::try_unwrap(ret) {
			// no one else has a refrence, so we're all good.
			Ok(mutex) => Ok(mutex.into_inner().into()),
			// we have to clone it now. darn!
			Err(arc) => {
				println!("having to clone");
				Ok(arc.lock().clone().into())
			}
		}
	}
}


impl_object_type!{
for Iterable [(parents super::Basic)]:
	"map" => function Self::qs_map,
}
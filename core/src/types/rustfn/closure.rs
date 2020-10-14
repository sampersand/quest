use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use crate::{Object, Args};
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
use tracing::instrument;

#[derive(Clone)]
pub struct RustClosure { 
	func: Arc<dyn Fn(Args) -> crate::Result<Object> + Send + Sync>,
	id: usize
}


impl Debug for RustClosure {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_tuple("RustClosure")
			.field(&self.id)
			.finish()
	}
}

impl Eq for RustClosure {}
impl PartialEq for RustClosure {
	#[inline]
	fn eq(&self, rhs: &Self) -> bool {
		self.id == rhs.id
	}
}

impl Hash for RustClosure {
	#[inline]
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.id.hash(h)
	}
}


impl RustClosure {
	pub fn new(func: impl Fn(Args) -> crate::Result<Object> + Send + Sync + 'static) -> Self {
		static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
		Self {
			func: Arc::new(func),
			id: NEXT_ID.fetch_add(1, Ordering::Relaxed)
		}
	}

	#[inline]
	pub fn call(&self, args: Args) -> crate::Result<Object> {
		(self.func)(args)
	}
}


impl RustClosure {
	#[instrument(name="RustClosure::inspect", level="trace", skip(this), fields(self=?this))]
	pub fn qs_inspect(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(format!("{:?}", *this).into())
	}

	#[instrument(name="RustClosure::()", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_call(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		this.call(args)
	}
}

impl_object_type! {
for RustClosure [(parents super::super::Function)]:
	"inspect" => function Self::qs_inspect,
	"()" => function Self::qs_call,
}

use crate::obj::{DataEnum, Mapping, Result, Object, types::ObjectType};
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};

type InternalRepr = fn(&Object, &[Object]) -> Result;

pub struct RustFn(&'static str, InternalRepr);

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
	pub fn new(name: &'static str, n: InternalRepr) -> Self {
		RustFn(name, n.into())
	}
}

impl From<RustFn> for DataEnum {
	fn from(this: RustFn) -> DataEnum {
		DataEnum::RustFn(this)
	}
}

impl ObjectType for RustFn {
	fn mapping() -> Arc<RwLock<Mapping>> {
		// use std::sync::Once;
		static MAPPING: Mapping = {
		let mut m = Mapping::new(None);
		m.insert(
			"()".to_owned().into(),
			RustFn::new("()", (|x, a| panic!())).into());//x.x.call("clone", &[]))).into());

		// m.insert(
		// 	super::Text::new("+").into(),
		// 	super::RustFn::new("+",
		// 		(|x, y| Ok(x.clone()))
		// 	).into()
		// );
		Arc::new(RwLock::new(m))
		// m.insert()
		// };

		// MAPPING
	
	}
}
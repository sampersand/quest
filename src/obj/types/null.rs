use crate::obj::{DataEnum, Mapping, types::ObjectType};
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Null;

impl Null {
	pub fn new() -> Self {
		Null
	}
}

impl From<Null> for DataEnum {
	fn from(this: Null) -> DataEnum {
		DataEnum::Null(this)
	}
}

impl ObjectType for Null {
	fn mapping() -> Arc<RwLock<Mapping>> {
		// use std::sync::Once;
		// static MAPPING: Mapping = {
		Arc::new(RwLock::new(Mapping::new(None)))
		// m.insert()
		// };

		// MAPPING
	}
}
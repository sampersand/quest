use crate::Value;
use std::fmt::Debug;
use std::any::{Any, TypeId};
use crate::value::allocated::AllocatedType;

#[derive(Debug)]
pub struct Custom {
	parent: Value,
	data: Box<dyn CustomType>,
}

pub trait CustomType : Debug + Any + 'static {
	fn parent(&self) -> Value;
	fn try_clone(&self) -> crate::Result<Box<dyn CustomType>>;
}

unsafe impl<T: CustomType> AllocatedType for T {}

impl<T: Debug + Clone + Any + 'static> CustomType for T {
	fn parent(&self) -> Value {
		todo!();
	}

	fn try_clone(&self) -> crate::Result<Box<dyn CustomType>> {
		Ok(Box::new(self.clone()))
	}
}

impl Custom {
	pub fn new<T: CustomType>(data: T) -> Self {
		Self {
			parent: data.parent(),
			data: Box::new(data)
		}
	}

	pub fn is_a<T: 'static>(&self) -> bool {
		TypeId::of::<T>() == self.data.type_id()
	}

	pub fn try_clone(&self) -> crate::Result<Custom> {
		let data = self.data.try_clone()?; // run `data` first, as it's (probably) more likely to fail.
		let parent = self.parent.try_clone()?;

		Ok(Self { parent, data })
	}
}

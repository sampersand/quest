use crate::{Value, LMap};
use std::fmt::Debug;
use std::any::{Any, TypeId};
use crate::value::allocated::AllocatedType;

#[derive(Debug)]
pub struct Object {
	parents: Vec<Value>,
	attrs: crate::LMap,
	data: Box<dyn QuestObject>,
}

pub trait QuestObject : Debug + Any {
	fn parents(&self) -> Vec<Value>;
	fn try_clone(&self) -> crate::Result<Box<dyn QuestObject>>;
}

impl Object {
	pub fn new<T: QuestObject>(data: T) -> Self {
		Self {
			parents: data.parents(),
			attrs: LMap::default(),
			data: Box::new(data)
		}
	}

	pub fn is_a<T: 'static>(&self) -> bool {
		TypeId::of::<T>() == self.data.type_id()
	}

	// #[inline]
	// pub fn try_into<T: QuestObject>(self) -> Result<T, Self> {
	// 	if self.is_a::<T>() {
	// 		Ok((self.data as Box<dyn Any>).downcast::<T>().expect("`is_a` and `downcast` failed."))
	// 	} else {
	// 		Err(self)
	// 	}

	pub fn downcast_ref<T: QuestObject>(&self) -> Option<&T> {
		(*self.data as dyn Any).downcast_ref()
	}

	// #[inline]
	// pub fn try_as_ref<T: QuestValueRef>(&self) -> Option<&T> {
	// 	T::try_value_as_ref(self)
	// }

	// #[inline]
	// pub fn try_as_mut<T: QuestValueRef>(&mut self) -> Option<&mut T> {
	// 	T::try_value_as_mut(self)
	// }
}

// pub trait QuestObject : Debug + Any + 'static {
// 	fn parent(&self) -> Value;
// 	fn try_clone(&self) -> crate::Result<Box<dyn QuestObject>>;
// }

// unsafe impl<T: QuestObject> AllocatedType for T {}

// impl<T: Debug + Clone + Any + 'static> QuestObject for T {
// 	fn parent(&self) -> Value {
// 		todo!();
// 	}

// 	fn try_clone(&self) -> crate::Result<Box<dyn QuestObject>> {
// 		Ok(Box::new(self.clone()))
// 	}
// }

// impl Object {
// 	pub fn with_data
// 	pub fn new<T: QuestObject>(data: T,) -> Self {
// 		Self {
// 			parent: data.parent(),
// 			data: Box::new(data)
// 		}
// 	}

// 	pub fn is_a<T: 'static>(&self) -> bool {
// 		TypeId::of::<T>() == self.data.type_id()
// 	}

// 	pub fn try_clone(&self) -> crate::Result<Object> {
// 		let data = self.data.try_clone()?; // run `data` first, as it's (probably) more likely to fail.
// 		let parent = self.parent.try_clone()?;

// 		Ok(Self { parent, data })
// 	}
// }

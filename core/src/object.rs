#![allow(unused)]

use std::convert::TryFrom;
use crate::types;

#[derive(Debug)]
pub struct Object(Inner);

impl Object {
	pub fn new(data: impl Into<Self>) -> Self {
		data.into()
	}

	pub fn downcast_mut<'a, T: TryFrom<&'a mut Self>>(&'a mut self) -> Result<T, T::Error> {
		T::try_from(self)
	}
}
auto trait Foo {}

#[derive(Debug)]
pub struct Object(Inner);

#[derive(Debug)]
enum Inner {
	Primitive(Primitive),
	WithVTable(Box<dyn std::any::Any>, VTable)
}

#[derive(Debug, Clone, Copy)]
enum Primitive {
	Null(types::Null),
	Boolean(types::Boolean),
	Number(types::Number),
	Class(types::Class),
	RustFn(types::RustFn),
}

#[derive(Debug)]
struct VTable {
	data: HashMap<Object, Object>
}

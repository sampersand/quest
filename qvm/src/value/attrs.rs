use crate::{Literal, Value};

// for a total of 8192 ubiltins for each type
const NUM_BUCKETS: usize = u8::MAX as usize;

#[derive(Debug)]
pub struct BuiltinAttributes(Inner) {
	attrs: [&'static mut [(Literal, Value)]; NUM_BUCKETS]
}

impl BuiltinAttributes {
	pub const fn new() -> Self {
		Self { attrs: [&mut []; NUM_BUCKETS] }
	}

	#[inline]
	fn bucket(&self, literal: Literal) -> &[(Literal, Value)] {
		&*self.attrs[literal.bits() as usize & NUM_BUCKETS]
	}

	#[inline]
	fn bucket_mut(&self, literal: Literal) -> &mut [(Literal, Value)] {
		self.attrs[literal.bits() as usize & NUM_BUCKETS]
	}

	pub const fn register(&mut self, literal: Literal, value: Value) -> Option<Value> {
		for attr in self.bucket_mut(literal)
		for bucket in self.attrs {
			for attr in 
		}
	}
}

macro_rules! define_attrs {
	(for $ty:ty; $($rest:tt)*) => { define_attrs!(for $ty where attrs=_attrs; $($rest)*); };
	(for $ty:ty where attrs=$mod:ident; $($name:ident => $attr:expr),*) => {
		mod $mod {
			use super::*;
			use crate::{Literal, Value};
		}
	};
}

define_attrs!(for (););

// mod attrs {
// 	use crate::{Literal, Value};

// 	pub const NUM_BUCKETS: usize = u8::MAX as usize;
// 	pub static mut ATTRIBUTES: [&[(Literal, Value)]; NBUCKETS] = [&[]; NBUCKETS];
// }

// impl crate::value::HasAttrs for Boolean {
// 	fn get_attr(&self, literal: Literal) -> Option<Value> {
// 		for (lit, value) in attrs::ATTRIBUTES[(literal.bits() as usize) & attrs::NBUCKETS] {
// 			if *lit == literal {
// 				return Some(*value);
// 			}
// 		}

// 		None
// 	}

// 	fn del_attr(&mut self, _: Literal) -> Option<Value> { todo!() }
// 	fn set_attr(&mut self, _: Literal, _: Value) { todo!() }
// }

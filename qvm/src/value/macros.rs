macro_rules! define_attrs {
	(for $ty:ty; $($rest:tt)*) => { define_attrs!(for $ty where attrs=_attrs; $($rest)*); };
	(for $ty:ty where attrs=$mod:ident; $($name:ident => $attr:expr),*) => {
		mod $mod {
			use super::*;
			use crate::{Literal, Value};
		}
	};
}

// define_attrs!(for (););

// mod attrs {
// 	use crate::{Literal, Value};

// 	pub const NBUCKETS: usize = u8::MAX as usize;
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

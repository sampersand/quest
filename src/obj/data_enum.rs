use crate::obj::types::*;
use std::fmt::{self, Debug, Formatter};

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum DataEnum {
	Empty,
	Null,
	Boolean(Boolean),
	Number(Number),
	RustFn(RustFn),
	Text(Text)
}

// impl Debug for DataEnum {
// 	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
// 		if f.alternate() {
// 			match self {
// 				Self::Empty => write!(f, "DataEnum()"),
// 				Self::Boolean(ref bol) => write!(f, "DataEnum({:#?})", bol),
// 				Self::Number(ref num) => write!(f, "DataEnum({:#?})", num),
// 				Self::RustFn(ref rfn) => write!(f, "DataEnum({:#?})", rfn),
// 				Self::Text(ref txt) => write!(f, "DataEnum({:#?})", txt),
// 				_ => unimplemented!()
// 			}
// 		} else {
// 			match self {
// 				Self::Empty => Debug::fmt("()", f),
// 				Self::Boolean(ref bol) => Debug::fmt(bol, f),
// 				Self::Number(ref num) => Debug::fmt(num, f),
// 				Self::RustFn(ref rfn) => Debug::fmt(rfn, f),
// 				Self::Text(ref txt) => Debug::fmt(txt, f),
// 				_ => unimplemented!()
// 			}
// 		}
// 		// if f.alternate() && let Null = self {
// 		// 	f.debug_struct("DataEnum").field(&self)
// 		// }
// 	}
// }
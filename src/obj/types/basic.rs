use crate::obj::{DataEnum, Mapping, types::ObjectType};
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Formatter};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Basic;

impl Basic {
	pub fn new() -> Self {
		Basic
	}
}

impl From<Basic> for DataEnum {
	fn from(_: Basic) -> Self {
		Self::Empty
	}
}


impl_object_type!{for Basic, super::Pristine;
	"==" => (|args| {
		let ref lhs_id = args.get(0)?.call("__id__", &[])?;
		let ref rhs_id = args.get(1)?.call("__id__", &[])?;
		lhs_id.call("==", &[rhs_id])
	}),

	"!=" => (|args| {
		args.get(0)?.call("==", &args.get(1..)?)?.call("!", &[])
	}),

	"@bool" => (|_args| {
		Ok(true.into())
	}),

	"!" => (|args| {
		args.get(0)?.call("@bool", &args.get(1..)?)?.call("!", &[])
	}),

	"ancestors" => (|args| {
		unimplemented!()
	})
}









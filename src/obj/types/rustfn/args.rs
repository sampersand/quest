use std::slice::SliceIndex;
use crate::obj::Object;
// use std::any::{Any, TypeId};

#[derive(Debug)]
pub struct Args<'a>(Vec<&'a Object>);

impl<'a> Args<'a> {
	pub fn new<T: Into<Vec<&'a Object>>>(args: T) -> Args<'a> {
		Args(args.into())
	}

	pub fn get<'b, I: SliceIndex<[&'a Object]> + 'b>(&'b self, idx: I) -> Result<&I::Output, Object> {
		self.0.get(idx)
			.ok_or_else(|| format!("index is invalid (len={})", self.0.len()).into())
	}

	// pub fn get_as<T>()
}

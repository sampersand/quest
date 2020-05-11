use std::slice::SliceIndex;
use crate::obj::{self, Object};
use std::any::Any;

#[derive(Debug)]
pub struct Args<'a>(Vec<&'a Object>);

impl<'a> Args<'a> {
	pub fn new<T: Into<Vec<&'a Object>>>(args: T) -> Args<'a> {
		Args(args.into())
	}

	pub fn get<'b, I: SliceIndex<[&'a Object]> + 'b>(&'b self, idx: I) -> obj::Result<&I::Output> {
		self.0.get(idx).ok_or_else(|| format!("index is invalid (len={})", self.0.len()).into())
	}

	pub fn get_downcast<'b, T: std::any::Any>(&'b self, index: usize) -> obj::Result<impl std::ops::Deref<Target=T> + 'b> {
		self.0.get(index)
			.ok_or_else(|| format!("index is invalid (len={})", self.0.len()).into())
			.and_then(|thing| thing.try_downcast_ref::<T>())
	}

	pub fn this_any(&self) -> obj::Result<&Object> {
		let ret = self.get(0);
		assert!(ret.is_ok(), "invalid index given");
		ret.map(|x| *x)
	}

	pub fn this_obj<T: Any>(&self) -> obj::Result<&Object> {
		let ret = self.this_any()?;
		assert!(ret.is_type::<T>(), "invalid this encountered");
		Ok(ret)
	}

	pub fn this<'b, T: Any>(&'b self) -> obj::Result<impl std::ops::Deref<Target=T> + 'b> {
		let ret = self.get_downcast(0);
		assert!(ret.is_ok(), "invalid this encountered");
		ret
	}
}

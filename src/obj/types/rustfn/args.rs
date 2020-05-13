use std::slice::SliceIndex;
use crate::obj::{self, Object};
use std::any::Any;
use std::borrow::Cow;

#[derive(Debug, Default)]
pub struct Args<'s, 'o: 's> {
	// this: Option<&'o Object>,
	args: Cow<'s, [&'o Object]>
}

impl<'o> Args<'_, 'o> {
	pub fn new(args: Vec<&'o Object>) -> Args<'_, 'o> {
		Args {
			// this: None,
			args: Cow::Owned(args)
		}
	}
}

impl<'s, 'o> Args<'s, 'o> {
	pub fn new_shared(args: &'s [&'o Object]) -> Args<'s, 'o> {
		Args {
			// this: None,
			args: Cow::Borrowed(args)
		}
	}
}

impl<'s, 'o: 's> From<&'s [&'o Object]> for Args<'s, 'o> {
	fn from(args: &'s [&'o Object]) -> Self {
		Args::new_shared(args)
	}
}

macro_rules! impl_from {
	($($n:tt)*) => {
		$(
			impl<'s, 'o: 's> From<&'s [&'o Object; $n]> for Args<'s, 'o> {
				fn from(args: &'s [&'o Object; $n]) -> Self {
					Args::new_shared(args)
				}
			}
		)*
	};
}

impl_from!(0 1 2 3 4 5 6); // we're not going to pass more than 6 arguments...; if we do, just cast.

impl<'o> AsRef<[&'o Object]> for Args<'_, 'o> {
	fn as_ref(&self) -> &[&'o Object] {
		self.args.as_ref()
	}
}


impl<'o> Args<'_, 'o> {
	// pub fn bind(&mut self, this: &'o Object) {
	// 	if let Some(x) = self.this.take() {
	// 		println!("a `this` existed before: {:?}", x);
	// 		// self.args.to_mut().insert(0, x);
	// 	}
	// 	self.args.to_mut().insert(0, this);

	// 	self.this = Some(this);
	// }

	pub fn get_rng<'c, I>(&'c self, idx: I) -> obj::Result<Args<'_, 'o>>
	where
		I: SliceIndex<[&'o Object], Output=[&'o Object]> + 'c
	{
		if let Some(rng) = self.args.get(idx) {
			Ok(Args::from(rng))
		} else {
			Err(format!("index is invalid (len={})", self.args.len()).into())
		}
	}

	pub fn get<'c>(&'c self, idx: usize) -> obj::Result<&'c Object> {
		if let Some(obj) = self.args.get(idx) {
			Ok(*obj)
		} else {
			Err(format!("index is invalid (len={})", self.args.len()).into())
		}
	}

	pub fn get_downcast<'c, T: Any>(&'c self, index: usize) -> obj::Result<impl std::ops::Deref<Target=T> + 'c> {
		self.args.get(index)
			.ok_or_else(|| format!("index is invalid (len={})", self.args.len()).into())
			.and_then(|thing| thing.try_downcast_ref::<T>())
	}

	pub fn this_any<'c>(&'c self) -> obj::Result<&'c Object> {
		let ret = self.get(0);
		assert!(ret.is_ok(), "invalid index given");
		ret
	}

	pub fn this_obj<'c, T: Any>(&'c self) -> obj::Result<&'c Object> {
		let ret = self.this_any()?;
		assert!(ret.is_type::<T>(), "invalid this encountered");
		Ok(ret)
	}

	pub fn this<'c, T: Any>(&'c self) -> obj::Result<impl std::ops::Deref<Target=T> + 'c> {
		let ret = self.get_downcast(0);
		assert!(ret.is_ok(), "invalid this encountered");
		ret
	}
}











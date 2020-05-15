use std::slice::SliceIndex;
use crate::obj::{self, Object};
use std::any::Any;
use std::borrow::Cow;

pub type Binding = Object;

#[derive(Debug, Clone, Default)]
pub struct Args<'s> {
	binding: Binding,
	args: Cow<'s, [Object]>
}

impl<'s> Args<'s> {
	pub fn _new<V: Into<Cow<'s, [Object]>>>(args: V) -> Self { 
		Args::new(args, Default::default())
	}

	pub fn new<V: Into<Cow<'s, [Object]>>>(args: V, binding: Binding) -> Self {
		Args { args: args.into(), binding }
	}

	pub fn new_args_slice<'a>(&self, args: &'a [Object]) -> Args<'a> {
		Args { args: args.into(), binding: self.binding.clone() }
	}


	pub fn add_this(&mut self, this: Object)  {
		self.args.to_mut().insert(0, this);
	}

	pub fn binding(&self) -> &Binding {
		&self.binding
	}
}

// impl<'s, 'o: 's> From<&'s [&'o Object]> for Args<'s, 'o> {
// 	fn from(args: &'s [&'o Object]) -> Self {
// 		Args::_new(args)
// 	}
// }

// macro_rules! impl_from {
// 	($($n:tt)*) => {
// 		$(
// 			impl<'s, 'o: 's> From<&'s [&'o Object; $n]> for Args<'s, 'o> {
// 				fn from(args: &'s [&'o Object; $n]) -> Self {
// 					Args::_new_shared(args)
// 				}
// 			}
// 		)*
// 	};
// }

// impl_from!(0 1 2 3 4 5 6); // we're not going to pass more than 6 arguments...; if we do, just cast.

impl AsRef<[Object]> for Args<'_> {
	fn as_ref(&self) -> &[Object] {
		self.args.as_ref()
	}
}


impl Args<'_> {
	// pub fn bind(&mut self, this: &'o Object) {
	// 	if let Some(x) = self.this.take() {
	// 		println!("a `this` existed before: {:?}", x);
	// 		// self.args.to_mut().insert(0, x);
	// 	}
	// 	self.args.to_mut().insert(0, this);

	// 	self.this = Some(this);
	// }

	pub fn get_rng<'c, I>(&'c self, idx: I) -> obj::Result<Args<'c>>
	where I: SliceIndex<[Object], Output=[Object]> + 'c
	{
		if let Some(rng) = self.args.get(idx) {
			Ok(Args::_new(rng))
		} else {
			Err(format!("index is invalid (len={})", self.args.len()).into())
		}
	}

	pub fn get(&self, idx: usize) -> obj::Result<Object> {
		if let Some(obj) = self.args.get(idx) {
			Ok(obj.clone())
		} else {
			Err(format!("index is invalid (len={})", self.args.len()).into())
		}
	}

	pub fn get_downcast<'c, T: Any>(&'c self, index: usize) -> obj::Result<impl std::ops::Deref<Target=T> + 'c> {
		self.args.get(index)
			.ok_or_else(|| format!("index is invalid (len={})", self.args.len()).into())
			.and_then(|thing| thing.try_downcast_ref::<T>())
	}

	pub fn this_any(&self) -> obj::Result<Object> {
		let ret = self.get(0);
		assert!(ret.is_ok(), "invalid index given");
		ret
	}

	pub fn this_obj<T: Any>(&self) -> obj::Result<Object> {
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











use std::slice::SliceIndex;
use crate::{Result, Object, types::{self, Convertible}};
use std::any::Any;
use std::borrow::Cow;
use std::ops::Deref;

use crate::types::rustfn::Binding;

#[derive(Clone, Default)]
pub struct Args<'s> {
	binding: Option<Binding>,
	args: Cow<'s, [Object]>
}
use std::fmt::{self, Debug, Formatter};
impl Debug for Args<'_> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(&self.args, f)
	}
}

impl<'s> Args<'s> {
	pub fn new<V: Into<Cow<'s, [Object]>>>(args: V) -> Self {
		Args { args: args.into(), binding: Binding::try_instance() }
	}

	pub fn new_slice(args: &'s [Object]) -> Self {
		Args { args: args.into(), binding: Binding::try_instance() }
	}

	pub fn new_args_slice<'a>(&self, args: &'a [Object]) -> Args<'a> {
		Args { args: args.into(), binding: self.binding.clone() }
	}

	pub fn add_this(&mut self, this: Object)  {
		self.args.to_mut().insert(0, this);
	}

	pub fn binding(&self) -> Option<&Binding> {
		self.binding.as_ref()
	}
}

impl From<Args<'_>> for Vec<Object> {
	fn from(args: Args) -> Self {
		args.args.to_vec()
	}
}

impl From<Vec<Object>> for Args<'static> {
	fn from(args: Vec<Object>) -> Self {
		Args::new(args)
	}
}

impl AsRef<[Object]> for Args<'_> {
	fn as_ref(&self) -> &[Object] {
		self.args.as_ref()
	}
}

impl From<Args<'_>> for types::List {
	fn from(args: Args<'_>) -> Self {
		types::List::from(args.args.to_vec())
	}
}

impl Args<'_> {
	pub fn arg<'s>(&'s self, index: usize) -> Result<&'s Object> {
		self.args.get(index + 1)
			.ok_or_else(|| format!("index `{}' is too big (args={:?})", index + 1, self).into())
	}	

	pub fn args<'c, I>(&'c self, idx: I) -> Result<Args<'c>>
	where
		I: SliceIndex<[Object], Output=[Object]> + 'c
	{
		if let Some(rng) = self.args.get(1..).and_then(|args| args.get(idx)) {
			Ok(self.new_args_slice(rng))
		} else {
			Err(format!("index is invalid (len={})", self.args.len()).into())
		}
	}


	pub fn this<'s>(&'s self) -> Result<&'s Object> {
		self.args.get(0)
			.ok_or_else(|| format!("no `this` supplied (args={:?})", self).into())
	}
}


/*

impl Args<'_> {
	#[deprecated]
	pub fn get_rng<'c, I>(&'c self, idx: I) -> obj::Result<Args<'c>>
	where I: SliceIndex<[Object], Output=[Object]> + 'c
	{
		if let Some(rng) = self.args.get(idx) {
			Ok(self.new_args_slice(rng))
		} else {
			Err(format!("index is invalid (len={})", self.args.len()).into())
		}
	}

	#[deprecated]
	pub fn get(&self, idx: usize) -> obj::Result<Object> {
		if let Some(obj) = self.args.get(idx) {
			Ok(obj.clone())
		} else {
			Err(format!("index is invalid (len={})", self.args.len()).into())
		}
	}

	#[deprecated]
	pub fn get_downcast<'c, T: Any>(&'c self, index: usize) -> obj::Result<impl std::ops::Deref<Target=T> + 'c> {
		self.args.get(index)
			.ok_or_else(|| format!("index is invalid (len={})", self.args.len()).into())
			.and_then(|thing| thing.try_downcast_ref::<T>())
	}

	pub fn _this(&self) -> obj::Result<Object> {
		let ret = self.get(0);
		debug_assert!(ret.is_ok(), "invalid index given");
		ret
	}

	pub fn _this_obj<T: Any>(&self) -> obj::Result<Object> {
		let ret = self._this()?;
		assert!(ret.is_a::<T>(), "invalid this encountered");
		Ok(ret)
	}

	pub fn _this_downcast_ref<'c, T: Any>(&'c self) -> obj::Result<impl std::ops::Deref<Target=T> + 'c> {
		self.this_downcast_ref()
		// let ret = self.get_downcast(0);
		// debug_assert!(ret.is_ok(), "invalid `this` encountered: {:?}, {:?}", self, ret.map(|x| {panic!(); 0i32}).unwrap_err());
		// ret
	}
}

*/
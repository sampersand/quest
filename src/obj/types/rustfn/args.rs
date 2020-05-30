use std::slice::SliceIndex;
use crate::obj::{self, Result,  Object, types::{self, Convertible}};
use std::any::Any;
use std::borrow::Cow;
use std::ops::Deref;

use crate::obj::types::rustfn::Binding;

#[derive(Debug, Clone, Default)]
pub struct Args<'s> {
	binding: Binding,
	args: Cow<'s, [Object]>
}

impl Args<'_> {
	pub fn child_binding(&self) -> Result<Binding> {
		unimplemented!()
		// let obj = Object::new_with_parent((), Some(self.binding.clone()));
		// obj.set_attr(
		// 	"__args__".into(),
		// 	types::List::from(self.clone()).into(),
		// 	&self.binding
		// )?;
		// Ok(obj)
	}
}

impl<'s> Args<'s> {
	pub fn new<V: Into<Cow<'s, [Object]>>>(args: V) -> Self {
		Args { args: args.into(), binding: Binding::instance() }
	}

	pub fn new_slice(args: &'s [Object]) -> Self {
		Args { args: args.into(), binding: Binding::instance() }
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

	pub fn arg_downcast_ref<'s, T: Any>(&'s self, index: usize) -> Result<impl Deref<Target=T> + 's> {
		self.arg(index)?.try_downcast_ref::<T>()
	}

	pub fn arg_downcast<T: Any + Clone>(&self, index: usize) -> Result<T> {
		self.arg_downcast_ref::<T>(index).map(|x| (*x).clone())
	}

	pub fn arg_call_into<T: Convertible>(&self, index: usize) -> Result<T> {
		self.arg(index)?.downcast_call()
	}
	
	pub fn args<'c, I>(&'c self, idx: I) -> obj::Result<Args<'c>>
	where I: SliceIndex<[Object], Output=[Object]> + 'c
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

	pub fn this_downcast_ref<'s, T: Any>(&'s self) -> Result<impl Deref<Target=T> + 's> {
		self.this()?.try_downcast_ref::<T>()
	}

	pub fn this_downcast<T: Any + Clone>(&self) -> Result<T> {
		self.this_downcast_ref::<T>().map(|x| (*x).clone())
	}
}




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
		let ret = self.get_downcast(0);
		debug_assert!(ret.is_ok(), "invalid `this` encountered: {:?}, {:?}", self, ret.map(|x| {panic!(); 0i32}).unwrap_err());
		ret
	}
}











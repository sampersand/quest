use std::slice::SliceIndex;
use crate::{Result, Object, error::KeyError, types};
use std::borrow::Cow;
use std::iter::FromIterator;

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
		Self::new(args)
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

impl FromIterator<Object> for Args<'static> {
	fn from_iter<I: IntoIterator<Item=Object>>(iter: I) -> Self {
		Args::new(iter.into_iter().collect::<Vec<_>>())
	}
}

impl Args<'_> {
	pub fn arg<'s>(&'s self, idx: usize) -> Result<&'s Object> {
		self.args.get(idx + 1)
			.ok_or_else(|| KeyError::OutOfBounds { idx: idx + 1, len: self.args.len() }.into())
	}	

	pub fn args<'c, I>(&'c self, idx: I) -> Result<Args<'c>>
	where
		I: SliceIndex<[Object], Output=[Object]> + 'c + fmt::Debug + Clone
	{
		if let Some(rng) = self.args.get(1..).and_then(|args| args.get(idx.clone())) {
			Ok(self.new_args_slice(rng))
		} else {
			Err(KeyError::BadSlice { slice: format!("{:?}", idx), len: self.args.len() }.into())
		}
	}


	pub fn this<'s>(&'s self) -> Result<&'s Object> {
		self.args.get(0).ok_or_else(|| KeyError::NoThisSupplied.into())
	}
}

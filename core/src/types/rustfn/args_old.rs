use std::slice::SliceIndex;
use crate::{Result, Object, error::KeyError, types};
use std::borrow::Cow;
use std::iter::FromIterator;


#[deprecated]
#[derive(Clone, Default)]
pub struct ArgsOld<'s> {
	args: Cow<'s, [Object]>
}

use std::fmt::{self, Debug, Formatter};

impl Debug for ArgsOld<'_> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(&self.args, f)
	}
}


impl From<crate::Args<'_, '_>> for ArgsOld<'static> {
	fn from(args: crate::Args<'_, '_>) -> Self {
		args.into_iter().cloned().collect()
	}
}
impl<'s> ArgsOld<'s> {
	pub fn new<V: Into<Cow<'s, [Object]>>>(args: V) -> Self {
		ArgsOld { args: args.into() }
	}

	pub fn new_slice(args: &'s [Object]) -> Self {
		ArgsOld { args: args.into() }
	}

	pub fn new_args_slice<'a>(&self, args: &'a [Object]) -> ArgsOld<'a> {
		ArgsOld { args: args.into() }
	}

	pub fn add_this(&mut self, this: Object)  {
		self.args.to_mut().insert(0, this);
	}
}

impl From<ArgsOld<'_>> for Vec<Object> {
	fn from(args: ArgsOld) -> Self {
		args.args.to_vec()
	}
}

impl From<&'_ [&'_ Object]> for ArgsOld<'static> {
	fn from(args: &[&Object]) -> Self {
		ArgsOld::new(args.iter().map(|x| (*x).clone()).collect::<Vec<_>>())
	}
}

macro_rules! impl_from_slice {
	($($n:literal)*) => {
		$(
			impl<'a> From<&'a [Object; $n]> for ArgsOld<'a> {
				fn from(n: &'a [Object; $n]) -> Self {
					Self::new(n as &'a [Object])
				}
			}
		)+
	};
}

impl_from_slice!(0 1 2 3 4 5);

impl From<Vec<Object>> for ArgsOld<'static> {
	fn from(args: Vec<Object>) -> Self {
		Self::new(args)
	}
}

impl<'a> From<&'a [Object]> for ArgsOld<'a> {
	fn from(args: &'a [Object]) -> Self {
		Self::new(args)
	}
}

impl AsRef<[Object]> for ArgsOld<'_> {
	fn as_ref(&self) -> &[Object] {
		self.args.as_ref()
	}
}

impl From<ArgsOld<'_>> for types::List {
	fn from(args: ArgsOld<'_>) -> Self {
		types::List::from(args.args.to_vec())
	}
}

impl FromIterator<Object> for ArgsOld<'static> {
	fn from_iter<I: IntoIterator<Item=Object>>(iter: I) -> Self {
		ArgsOld::new(iter.into_iter().collect::<Vec<_>>())
	}
}

impl ArgsOld<'_> {
	pub fn arg<'s>(&'s self, idx: usize) -> Result<&'s Object> {
		self.args.get(idx + 1)
			.ok_or_else(|| KeyError::OutOfBounds { idx: (idx as isize) + 1, len: self.args.len() }.into())
	}	

	pub fn args<'c, I>(&'c self, idx: I) -> Result<ArgsOld<'c>>
	where
		I: SliceIndex<[Object], Output=[Object]> + 'c + fmt::Debug + Clone
	{
		if self.args.len() == 0 {
			return Ok(Default::default())
		}

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

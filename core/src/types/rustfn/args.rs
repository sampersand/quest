use std::slice::SliceIndex;
use crate::{Object, types};
use crate::error::KeyError;
use std::borrow::Cow;
use std::iter::FromIterator;


#[derive(Clone, Default)]
pub struct Args<'s, 'o: 's>(Cow<'s, [&'o Object]>);

use std::fmt::{self, Debug, Formatter};

impl Debug for Args<'_, '_> {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(&self.0, f)
	}
}

impl<'s, 'o: 's> Args<'s, 'o> {
	#[inline]
	pub fn new<V: Into<Cow<'s, [&'o Object]>>>(args: V) -> Self {
		Args(args.into())
	}

	#[inline]
	pub fn into_inner(self) -> Cow<'s, [&'o Object]> {
		self.0
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.0.len()
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	pub fn iter<'a: 's>(&'a self) -> impl Iterator<Item=&'o Object> + 'a {
		struct Iter<'s, 'o: 's>(std::slice::Iter<'s, &'o Object>);

		impl<'s, 'o: 's> Iterator for Iter<'s, 'o> {
			type Item = &'o Object;
			fn next(&mut self) -> Option<Self::Item> {
				self.0.next().copied()
			}
		}

		Iter(self.0.iter())
	}

	pub fn arg(&self, index: usize) -> Option<&'o Object> {
		self.0.get(index).copied()
	}

	pub fn try_arg(&self, index: usize) -> Result<&'o Object, KeyError> {
		self.arg(index).ok_or_else(|| KeyError::MissingArgument { index })
	}

	pub fn args<I>(&self, index: I) -> Option<Args<'_, 'o>>
	where
		I: SliceIndex<[&'o Object], Output=[&'o Object]>
	{
		self.0.get(index).map(Args::from)
	}

	pub fn try_args<I>(&self, index: I) -> Result<Args<'_, 'o>, KeyError>
	where
		I: SliceIndex<[&'o Object], Output=[&'o Object]> + fmt::Debug + Clone
	{
		if let Some(rng) = self.args(index.clone()) {
			Ok(rng)
		} else {
			Err(KeyError::BadSlice { range: format!("{:?}", index), len: self.0.len() })
		}
	}
}

impl From<Args<'_, '_>> for Vec<Object> {
	fn from(args: Args) -> Self {
		args.0.iter().map(|x| (*x).clone()).collect()
	}
}

impl<'s, 'o> From<&'s [&'o Object]> for Args<'s, 'o> {
	#[inline]
	fn from(args: &'s [&'o Object]) -> Self {
		Args::new(args)
	}
}

macro_rules! impl_from_slice {
	($($n:literal)*) => {
		$(
			impl<'a, 'o> From<&'a [&'o Object; $n]> for Args<'a, 'o> {
				#[inline]
				fn from(n: &'a [&'o Object; $n]) -> Self {
					Self::new(n as &'a [&'o Object])
				}
			}
		)+
	};
}

impl_from_slice!(0 1 2 3 4 5);

impl<'o> From<Vec<&'o Object>> for Args<'o, 'o> {
	#[inline]
	fn from(args: Vec<&'o Object>) -> Self {
		Self::new(args)
	}
}

impl<'o> AsRef<[&'o Object]> for Args<'_, 'o> {
	#[inline]
	fn as_ref(&self) -> &[&'o Object] {
		self.0.as_ref()
	}
}

impl<'o> AsMut<Vec<&'o Object>> for Args<'_, 'o> {
	#[inline]
	fn as_mut(&mut self) -> &mut Vec<&'o Object> {
		self.0.to_mut()
	}
}

impl From<Args<'_, '_>> for types::List {
	fn from(args: Args) -> Self {
		types::List::from(Vec::<Object>::from(args))
	}
}

impl<'o> FromIterator<&'o Object> for Args<'o, 'o> {
	fn from_iter<I: IntoIterator<Item=&'o Object>>(iter: I) -> Self {
		Args::new(iter.into_iter().collect::<Vec<_>>())
	}
}

impl<'s, 'o: 's> IntoIterator for Args<'s, 'o> {
	// in the future, maybe figure out a way to return the slice?
	type Item = <Vec<&'o Object> as IntoIterator>::Item;
	type IntoIter = <Vec<&'o Object> as IntoIterator>::IntoIter;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.0.into_owned().into_iter()
	}
}

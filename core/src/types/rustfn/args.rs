use crate::utils::SliceIndex;
use crate::{Object, types};
use crate::error::{KeyError, ArgumentError};
use std::borrow::Cow;
use std::iter::FromIterator;
use std::convert::TryFrom;

#[derive(Clone, Default)]
pub struct Args<'s, 'o>(Cow<'s, [&'o Object]>);

use std::fmt::{self, Debug, Formatter};

impl Debug for Args<'_, '_> {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(&self.0, f)
	}
}

impl<'s, 'o> Args<'s, 'o> {
	#[inline]
	pub fn new(args: impl Into<Self>) -> Self {
		args.into()
	}

	pub const fn const_new(args: &'s [&'o Object]) -> Self {
		Self(Cow::Borrowed(args))
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

	pub fn to_vec(&self) -> Vec<&'o Object> {
		self.0.to_owned().to_vec()
	}

	pub fn to_cloned_vec(&self) -> Vec<Object> {
		self.to_vec().into_iter().cloned().collect()
	}

	pub fn as_ref_checked(&self, expected: usize) -> Result<&[&'o Object], ArgumentError> {
		let given = self.len();

		if given == expected {
			Ok(self.as_ref())
		} else {
			Err(ArgumentError::InvalidLength { expected, given })
		}
	}

	pub fn as_mut_checked(&mut self, expected: usize) -> Result<&mut [&'o Object], ArgumentError> {
		let given = self.len();

		if given == expected {
			Ok(self.as_mut())
		} else {
			Err(ArgumentError::InvalidLength { expected, given })
		}
	}

	pub fn shorten<'new_o>(self) -> Args<'s, 'new_o>
	where
		'o: 'new_o
	{
		Args(self.0.into_owned().into())
	}

	pub fn prepend(&mut self, ele: &'o Object) {
		self.as_mut().insert(0, ele);
	}

	pub fn iter<'a>(&'a self) -> impl Iterator<Item=&'o Object> + 'a {
		struct Iter<'s, 'o>(std::slice::Iter<'s, &'o Object>);

		impl<'s, 'o> Iterator for Iter<'s, 'o> {
			type Item = &'o Object;
			fn next(&mut self) -> Option<Self::Item> {
				self.0.next().copied()
			}
		}

		Iter(self.0.iter())
	}

	pub fn get<I>(&self, index: I) -> Option<&I::Output>
	where
		I: SliceIndex<[&'o Object]>
	{
		index.get(self.as_ref())
	}

	pub fn get_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
	where
		I: SliceIndex<[&'o Object]>
	{
		index.get_mut(self.as_mut())
	}

	pub fn arg(&self, index: usize) -> Option<&'o Object> {
		self.get(index).copied()
	}

	pub fn try_arg(&self, index: usize) -> Result<&'o Object, KeyError> {
		self.arg(index)
			.ok_or_else(|| KeyError::OutOfBounds { idx: index as isize, len: self.0.len() })
	}

	pub fn args<I>(&self, index: I) -> Option<Args<'_, 'o>>
	where
		I: std::slice::SliceIndex<[&'o Object], Output=[&'o Object]>
	{
		self.0.get(index).map(Args::from)
	}

	pub fn try_args<I>(&self, index: I) -> Result<Args<'_, 'o>, KeyError>
	where
		I: std::slice::SliceIndex<[&'o Object], Output=[&'o Object]> + fmt::Debug + Clone
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
		Self(args.into())
	}
}

macro_rules! impl_from_slice {
	($($n:literal)*) => {
		$(
			impl<'a, 'o> From<&'a [&'o Object; $n]> for Args<'a, 'o> {
				#[inline]
				fn from(n: &'a [&'o Object; $n]) -> Self {
					Self::const_new(n as &'a [&'o Object])
				}
			}

			impl<'o> TryFrom<Args<'_, 'o>> for [&'o Object; $n] {
				type Error = ArgumentError;

				fn try_from(args: Args<'_, 'o>) -> Result<Self, Self::Error> {
					TryFrom::try_from(args.as_ref())
						.map_err(|_| ArgumentError::InvalidLength { expected: $n, given: args.len() })
				}
			}
		)+
	};
}

impl_from_slice!(0 1 2 3 4 5 6);

impl<'o> From<Vec<&'o Object>> for Args<'o, 'o> {
	#[inline]
	fn from(args: Vec<&'o Object>) -> Self {
		Self(args.into())
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

impl<'s, 'o> IntoIterator for Args<'s, 'o> {
	// in the future, maybe figure out a way to return the slice?
	type Item = <Vec<&'o Object> as IntoIterator>::Item;
	type IntoIter = <Vec<&'o Object> as IntoIterator>::IntoIter;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.0.into_owned().into_iter()
	}
}

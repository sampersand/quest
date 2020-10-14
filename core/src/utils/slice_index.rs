#![allow(unused)]

use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

/// Like the stdlib's, but it also allows for `isize` slicing.
pub trait SliceIndex<T: ?Sized> {
	type Output: ?Sized;

	fn get(self, slice: &T) -> Option<&Self::Output>;
	fn get_mut(self, slice: &mut T) -> Option<&mut Self::Output>;
}

impl<T> SliceIndex<[T]> for usize {
	type Output = T;

	fn get(self, slice: &[T]) -> Option<&Self::Output> {
		slice.get(self)
	}

	fn get_mut(self, slice: &mut [T]) -> Option<&mut Self::Output> {
		slice.get_mut(self)
	}
}

impl<T> SliceIndex<[T]> for isize {
	type Output = T;

	fn get(self, slice: &[T]) -> Option<&Self::Output> {
		super::correct_index(self, slice.len())
			.ok()
			.and_then(|u| u.get(slice))
	}

	fn get_mut(self, slice: &mut [T]) -> Option<&mut Self::Output> {
		super::correct_index(self, slice.len())
			.ok()
			.and_then(move |u| u.get_mut(slice))
	}
}


// 	pub fn arg(&self, index: usize) -> Option<&'o Object> {
// 		self.0.get(index).copied()
// 	}

// 	pub fn args<I>(&self, index: I) -> Option<Args<'_, 'o>>
// 	where
// 		I: SliceIndex<[&'o Object], Output=[&'o Object]>
// 	{
// 		self.0.get(index).map(Args::from)
// 	}

// 	pub fn try_args<I>(&self, index: I) -> Result<Args<'_, 'o>, KeyError>
// 	where
// 		I: SliceIndex<[&'o Object], Output=[&'o Object]> + fmt::Debug + Clone
// 	{
// 		if let Some(rng) = self.args(index.clone()) {
// 			Ok(rng)
// 		} else {
// 			Err(KeyError::BadSlice { range: format!("{:?}", index), len: self.0.len() })
// 		}
// 	}
// }

// impl From<Args<'_, '_>> for Vec<Object> {
// 	fn from(args: Args) -> Self {
// 		args.0.iter().map(|x| (*x).clone()).collect()
// 	}
// }

// impl<'s, 'o> From<&'s [&'o Object]> for Args<'s, 'o> {
// 	#[inline]
// 	fn from(args: &'s [&'o Object]) -> Self {
// 		Args::new(args)
// 	}
// }

// macro_rules! impl_from_slice {
// 	($($n:literal)*) => {
// 		$(
// 			impl<'a, 'o> From<&'a [&'o Object; $n]> for Args<'a, 'o> {
// 				#[inline]
// 				fn from(n: &'a [&'o Object; $n]) -> Self {
// 					Self::new(n as &'a [&'o Object])
// 				}
// 			}
// 		)+
// 	};
// }

// impl_from_slice!(0 1 2 3 4 5);

// impl<'o> From<Vec<&'o Object>> for Args<'o, 'o> {
// 	#[inline]
// 	fn from(args: Vec<&'o Object>) -> Self {
// 		Self::new(args)
// 	}
// }

// impl<'o> AsRef<[&'o Object]> for Args<'_, 'o> {
// 	#[inline]
// 	fn as_ref(&self) -> &[&'o Object] {
// 		self.0.as_ref()
// 	}
// }

// impl<'o> AsMut<Vec<&'o Object>> for Args<'_, 'o> {
// 	#[inline]
// 	fn as_mut(&mut self) -> &mut Vec<&'o Object> {
// 		self.0.to_mut()
// 	}
// }

// impl From<Args<'_, '_>> for types::List {
// 	fn from(args: Args) -> Self {
// 		types::List::from(Vec::<Object>::from(args))
// 	}
// }

// impl<'o> FromIterator<&'o Object> for Args<'o, 'o> {
// 	fn from_iter<I: IntoIterator<Item=&'o Object>>(iter: I) -> Self {
// 		Args::new(iter.into_iter().collect::<Vec<_>>())
// 	}
// }

// impl<'s, 'o: 's> IntoIterator for Args<'s, 'o> {
// 	// in the future, maybe figure out a way to return the slice?
// 	type Item = <Vec<&'o Object> as IntoIterator>::Item;
// 	type IntoIter = <Vec<&'o Object> as IntoIterator>::IntoIter;

// 	#[inline]
// 	fn into_iter(self) -> Self::IntoIter {
// 		self.0.into_owned().into_iter()
// 	}
// }

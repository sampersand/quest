use crate::{Object, Result, Args};
use std::fmt::{self, Debug, Formatter};

/// An iterator that can be used within [`Iter`].
trait ClonableIter : Iterator<Item=Result<Object>> + 'static + Send + Sync {
	fn clone(&self) -> Box<dyn ClonableIter>;
}

impl<I> ClonableIter for I
where
	I: Iterator<Item=Result<Object>> + Clone + 'static + Send + Sync
{
	fn clone(&self) -> Box<dyn ClonableIter> {
		Box::new(self.clone())
	}
}

/// A type that's entire purpose is to be iterated over.
pub struct Iter {
	iter: Box<dyn ClonableIter>,
	put_back: Option<Object>
}

impl Debug for Iter {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			f.debug_struct("Iter")
				.field("iter", &(&*self.iter as *const _))
				.field("put_back", &self.put_back)
				.finish()
		} else {
			f.debug_tuple("Iter")
				.field(&(&*self.iter as *const _))
				.finish()
		}
	}
}

impl Clone for Iter {
	fn clone(&self) -> Self {
		Self {
			iter: self.iter.clone(), // note: this calls `ClonableIter::clone`.
			put_back: self.put_back.clone()
		}
	}
}

impl Default for Iter {
	fn default() -> Self {
		Self::new(std::iter::empty())
	}
}

impl Iter {
	/// Creates a new [`Iter`] with the given function.
	pub fn new<I>(iter: I) -> Self
	where
		I: IntoIterator<Item=Result<Object>>,
		I::IntoIter: Clone + 'static + Send + Sync,
	{
		Self {
			iter: Box::new(iter.into_iter()),
			put_back: None
		}
	}

	/// Creates a new [`Iter`] from a function, which will iterate until the
	/// function returns `None`.
	pub fn from_fn<F>(func: F) -> Self
	where
		F: FnMut() -> Option<Result<Object>> + Send + Sync + Clone + 'static
	{
		Self::new(std::iter::from_fn(func))
	}

	pub fn with_objects<I>(iter: I) -> Self
	where
		I: IntoIterator<Item=Object>,
		I::IntoIter: Clone + 'static + Send + Sync,
	{
		Self::new(iter.into_iter().map(Ok))
	}

	/// Creates an iterator from an object. The object will be repeatedly called until it returns
	/// [`StopIteration`].
	pub fn from_callable(obj: Object) -> Self {
		Self::new(std::iter::from_fn(move ||
			obj.call_attr_lit("()", &[])
				.map(|result| if result.is_a::<StopIteration>() { None } else { Some(result) })
				.transpose()
			)
		)
	}
}

impl Iterator for Iter {
	type Item = Result<Object>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.put_back
			.take()
			.map(Ok)
			.or_else(|| self.iter.next())
	}
}

// Note that we're unable to use a lot of the builtin functions like `take_while`, as
// they don't allow us to return a `Result<bool>`.
impl Iter {
	/// Attempts to collect the elements of this iterator into a [`List`], returning any errors
	/// that occur during the process.
	pub fn try_into_list(&mut self) -> Result<crate::types::List> {
		self.collect::<Result<Vec<_>>>().map(From::from)
	}

	/// Calls the function for each element in this iterator.
	pub fn each<F>(mut self, mut func: F) -> Self
	where
		F: FnMut(&Object) -> Result<()> + Send + Sync + Clone + 'static
	{
		Self::from_fn(move || {
			Some(self.next()?.and_then(|ele| func(&ele).and(Ok(ele))))
		})
	}

	/// Runs this iterator through. This is only really helpful when an earlier streaming has
	/// a side effect.
	pub fn run(&mut self) -> Result<()> {
		for ele in self {
			ele?;
		}

		Ok(())
	}

	/// Enumerates all the items in the list.
	pub fn enumerate(self) -> Self {
		Self::new(Iterator::enumerate(self).map(|(idx, val)| Ok(vec![val?, idx.into()].into())))
	}

	/// Maps all elements through the function.
	pub fn map<F>(self, mut func: F) -> Self
	where
		F: FnMut(Object) -> Result<Object> + Send + Sync + Clone + 'static
	{
		Self::new(Iterator::map(self, move |val| func(val?)))
	}

	/// Only pass values onwards for which `func` returns true.
	pub fn select<F>(mut self, mut func: F) -> Self
	where
		F: FnMut(&Object) -> Result<bool> + Send + Sync + Clone + 'static
	{
		Self::from_fn(move || {
			while let Some(value) = self.next() {
				match value.and_then(|value| Ok((func(&value)?, value))) {
					Ok((true, value)) => return Some(Ok(value)),
					Ok((false, _)) => { /* continue onwards */ },
					Err(err) => return Some(Err(err)),
				}
			}

			None
		})
	}

	/// Continuously apply the `func` to the last result of it and the value of the iter.
	pub fn reduce<T, F>(self, mut acc: T, mut func: F) -> Result<T>
	where
		F: FnMut(T, Object) -> Result<T>
	{
		for ele in self {
			acc = func(acc, ele?)?;
		}

		Ok(acc)
	}

	/// Zips this iterator with other ones.
	pub fn zip(self, iters: impl IntoIterator<Item=Self>) -> Self {
		let iters = iters.into_iter();
		let mut zippers = Vec::with_capacity(iters.size_hint().0 + 1);

		zippers.push(self);
		zippers.extend(iters);

		Self::from_fn(move || {
			let mut args = Vec::with_capacity(zippers.len());

			for zip in &mut zippers {
				match zip.next()? {
					Ok(value) => args.push(value),
					Err(err) => return Some(Err(err))
				}
			}

			Some(Ok(args.into()))
		})
	}

	/// Takes the first `amount` elements, returning `None` after.
	pub fn take(self, amount: usize) -> Self {
		Self::new(Iterator::take(self, amount))
	}

	/// Takes elements while the function returns true.
	pub fn take_while<F>(mut self, mut func: F) -> Self
	where
		F: FnMut(&Object) -> Result<bool> + Clone + Send + Sync + 'static
	{
		let mut finished = false;

		Self::from_fn(move || {
			if finished {
				return None;
			}

			match self.next()?.and_then(|object| Ok((func(&object)?, object))) {
				Ok((true, object)) => Some(Ok(object)),
				Ok((false, _)) => { finished = true; None },
				Err(err) => Some(Err(err))
			}
		})
	}

	/// Ignores the first `amount` elements.
	pub fn drop(self, amount: usize) -> Self {
		Self::new(Iterator::skip(self, amount))
	}

	/// Ignores elements while the function returns true.
	pub fn drop_while<F>(mut self, mut func: F) -> Self
	where
		F: FnMut(&Object) -> Result<bool> + Clone + Send + Sync + 'static
	{
		let mut started = false;

		Self::from_fn(move || {
			if started {
				return self.next();
			}

			while let Some(result) = self.next() {
				match result.and_then(|object| Ok((func(&object)?, object))) {
					Ok((true, _)) => { /* go to the next one */},
					Ok((false, object)) => { started = true; return Some(Ok(object)) },
					Err(err) => return Some(Err(err))
				}
			}

			None
		})
	}

	/// Cycles either `n` times, or forever if `None` is passed.
	pub fn cycle(self, amount: Option<usize>) -> Self {
		match amount {
			None => Self::new(Iterator::cycle(self)),
			Some(0) => Self::default(),
			Some(1) => self,
			// OPTIMIZE: this can probably be optimized. But something something root of all evil, I'll do it later.
			Some(n) => Self::new(Iterator::chain(Clone::clone(&self), self.cycle(Some(n - 1)))),
		}
	}

	/// Takes every `n` elements and returns them. If `< n` elements are left, all of them are returned.
	///
	/// # Panics
	/// Panics if `size` is zero.
	pub fn chunk(mut self, size: usize) -> Self {
		assert_ne!(size, 0, "can't chunk by zero elements!");

		Self::from_fn(move || {
			match self.by_ref().take(size).collect::<Result<Vec<_>>>() {
				Ok(vec) if vec.is_empty() => None,
				Ok(vec) => Some(Ok(vec.into())),
				Err(err) => Some(Err(err))
			}
		})
	}

	/// Takes elements while the function returns `true`.
	pub fn chunk_while<F>(mut self, mut func: F) -> Self
	where
		F: FnMut(&Object) -> Result<bool> + Clone + Send + Sync + 'static
	{
		Self::from_fn(move || {
			let mut vec = Vec::new();
			let mut had_a_result = false;

			while let Some(result) = self.next() {
				had_a_result = true;

				match result.and_then(|obj| Ok((func(&obj)?, obj))) {
					Ok((true, object)) => vec.push(object),
					Ok((false, object)) => {
						debug_assert!(self.put_back.is_none(), "attempted to put back when we already have an ele?");
						self.put_back = Some(object);
						break;
					},
					Err(err) => return Some(Err(err))
				}
			}

			if had_a_result {
				Some(Ok(vec.into()))
			} else {
				None
			}
		})
	}

	/// Checks to see if `func` returns true for all elements.
	///
	/// If no elements exist, this returns `true`.
	pub fn all<F>(&mut self, mut func: F) -> Result<bool>
	where
		F: FnMut(Object) -> Result<bool> + Clone + Send + Sync + 'static
	{
		for ele in self {
			if !func(ele?)? {
				return Ok(false);
			}
		}

		Ok(true)
	}

	/// Checks to see if `func` returns true for any elements.
	///
	/// If no elements exist, this returns `false`.
	pub fn any<F>(&mut self, mut func: F) -> Result<bool>
	where
		F: FnMut(Object) -> Result<bool> + Clone + Send + Sync + 'static
	{
		for ele in self {
			if func(ele?)? {
				return Ok(true);
			}
		}

		Ok(false)
	}

	/// Checks to see if `func` returns true for exactly one element.
	///
	/// If no elements exist, this returns `true`.
	pub fn one<F>(&mut self, mut func: F) -> Result<bool>
	where
		F: FnMut(Object) -> Result<bool> + Clone + Send + Sync + 'static
	{
		let mut one = false;

		for ele in self {
			if func(ele?)? {
				if one {
					return Ok(false);
				}
				one = true;
			}
		}

		Ok(one)
	}

	/// Checks to see if `func` returns true for exactly one element.
	///
	/// If no elements exist, this returns `true`.
	pub fn none<F>(&mut self, mut func: F) -> Result<bool>
	where
		F: FnMut(Object) -> Result<bool> + Clone + Send + Sync + 'static
	{
		for ele in self {
			if func(ele?)? {
				return Ok(false);
			}
		}

		Ok(true)
	}
}


impl Iter {
	#[tracing::instrument(name="Iter::()", level="trace", skip(this), fields(self = ?this))]
	pub fn qs_call(this: &Object, _: Args) -> Result<Object> {
		let mut this = this.try_downcast_mut::<Self>()?;
		let next = this.next().transpose()?;

		Ok(next.unwrap_or_else(|| StopIteration.into()))
	}

	#[tracing::instrument(name="Iter::@iter", level="trace", skip(this), fields(self = ?this))]
	pub fn qs_at_iter(this: &Object, _: Args) -> Result<Object> {
		Ok(Clone::clone(&*this.try_downcast::<Self>()?).into())
	}

	#[tracing::instrument(name="Iter::run", level="trace", skip(this), fields(self = ?this))]
	pub fn qs_run(this: &Object, _: Args) -> Result<Object> {
		this.try_downcast_mut::<Self>()
			.and_then(|mut iter| iter.run())
			.map(|_| Object::default())
	}


}

impl_object_type! { for Iter [(parents super::Iterable) (convert "@iter")]:
	"()" => method Self::qs_call,
	"@iter" => method Self::qs_at_iter,
	"run" => method Self::qs_run,
}



/// A type that signifies iteration has stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StopIteration;

impl_object_type!(for StopIteration [(parents crate::types::Basic)]:);

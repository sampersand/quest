use crate::{Object, Args, Literal, error::KeyError};
use crate::utils::{correct_index, IndexError};
use crate::types::{Convertible, Text, Boolean, Number};
use std::convert::{TryFrom, TryInto};
use std::iter::FromIterator;
use std::fmt::{self, Debug, Formatter};
use tracing::instrument;

/// A List in Quest.
///
/// Lists are what you'd expect from other languages: They start at 0, you can index
/// from the end (eg `list.(-1)` is the same as `list.(list.$len() - 1))`), etc.
#[derive(Clone, Default)]
pub struct List(Vec<Object>);

impl Debug for List {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			f.debug_tuple("List").field(&self.as_ref()).finish()
		} else {
			Debug::fmt(&self.as_ref(), f)
		}
	}
}

impl IntoIterator for List {
	type Item = <Vec<Object> as IntoIterator>::Item;
	type IntoIter = <Vec<Object> as IntoIterator>::IntoIter;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

impl FromIterator<Object> for List {
    fn from_iter<T: IntoIterator<Item=Object>>(iter: T) -> Self {
    	Self::from(Vec::from_iter(iter))
    }
}

/// Rust-centric list methods
impl List {
	/// Create a new list.
	#[inline]
	pub fn new(list: impl IntoIterator<Item=Object>) -> Self {
		list.into_iter().collect()
	}

	/// Get the list's length
	#[inline]
	pub fn len(&self) -> usize {
		self.0.len()
	}

	/// Checks if the list is empty
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	/// Get an [`Iterator`](std::iter::Iterator) over elements in this list.
	#[inline]
	pub fn iter(&self) -> impl Iterator<Item=&Object> {
		self.0.iter()
	}

	/// Returns the internal vector.
	#[inline]
	pub fn into_inner(self) -> Vec<Object> {
		self.0
	}

	/// Remove all elements from the list
	#[inline]
	pub fn clear(&mut self) {
		self.0.clear();
	}

	/// Get either a single element or a range of elements.
	///
	/// Quest supports negative indexing, which allows you to index from the end of the list.
	pub fn get(&self, index: isize) -> Option<&Object> {
		correct_index(index, self.len())
			.map(|index| &self.0[index])
			.ok()
	}

	/// Get either a single element or a range of elements.
	pub fn get_rng(&self, start: isize, stop: isize) -> Option<&[Object]> {
		let start = correct_index(start, self.len()).ok()?;
		let stop = 
			match correct_index(stop, self.len()) {
				Ok(stop) => stop + 1,
				Err(IndexError::TooPositive) => self.len(), // saturate to our biggest value.
				Err(IndexError::TooNegative) => return None
			};

		if stop < start {
			None
		} else {
			Some(&self.0[start..stop])
		}
	}

	/// Sets a single element in a list
	#[must_use="it's possible for the index to be out of bounds."]
	pub fn set(&mut self, index: isize, ele: Object) -> Option<Object> {
		match correct_index(index, self.len()) {
			Ok(index) => {
				self.0[index] = ele;
				None
			},
			Err(IndexError::TooPositive) => {
				let index = index as usize;
				self.0.resize_with(index, Default::default);
				self.0.push(ele);
				None
			},
			Err(IndexError::TooNegative) => Some(ele)
		}
	}

	/// Sets a range of elements within the list.
	///
	/// This can be used to delete sections of the list (set them to an empty list), and also
	/// resize lists.
	pub fn set_rng(&mut self, start: isize, stop: isize, eles: Vec<Object>) -> Option<Vec<Object>> {
		let start = correct_index(start, self.len()).ok()?;
		let stop = 
			match correct_index(stop, self.len()) {
				Ok(stop) => stop + 1,
				Err(IndexError::TooPositive) => self.len(), // saturate to our biggest value.
				Err(IndexError::TooNegative) => return Some(eles)
			};

		if stop < start {
			return Some(eles);
		}

		if stop >= self.len() {
			self.0.resize_with(stop, Default::default);
		}

		self.0.splice(start..stop, eles);

		None
	}

	/// Combine a list's elements into a [`Text`], separated by `joiner`.
	///
	/// If `joiner` is omitted, nothing is inserted between elements.
	pub fn join(&self, joiner: Option<&str>) -> crate::Result<String> {
		self.iter()
			.map(|obj| obj.call_downcast::<Text>().map(|t| t.to_string()))
			.collect::<crate::Result<Vec<_>>>()
			.map(|ok| ok.join(joiner.unwrap_or_default()))
	}

	/// Check to see if two lists are equal, length-wise and element-wise.
	pub fn eql(&self, rhs: &Self) -> crate::Result<bool> {
		if self.len() != rhs.len() {
			return Ok(false);
		}

		for (lhs, rhs) in self.iter().zip(rhs.iter()) {
			if !lhs.eq_obj(rhs)? {
				return Ok(false)
			}
		}

		Ok(true)
	}

	/// Add a new element to the end of the list.
	#[inline]
	pub fn push(&mut self, what: Object) {
		self.0.push(what);
	}

	/// Remove an element from the end of the list.
	#[inline]
	pub fn pop(&mut self) -> Option<Object> {
		self.0.pop()
	}

	/// Add an element to the front of the list.
	#[inline]
	pub fn unshift(&mut self, what: Object) {
		self.0.insert(0, what);
	}

	/// Add an element to the end of the list.
	#[inline]
	pub fn shift(&mut self)  -> Option<Object> {
		if self.is_empty() {
			None
		} else {
			Some(self.0.remove(0))
		}
	}

	/// Find an element in the list
	pub fn index(&self, needle: &Object) -> crate::Result<Option<usize>> {
		for (idx, val) in self.iter().enumerate() {
			if val.eq_obj(needle)? {
				return Ok(Some(idx));
			}
		}

		Ok(None)
	}
}

impl From<List> for Vec<Object> {
	#[inline]
	fn from(list: List) -> Self {
		list.into_inner()
	}
}

impl From<Vec<Object>> for List {
	#[inline]
	fn from(list: Vec<Object>) -> Self {
		Self(list)
	}
}

impl From<Vec<Object>> for Object {
	#[inline]
	fn from(list: Vec<Object>) -> Self {
		List::from(list).into()
	}
}

impl AsRef<[Object]> for List {
	#[inline]
	fn as_ref(&self) -> &[Object] {
		self.0.as_ref()
	}
}

impl TryFrom<&List> for Text {
	type Error = crate::Error;

	fn try_from(list: &List) -> crate::Result<Self> {
		let mut t = Vec::with_capacity(list.len());
		for item in list.iter() {
			t.push(item.call_attr_lit(&Literal::INSPECT, &[])?.call_downcast::<Text>()?.to_string());
		}
		Ok(format!("[{}]", t.join(", ")).into())
	}
}

impl From<&List> for Boolean {
	#[inline]
	fn from(l: &List) -> Self {
		(!l.is_empty()).into()
	}
}

impl std::ops::Add<List> for &List {
	type Output = List;

	/// Create a new list with the other added to the end of the current one
	fn add(self, other: List) -> Self::Output {
		let mut dup = self.clone();
		dup += other;
		dup
	}
}

impl std::ops::AddAssign for List {
	/// Add the other list to this one in place.
	#[inline]
	fn add_assign(&mut self, mut other: Self)  {
		self.0.append(&mut other.0);
	}
}

impl std::ops::Mul<usize> for &List {
	type Output = List;

	fn mul(self, len: usize) -> List {
		let mut v = Vec::with_capacity(len * self.len());

		for _ in 0..len {
			v.extend(self.clone());
		}

		v.into()
	}
}

impl std::ops::MulAssign<usize> for List {
	fn mul_assign(&mut self, len: usize) {
		if len == 0 {
			self.clear();
			return;
		}

		self.0.reserve(self.len() * len);
		let slice = self.0.clone();

		// start from `1` so we skip what we already have, end at len-1 so we can
		// insert the full slice in when we're done.
		for _ in 1..len-1 {
			self.0.extend(slice.iter().cloned());
		}

		self.0.extend(slice);
	}
}

/// "Try" operators
impl List {
	#[inline]
	pub fn try_sub(&self, other: &Self) -> crate::Result<Self> {
		let mut dup = self.clone();
		dup.try_sub_assign(other).and(Ok(dup))
	}

	pub fn try_sub_assign(&mut self, other: &Self) -> crate::Result<()>  {
		let mut i = 0;

		while i < self.len() {
			if other.index(&self.0[i])?.is_some() {
				self.0.remove(i);
			} else {
				i += 1;
			}
		}

		Ok(())
	}

	#[inline]
	pub fn try_bitand(&self, other: &Self) -> crate::Result<Self> {
		let mut dup = self.clone();
		dup.try_bitand_assign(other).and(Ok(dup))
	}

	pub fn try_bitand_assign(&mut self, other: &Self) -> crate::Result<()>  {
		let mut i = 0;
		let mut other = other.clone();

		while i < self.len() {
			if let Some(j) = other.index(&self.0[i])? {
				other.0.remove(j);
				i += 1;
			} else {
				self.0.remove(i);
			}
		}

		Ok(())
	}

	#[inline]
	pub fn try_bitor(&self, other: &Self) -> crate::Result<Self> {
		let mut dup = self.clone();
		dup.try_bitor_assign(other).and(Ok(dup))
	}

	pub fn try_bitor_assign(&mut self, other: &Self) -> crate::Result<()>  {
		let mut i = 0;

		while i < other.len() {
			if self.index(&other.0[i])?.is_none() {
				self.0.push(other.0[i].clone());
			}

			i += 1;
		}

		Ok(())
	}

	#[inline]
	pub fn try_bitxor(&self, other: &Self) -> crate::Result<Self> {
		let mut dup = self.clone();
		dup.try_bitxor_assign(other).and(Ok(dup))
	}

	pub fn try_bitxor_assign(&mut self, other: &Self) -> crate::Result<()>  {
		let mut i = 0;
		let mut other = other.clone();

		while i < other.len() {
			if let Some(j) = self.index(&other.0[i])? {
				other.0.remove(i);
				self.0.remove(j);
			} else {
				i += 1;
			}
		}

		Ok(())
	}
}

/// Quest methods
impl List {
	/// Simply returns the list.
	///
	/// # Quest Examples
	///
	/// ```quest
	/// $list = [1, 2, 3];
	/// $list2 = list.$@list();
	///
	/// assert(list.$__id__ == list2.$__id__);
	///
	/// list.$push(4);
	///
	/// assert(clone == list);
	/// ```
	#[instrument(name="List::@list", level="trace", skip(this), fields(self=?this))]
	pub fn qs_at_list(this: &Object, _: Args) -> crate::Result<Object> {
		Ok(this.clone())
	}

	/// Attempts to convert this into a [`Text`].
	///
	/// This method calls the `@text` attribute of each element; if any of those attributes cause
	/// an error to occur, this function stops executing.
	///
	/// # Quest Examples
	/// ```quest
	/// $list = [1, "a", true];
	///
	/// assert(list.$@text() == '[1, "a", true]')
	/// ```
	#[instrument(name="List::@text", level="trace", skip(this), fields(self=?this))]
	pub fn qs_at_text(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(Text::try_from(&*this)?.into())
	}

	/// Attempts to get an internal representation of the list.
	///
	/// # Quest Examples
	/// ```quest
	/// $list = [1, "a", true];
	///
	/// assert(list.$inspect() == '[1, "a", true]')
	/// ```
	#[instrument(name="List::inspect", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_inspect(this: &Object, args: Args) -> crate::Result<Object> {
		Self::qs_at_text(this, args)
	}

	/// Converts this into a [`Boolean`].
	///
	/// A list is considered to be `false` when it is empty.
	/// 
	/// # Quest Examples
	/// ```quest
	/// assert([1, "a", true]);
	/// assert(![]);
	/// ```
	#[instrument(name="List::@bool", level="trace", skip(this), fields(self=?this))]
	pub fn qs_at_bool(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(Boolean::from(&*this).into())
	}

	#[instrument(name="List::each", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_each(this: &Object, args: Args) -> crate::Result<Object> {
		let block = args.try_arg(0)?;

		for idx in 0.. {
			// so as to not lock the object, we check the index each and every time.
			// this allows it to be modified during the `each` invocation.
			let obj = {
				let this = this.try_downcast::<Self>()?;
				if idx >= this.len() {
					break;
				}
				this.0[idx].clone()
			};

			block.call_attr_lit(&Literal::CALL, &[&obj])?;
		}

		Ok(this.clone())
	}

	/// Finds an object within the list, returning its index.
	///
	/// If the object isn't in the list, [`Null`](crate::types::Null) is returned.
	///
	/// # Arguments
	/// 
	/// 1. (required) The element to index.
	///
	/// # Quest Examples
	/// ```quest
	/// $list = [1, true, 3.5, "a"];
	///
	/// assert(list.$index(3.5) == 2);
	/// assert(list.$index("dog") == null);
	/// ```
	#[instrument(name="List::index", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_index(this: &Object, args: Args) -> crate::Result<Object> {
		let needle = args.try_arg(0)?;
		let this = this.try_downcast::<Self>()?;

		Ok(this.index(needle)?
			.map(Object::from)
			.unwrap_or_default())
	}

	/// Remove all elements from the list and returns the list.
	///
	/// # Quest Examples
	/// ```quest
	/// $list = [1, 2, 3];
	///
	/// assert(list);
	/// list.$clear();
	/// assert(!list);
	/// ```
	#[instrument(name="List::clear", level="trace", skip(this), fields(self=?this))]
	pub fn qs_clear(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_mut::<Self>()?.clear();

		Ok(this.clone())
	}

	/// Get the length of the list
	///
	/// # Quest Examples
	/// ```quest
	/// $list = ["foo", "bar", "baz"];
	///
	/// assert(list.$len() == 3);
	/// assert([].$len() == 0);
	#[instrument(name="List::len", level="trace", skip(this), fields(self=?this))]
	pub fn qs_len(this: &Object, _: Args) -> crate::Result<Object> {
		Ok(this.try_downcast::<Self>()?
			.len()
			.into())
	}

	/// Gets an element or range from the list
	///
	/// If the element is out of range, [`Null`](crate::types::Null) is returned.
	/// When using the range form, out-of-bounds `start` and `stop` values are converted to the
	/// min/max of the list (respectively).
	///
	/// Quest supports negative indexing, which allows you to index from the end of the list.
	/// 
	/// # Arguments
	///
	/// 1. (required, `@num`) The index / start of the range.
	/// 2. (optional, `@num`) The end of the range.
	///
	/// # Quest Examples
	///
	/// ```quest
	/// $list = ['a', 2, 3, false];
	/// 
	/// assert(list.$get(0) == 'a');
	/// assert(list.$get(5) == null);
	/// assert(list.$get(-1) == false);
	/// assert(list.$get(1, 2) == [2, 3]);
	/// assert(list.$get(1, Number::$INF) == [2, 3, false]);
	/// ```
	#[instrument(name="List::get", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_get(this: &Object, args: Args) -> crate::Result<Object> {
		let start: isize = args.try_arg(0)?.call_downcast::<Number>().map(|n| *n)?.try_into()?;

		let stop = 
			args.arg(1)
			.map(|n| n.call_downcast::<Number>().map(|n| *n))
			.transpose()?
			.map(isize::try_from)
			.transpose()?;

		let this = this.try_downcast::<Self>()?;

		if let Some(stop) = stop {
			Ok(this.get_rng(start, stop)
				.map(|x| Self::new(x.to_owned()).into())
				.unwrap_or_default())
		} else {
			Ok(this.get(start).cloned().unwrap_or_default())
		}
	}

	/// Sets an element or range of the list to an element or list.
	///
	/// This allows you to delete chunks of the list if you want to by setting them to empty lists.
	///
	/// # Arguments
	///
	/// 1. (required, `@num`) The index / start of the range
	/// 2. (required, `@num` if end of range) The element to set **or** the end of the range.
	/// 3. (optional) The element to set when using a range
	/// 
	/// # Quest Examples
	/// ```quest
	/// <TODO>
	/// ```
	#[instrument(name="List::set", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_set(this: &Object, args: Args) -> crate::Result<Object> {
		let pos: isize = args.try_arg(0)?.call_downcast::<Number>().map(|n| *n)?.try_into()?;

		if args.len() == 2 {
			let ele = args.arg(1).unwrap().clone();
			let mut this = this.try_downcast_mut::<Self>()?;

			if this.set(pos, ele.clone()).is_some() {
				Err(KeyError::OutOfBounds { idx: pos, len: this.len() }.into())
			} else {
				Ok(ele)
			}
		} else {
			let end: isize = args.try_arg(1)?.call_downcast::<Number>().map(|n| *n)?.try_into()?;
			let ele = args.try_arg(2)?.call_downcast::<Self>()?.clone();
			let mut this = this.try_downcast_mut::<Self>()?;

			if this.set_rng(pos, end, ele.0).is_some() {
				Err(KeyError::OutOfBounds { idx: pos, len: this.len() }.into())
			} else {
				// Ok(this.0[this.0.len() - 1].clone())
				// TODO: return type
				Ok(Default::default())
			}
		}
	}

	/// Combine all elements into a [`Text`], optionally separated by a deliminator.
	///
	/// # Arguments
	///
	/// 1. (optional, `@text`) The deliminator to be placed between elements; if omitted, nothing is
	/// inserted.
	///
	/// # Quest Examples
	/// ```quest
	/// assert(["foo", 123, true].$join() == "foo123true");
	/// assert(["foo", 123, true].$join(" ") == "foo 123 true");
	/// ```
	#[instrument(name="List::join", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_join(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;
		let delim = args.arg(0).map(|arg| arg.call_downcast::<Text>()).transpose()?;

		this.join(delim.as_ref().map(|delim| delim.as_ref())).map(Object::from)
	}

	#[instrument(name="List::*", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_mul(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;
		let amnt = usize::try_from(*args.try_arg(0)?.call_downcast::<Number>()?)?;

		Ok((&*this * amnt).into())
	}

	#[instrument(name="List::*=", level="trace", skip(this, args), fields(self=?this, args=?args))]
	pub fn qs_mul_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let amnt = usize::try_from(*args.try_arg(0)?.call_downcast::<Number>()?)?;

		*this.try_downcast_mut::<Self>()? *= amnt;

		Ok(this.clone())
	}

	/// Compares two [`List`]s
	///
	/// Two lists are considered equal if they have the same length, and each element in this list
	/// is equal to its counterpart in the other one.
	///
	/// # Quest Examples
	/// ```quest
	/// assert([1, 2] != [1, 2, 3]);
	/// assert([1, 2, "a"] == [1, 2, "a"]);
	/// assert([1, 2, "a"] != [1, "a", 2]);
	/// ```
	#[instrument(name="List::==", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_eql(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?;
		let mut eql = Ok(false);

		if this.is_identical(rhs) {
			eql = Ok(true)
		} else {
			let this = this.try_downcast::<Self>()?;
			if rhs.try_downcast::<Self>().map(|rhs| eql = this.eql(&rhs)).is_err() {
				// allow for downcasting errors whilst also having `eql` able to raise errors.
				eql = Ok(false);
			}
		}

		eql.map(Object::from)
	}

	/// Add an element to the back of the list, returning the list.
	///
	/// # Arguments
	///
	/// 1. (required) The element to add to the back of the list
	///
	/// # Quest Examples
	/// ```quest
	/// $list = [1, 2];
	///
	/// list.$push(3).$push(4);
	///
	/// assert(list == [1, 2, 3, 4])
	/// ```
	#[instrument(name="List::push", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_push(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?.clone();
		this.try_downcast_mut::<Self>()?.push(rhs);

		Ok(this.clone())
	}

	/// Remove an element from the end of the list, returning [`Null`](crate::types::Null) if empty.
	///
	/// # Quest Examples
	/// ```quest
	/// $list = ["a", "b"];
	///
	/// assert(list.$pop() == "b");
	/// assert(list.$pop() == "a");
	/// assert(list.$pop() == null);
	/// assert(!list);
	/// ```
	#[instrument(name="List::pop", level="trace", skip(this), fields(self=?this))]
	pub fn qs_pop(this: &Object, _: Args) -> crate::Result<Object> {
		Ok(this.try_downcast_mut::<Self>()?
			.pop()
			.unwrap_or_default())
	}

	/// Add an element at the front of the list, returning the list.
	///
	/// # Arguments
	///
	/// 1. (required) The element to add to the front of the list
	///
	/// # Quest Examples
	/// ```quest
	/// $list = [3, 4];
	///
	/// list.$unshift(2).$unshift(1);
	///
	/// assert(list == [1, 2, 3, 4])
	/// ```
	#[instrument(name="List::unshift", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_unshift(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?.clone();
		
		this.try_downcast_mut::<Self>()?.unshift(rhs);
		Ok(this.clone())
	}

	/// Remove an element from the front of the list, returning [`Null`](crate::types::Null) if empty.
	///
	/// # Quest Examples
	/// ```quest
	/// $list = ["a", "b"];
	///
	/// assert(list.$shift() == "a");
	/// assert(list.$shift() == "b");
	/// assert(list.$shift() == null);
	/// assert(!list);
	/// ```
	#[instrument(name="List::shift", level="trace", skip(this), fields(self=?this))]
	pub fn qs_shift(this: &Object, _: Args) -> crate::Result<Object> {
		Ok(this.try_downcast_mut::<Self>()?
			.shift()
			.unwrap_or_default())
	}

	/// Adds two lists together.
	///
	/// # Arguments
	///
	/// 1. (required, `@list`) The list to add.
	///
	/// # Quest Examples
	/// ```quest
	/// assert([1, 2] + [3, 4] == [1, 2, 3, 4]);
	/// assert(["a", "b"] + [] == ["a", "b"]);
	/// ```
	#[instrument(name="List::+", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_add(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?.call_downcast::<Self>()?;
		let this = this.try_downcast::<Self>()?;

		Ok((&*this + rhs.clone()).into())
	}

	/// Adds a list to the end of this one, in place, returning the first list.
	///
	/// # Quest Examples
	/// ```quest
	/// $list = [1, 2];
	///
	/// assert(list == [1, 2]);
	/// list += [3, 4];
	/// assert(list == [1, 2, 3, 4]);
	/// ```
	#[instrument(name="List::+=", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_add_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?;
		let mut this_mut = this.try_downcast_mut::<Self>()?;

		if this.is_identical(rhs) {
			let dup = this_mut.clone();
			*this_mut += dup;
		} else {
			*this_mut += rhs.call_downcast::<Self>()?.clone();
		};

		Ok(this.clone())
	}

	/// Returns a new list of elements in the first list but not the second.
	///
	/// # Arguments
	///
	/// 1. (required, `@list`) The other list.
	///
	/// # Quest Examples
	/// ```quest
	/// assert([1, 2, 3, "A", 2] - [1, 2] == [3, "A"]);
	/// assert(["1", "b", "c"] - [1, "c", "c", "e"] == ["1", "b"]);
	/// ```
	#[instrument(name="List::-", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_sub(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;
		let rhs = args.try_arg(0)?.call_downcast::<Self>()?;

		this.try_sub(&rhs).map(Object::from)
	}

	/// Delete all elements in the first list that are also in the second.
	///
	/// # Quest Examples
	/// ```quest
	/// $list = [1, 2, 3, "A", 2] 
	/// list -= [1, 2];
	/// assert(list == [3, "A"]);
	///
	/// $list = ["1", "b", "c"];
	/// list -= [1, "c", "c", "e"];
	/// assert(list == ["1", "b"]);
	/// ```
	#[instrument(name="List::-=", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_sub_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?;

		if this.is_identical(rhs) {
			return Self::qs_clear(rhs, Args::default());
		}

		this.try_downcast_mut::<Self>()?.try_sub_assign(&*rhs.call_downcast::<Self>()?)?;

		Ok(this.clone())
	}

	/// Get the intersection of two lists, i.e. the common elements
	///
	/// # Arguments
	///
	/// 1. (required, `@list`) The other list.
	///
	/// # Quest Examples
	/// ```quest
	/// assert([1, 2, 3] & [1, "a", "b"] == [1]);
	/// assert([1, 2, 3] & [4, 5, 6] == []);
	/// assert([1, 2, 3] & [1, 2, 4] == [1, 2]);
	/// ```
	#[instrument(name="List::&", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_bitand(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;
		let rhs = args.try_arg(0)?.call_downcast::<Self>()?;

		this.try_bitand(&rhs).map(Object::from)
	}

	/// Deletes all elements in the current list not common to both lists.
	///
	/// # Arguments
	///
	/// 1. (required, `@list`) The other list.
	///
	/// # Quest Examples
	/// ```quest
	/// $list = [1, 2, "a", 4];
	///
	/// list &= [1, 2, 3];
	/// assert(list == [1, 2]);
	///
	/// list &= [2, "a"];
	/// assert(list == [2]);
	/// ```
	#[instrument(name="List::&=", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_bitand_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?;

		if this.is_identical(rhs) {
			return Ok(this.clone());
		}

		this.try_downcast_mut::<Self>()?.try_bitand_assign(&*rhs.call_downcast::<Self>()?)?;

		Ok(this.clone())
	}

	/// Get the union of two lists, i.e. the combination of all elements
	///
	/// # Arguments
	///
	/// 1. (required, `@list`) The other list.
	///
	/// # Quest Examples
	/// ```quest
	/// assert([1, 2, 3] | [1, 1, "a", "b"] == [1, 2, 3, "a", "b"]);
	/// assert(["a", "b", "c"] | ["c", "b", "d", "e"] == ["a", "b", "c", "d", "e"]);
	/// ```
	#[instrument(name="List::|", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_bitor(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;
		let rhs = args.try_arg(0)?.call_downcast::<Self>()?;

		this.try_bitor(&rhs).map(Object::from)
	}

	/// Adds all unique elements in the second list to the original one.
	///
	/// # Arguments
	///
	/// 1. (required, `@list`) The other list.
	///
	/// # Quest Examples
	/// ```quest
	/// $list = [1, 2, "a", 4];
	///
	/// list |= [1, 2, 3];
	/// assert(list == [1, 2, "a", 4, 3]);
	///
	/// list |= ["a", 2]; # already present, don't do anything
	/// assert(list == [1, 2, "a", 4, 3]);
	/// ```
	#[instrument(name="List::|=", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_bitor_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?;

		if this.is_identical(rhs) {
			return Ok(this.clone());
		}

		this.try_downcast_mut::<Self>()?.try_bitor_assign(&*rhs.call_downcast::<Self>()?)?;

		Ok(this.clone())
	}

	/// Get the list of elements in only one list.
	///
	/// # Arguments
	///
	/// 1. (required, `@list`) The other list.
	///
	/// # Quest Examples
	/// ```quest
	/// assert([1, 2, 3] ^ [1, "a", "b"] == [2, 3, "a", "b"]);
	/// assert([1, 2, 3] ^ [4, 5, 6] == [1, 2, 3, 4, 5, 6]);
	/// assert([1, 2, 3] ^ [1, 2, 4] == [3, 4]);
	/// ```
	#[instrument(name="List::^", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_bitxor(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;
		let rhs = args.try_arg(0)?.call_downcast::<Self>()?;

		this.try_bitxor(&rhs).map(Object::from)
	}

	/// Deletes all elements in the current list not common to both lists.
	///
	/// # Arguments
	///
	/// 1. (required, `@list`) The other list.
	///
	/// # Quest Examples
	/// ```quest
	/// $list = [1, 2, "a", 4];
	///
	/// list ^= [1, 2, 3];
	/// assert(list == ["a", 4, 3]);
	///
	/// list ^= [2, "a"];
	/// assert(list == [4, 3, 2]);
	/// ```
	#[instrument(name="List::^=", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_bitxor_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?;

		if this.is_identical(rhs) {
			return Self::qs_clear(this, Args::default());
		}

		this.try_downcast_mut::<Self>()?.try_bitxor_assign(&*rhs.call_downcast::<Self>()?)?;

		Ok(this.clone())
	}
}

impl Convertible for List {
	const CONVERT_FUNC: Literal = Literal::AT_LIST;
}
impl_object_type!{
for List [(init_parent super::Basic super::Iterable) (parents super::Basic)]:
	"inspect" => function Self::qs_inspect,
	"@text" => function Self::qs_at_text,
	"@bool" => function Self::qs_at_bool,
	"@list" => function Self::qs_at_list,

	"each" => function Self::qs_each,

	"clear" => function Self::qs_clear,
	"index" => function Self::qs_index,
	"len"   => function Self::qs_len,

	"get"  => function Self::qs_get,
	"set"  => function Self::qs_set,
	"join" => function Self::qs_join,
	"*"    => function Self::qs_mul,
	"*="   => function Self::qs_mul_assign,

	"<<"      => function Self::qs_push,
	"push"    => function Self::qs_push,
	"pop"     => function Self::qs_pop,
	"unshift" => function Self::qs_unshift,
	"shift"   => function Self::qs_shift,

	"==" => function Self::qs_eql,
	"+"  => function Self::qs_add,
	"+=" => function Self::qs_add_assign,
	"-"  => function Self::qs_sub,
	"-=" => function Self::qs_sub_assign,
	"&"  => function Self::qs_bitand,
	"&=" => function Self::qs_bitand_assign,
	"|"  => function Self::qs_bitor,
	"|=" => function Self::qs_bitor_assign,
	"^"  => function Self::qs_bitxor,
	"^=" => function Self::qs_bitxor_assign,
}

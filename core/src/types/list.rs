use crate::{Object, Args};
use crate::literal::{Literal, INSPECT, AT_LIST};
use crate::types::{Convertible, Text, Boolean, Number};
use std::convert::TryFrom;
use std::iter::FromIterator;
use std::fmt::{self, Debug, Formatter};

/// A List in Quest.
///
/// Lists are what you'd expect from other languages: They start at 0, you can index
/// from the end (eg `list.(-1)` is the same as `list.(list.$len() - 1))`), etc.
#[derive(Clone)]
pub struct List(Vec<Object>);

impl Default for List {
	#[inline]
	fn default() -> Self {
		Self(Vec::default())
	}
}

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
	pub fn new<L: Into<Vec<Object>>>(list: L) -> Self {
		Self(list.into())
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
	pub fn iter(&self) -> std::slice::Iter<'_, Object> {
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
	pub fn get(&self, idx: isize) -> Object {
		correct_index(idx, self.len())
			.map(|idx| self.0[idx].clone())
			.unwrap_or_default()
	}

	/// Get either a single element or a range of elements.
	pub fn get_rng(&self, start: isize, stop: isize) -> Object {
		let start =
			if let Some(start) = correct_index(start, self.len()) {
				start
			} else {
				return Object::default()
			};

		let stop = correct_index(stop, self.len()).map(|x| x + 1).unwrap_or_else(|| self.len());
		if stop < start {
			Object::default()
		} else {
			self.0[start..stop].to_owned().into()
		}
	}

	/// Sets a single element in a list
	pub fn set(&self, _idx: isize, _ele: Object)  {
		unimplemented!()
	}

	/// Sets a range of elements within the list.
	///
	/// This can be used to delete sections of the list (set them to an empty list), and also
	/// resize lists.
	pub fn set_rng(&self, _start: isize, _stop: isize, _list: Self) {
		unimplemented!()
	}

	/// Combine a list's elements into a [`Text`], separated by `joiner`.
	///
	/// If `joiner` is omitted, nothing is inserted between elements.
	pub fn join(&self, joiner: Option<&str>) -> crate::Result<String> {
		self.iter()
			.map(|obj| obj.call_downcast_map(Text::to_string))
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
	pub fn find(&self, needle: &Object) -> crate::Result<Option<usize>> {
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
	fn from(list: Vec<Object>) -> Self {
		Self::new(list)
	}
}

impl From<Vec<Object>> for Object {
	fn from(list: Vec<Object>) -> Self {
		List::from(list).into()
	}
}

impl AsRef<[Object]> for List {
	fn as_ref(&self) -> &[Object] {
		self.0.as_ref()
	}
}

impl TryFrom<&'_ List> for Text {
	type Error = crate::Error;

	fn try_from(l: &List) -> crate::Result<Self> {
		let mut t = vec![];
		for item in l.iter() {
			t.push(item.call_attr_lit(INSPECT, &[])?.call_downcast_map(Text::to_string)?)
		}
		Ok(format!("[{}]", t.join(", ")).into())
	}
}

impl From<&'_ List> for Boolean {
	#[inline]
	fn from(l: &List) -> Self {
		(!l.is_empty()).into()
	}
}


impl std::ops::Add<List> for &'_ List {
	type Output = List;

	/// Create a new list with the other added to the end of the current one
	#[inline]
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
			if other.find(&self.0[i])?.is_some() {
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
			if let Some(j) = other.find(&self.0[i])? {
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
			if self.find(&other.0[i])?.is_none() {
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
			if let Some(j) = self.find(&other.0[i])? {
				other.0.remove(i);
				self.0.remove(j);
			} else {
				i += 1;
			}
		}

		Ok(())
	}
}

fn correct_index(idx: isize, len: usize) -> Option<usize> {
	if !idx.is_negative() {
		if (idx as usize) < len {
			Some(idx as usize)
		} else {
			None
		}
	} else {
		let idx = (-idx) as usize;
		if idx <= len {
			Some(len - idx)
		} else {
			None
		}
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
	pub fn qs_at_text(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_and_then(|this: &Self| Text::try_from(this))
			.map(Object::from)
	}

	/// Attempts to get an internal representation of the list.
	///
	/// # Quest Examples
	/// ```quest
	/// $list = [1, "a", true];
	///
	/// assert(list.$inspect() == '[1, "a", true]')
	/// ```
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
	pub fn qs_at_bool(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_map(|this: &Self| Boolean::from(this))
			.map(Object::from)
	}

	pub fn qs_each(this: &Object, args: Args) -> crate::Result<Object> {
		let block = args.arg(0)?;

		let mut done = false;
		for idx in 0.. {
			if done {
				break;
			}

			this.try_downcast_and_then(|this: &Self| {
				if idx >= this.len() {
					done = true;
					Ok(())
				} else {
					block.call_attr_lit("()", &[&this.0[idx]]).and(Ok(()))
				}
			})?;
		}

		Ok(this.clone())
	}

	/// Finds an object within the list, returning its index.
	///
	/// If the object isn't in the list, [`Null`](crate::types::Null) is returned.
	///
	/// # Arguments
	/// 
	/// 1. (required) The element to find.
	///
	/// # Quest Examples
	/// ```quest
	/// $list = [1, true, 3.5, "a"];
	///
	/// assert(list.$find(3.5) == 2);
	/// assert(list.$find("dog") == null);
	/// ```
	pub fn qs_find(this: &Object, args: Args) -> crate::Result<Object> {
		let needle = args.arg(0)?;

		this.try_downcast_and_then(|this: &Self| this.find(needle))
			.map(|x| x.map(Object::from).unwrap_or_default())
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
	pub fn qs_clear(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_mut_map(Self::clear).map(|_| this.clone())
	}

	/// Get the length of the list
	///
	/// # Quest Examples
	/// ```quest
	/// $list = ["foo", "bar", "baz"];
	///
	/// assert(list.$len() == 3);
	/// assert([].$len() == 0);
	pub fn qs_len(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_map(Self::len).map(Object::from)
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
	pub fn qs_get(this: &Object, args: Args) -> crate::Result<Object> {
		let start = args.arg(0)?
			.call_downcast_and_then(|n: &Number| isize::try_from(*n))?;

		let stop = args.arg(1)
			.ok()
			.map(|n| n.call_downcast_and_then(|n: &Number| isize::try_from(*n)))
			.transpose()?;

		this.try_downcast_map(|this: &Self| {
			stop.map(|stop| this.get_rng(start, stop))
				.unwrap_or_else(|| this.get(start))
		})
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
	pub fn qs_set(_: &Object, _: Args) -> crate::Result<Object> {
		todo!("set")
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
	pub fn qs_join(this: &Object, args: Args) -> crate::Result<Object> {
		this.try_downcast_and_then(|this: &Self| {
			if let Ok(arg) = args.arg(0) {
				arg.call_downcast_and_then(|delim: &Text| this.join(Some(delim.as_ref())))
			} else {
				this.join(None)
			}
		}).map(Object::from)
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
	pub fn qs_eql(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?;

		let mut eql = Ok(false);

		if this.is_identical(rhs) {
			eql = Ok(true)
		} else {
			this.try_downcast_map(|this: &Self| {
				if rhs.try_downcast_map(|rhs: &Self| eql = this.eql(rhs)).is_err() {
					// allow for downcasting errors whilst also having `eql` able to raise errors.
					eql = Ok(false);
				}
			})?;
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
	pub fn qs_push(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.clone();
		this.try_downcast_mut_map(|this: &mut Self| this.push(rhs))
			.map(|_| this.clone())
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
	pub fn qs_pop(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_mut_map(Self::pop).map(Option::unwrap_or_default)
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
	pub fn qs_unshift(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.clone();
		this.try_downcast_mut_map(|this: &mut Self| this.unshift(rhs))
			.map(|_| this.clone())
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
	pub fn qs_shift(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_mut_map(Self::shift).map(Option::unwrap_or_default)
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
	pub fn qs_add(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_map(|this| this + rhs).map(Object::from)
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
	pub fn qs_add_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.call_downcast_map(Self::clone)?;

		this.try_downcast_mut_map(|this: &mut Self| *this += rhs)
			.map(|_| this.clone())
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
	pub fn qs_sub(this: &Object, args: Args) -> crate::Result<Object> {
		this.call_downcast_and_then(|this: &Self| {
			args.arg(0)?
				.call_downcast_and_then(|rhs: &Self| this.try_sub(rhs))
				.map(Object::from)
		})
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
	pub fn qs_sub_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?;

		if this.is_identical(rhs) {
			return Self::qs_clear(rhs, Args::default());
		}

		this.try_downcast_mut_and_then(|this: &mut Self| {
			rhs.call_downcast_and_then(|rhs: &Self| this.try_sub_assign(rhs))
		}).map(|_| this.clone())
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
	pub fn qs_bitand(this: &Object, args: Args) -> crate::Result<Object> {
		this.call_downcast_and_then(|this: &Self| {
			args.arg(0)?
				.call_downcast_and_then(|rhs: &Self| this.try_bitand(rhs))
				.map(Object::from)
		})
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
	pub fn qs_bitand_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?;

		if this.is_identical(rhs) {
			return Ok(this.clone());
		}

		this.try_downcast_mut_and_then(|this: &mut Self| {
			rhs.call_downcast_and_then(|rhs: &Self| this.try_bitand_assign(rhs))
		}).map(|_| this.clone())
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
	pub fn qs_bitor(this: &Object, args: Args) -> crate::Result<Object> {
		this.call_downcast_and_then(|this: &Self| {
			args.arg(0)?
				.call_downcast_and_then(|rhs: &Self| this.try_bitor(rhs))
				.map(Object::from)
		})
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
	pub fn qs_bitor_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?;

		if this.is_identical(rhs) {
			return Ok(this.clone());
		}

		this.try_downcast_mut_and_then(|this: &mut Self| {
			rhs.call_downcast_and_then(|rhs: &Self| this.try_bitand_assign(rhs))
		}).map(|_| this.clone())
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
	pub fn qs_bitxor(this: &Object, args: Args) -> crate::Result<Object> {
		this.call_downcast_and_then(|this: &Self| {
			args.arg(0)?
				.call_downcast_and_then(|rhs: &Self| this.try_bitxor(rhs))
				.map(Object::from)
		})
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
	pub fn qs_bitxor_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?;

		if this.is_identical(rhs) {
			return Self::qs_clear(this, Args::default());
		}

		this.try_downcast_mut_and_then(|this: &mut Self| {
			rhs.call_downcast_and_then(|rhs: &Self| this.try_bitxor_assign(rhs))
		}).map(|_| this.clone())
	}
}

impl Convertible for List {
	const CONVERT_FUNC: Literal = AT_LIST;
}
impl_object_type!{
for List [(init_parent super::Basic super::Iterable) (parents super::Basic)]:
	"inspect" => function Self::qs_inspect,
	"@text" => function Self::qs_at_text,
	"@bool" => function Self::qs_at_bool,
	"@list" => function Self::qs_at_list,

	"each" => function Self::qs_each,

	"clear" => function Self::qs_clear,
	"find"  => function Self::qs_find,
	"len"   => function Self::qs_len,

	"get"  => function Self::qs_get,
	"set"  => function Self::qs_set,
	"join" => function Self::qs_join,

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

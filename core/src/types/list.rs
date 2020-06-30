use crate::{Object, Args};
use crate::literals::__INSPECT__;
use crate::types::{Text, Boolean, Number};
use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt::{self, Debug, Formatter};

/// A List in Quest.
///
/// Lists are what you'd expect from other languages: They start at 0, you can index
/// from the end (eg `list.(-1)` is the same as `list.(list.$len() - 1))`), etc.
#[derive(Clone)]
pub struct List(Cow<'static, [Object]>);

impl Debug for List {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			write!(f, "List({:?})", self.as_ref())
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
		self.0.into_owned().into_iter()
	}

}

impl List {
	/// Create a new list.
	#[inline]
	pub fn new<L: Into<Cow<'static, [Object]>>>(list: L) -> Self {
		List(list.into())
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


	/// Remove all elements from the list
	#[inline]
	pub fn clear(&mut self) {
		self.0.to_mut().clear();
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

		let stop = correct_index(stop, self.len()).map(|x| x + 1).unwrap_or(self.len());
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
	pub fn set_rng(&self, _start: isize, _stop: isize, _list: List) {
		unimplemented!()
	}

	/// Combine a list's elements into a [`Text`], separated by `joiner`.
	///
	/// If `joiner` is omitted, nothing is inserted between elements.
	pub fn join(&self, joiner: Option<&str>) -> crate::Result<Text> {
		Ok(self.iter()
			.map(|obj| obj.downcast_call::<Text>()
				.map(|txt| txt.as_ref().to_string()))
			.collect::<crate::Result<Vec<_>>>()?
			.join(joiner.unwrap_or_default()).into())
	}

	/// Check to see if two lists are equal, length-wise and element-wise.
	pub fn eql(&self, rhs: &List) -> crate::Result<bool> {
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
		self.0.to_mut().push(what);
	}

	/// Remove an element from the end of the list.
	#[inline]
	pub fn pop(&mut self) -> Option<Object> {
		self.0.to_mut().pop()
	}

	/// Add an element to the front of the list.
	#[inline]
	pub fn unshift(&mut self, what: Object) {
		self.0.to_mut().insert(0, what);
	}

	/// Add an element to the end of the list.
	#[inline]
	pub fn shift(&mut self)  -> Option<Object> {
		if self.is_empty() {
			None
		} else {
			Some(self.0.to_mut().remove(0))
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
	fn from(list: List) -> Self {
		list.0.into_owned()
	}
}

impl From<Vec<Object>> for List {
	fn from(list: Vec<Object>) -> Self {
		List::new(list)
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
			t.push(item.call_attr_lit(__INSPECT__, &[])?.downcast_call::<Text>()?.to_string());
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
	fn add_assign(&mut self, mut other: List)  {
		self.0.to_mut().append(other.0.to_mut());
	}
}

impl List {
	#[inline]
	pub fn try_sub(&self, other: List) -> crate::Result<List> {
		let mut dup = self.clone();
		dup.try_sub_assign(other).and(Ok(dup))
	}

	pub fn try_sub_assign(&mut self, other: List) -> crate::Result<()>  {
		let mut i = 0;

		while i < self.len() {
			if let Some(_) = other.find(&self.0[i])? {
				self.0.to_mut().remove(i);
			} else {
				i += 1;
			}
		}

		Ok(())
	}

	#[inline]
	pub fn try_bitand(&self, other: List) -> crate::Result<List> {
		let mut dup = self.clone();
		dup.try_bitand_assign(other).and(Ok(dup))
	}

	pub fn try_bitand_assign(&mut self, mut other: List) -> crate::Result<()>  {
		let mut i = 0;

		while i < self.len() {
			if let Some(j) = other.find(&self.0[i])? {
				other.0.to_mut().remove(j);
				i += 1;
			} else {
				self.0.to_mut().remove(i);
			}
		}

		Ok(())
	}

	#[inline]
	pub fn try_bitor(&self, other: List) -> crate::Result<List> {
		let mut dup = self.clone();
		dup.try_bitor_assign(other).and(Ok(dup))
	}

	pub fn try_bitor_assign(&mut self, other: List) -> crate::Result<()>  {
		let mut i = 0;

		while i < other.len() {
			if !self.find(&other.0[i])?.is_some() {
				self.0.to_mut().push(other.0[i].clone());
			}

			i += 1;
		}

		Ok(())
	}

	#[inline]
	pub fn try_bitxor(&self, other: List) -> crate::Result<List> {
		let mut dup = self.clone();
		dup.try_bitxor_assign(other).and(Ok(dup))
	}

	pub fn try_bitxor_assign(&mut self, mut other: List) -> crate::Result<()>  {
		let mut i = 0;

		while i < other.len() {
			if let Some(j) = self.find(&other.0[i])? {
				other.0.to_mut().remove(i);
				self.0.to_mut().remove(j);
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
	#[inline]
	pub fn qs_at_list(this: &Object, _: Args) -> Result<Object, !> {
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
	#[inline]
	pub fn qs_at_text(&self, _: Args) -> crate::Result<Text> {
		Text::try_from(self)
	}

	/// Attempts to get an internal representation of the list.
	///
	/// # Quest Examples
	/// ```quest
	/// $list = [1, "a", true];
	///
	/// assert(list.$__inspect__() == '[1, "a", true]')
	/// ```
	#[inline]
	#[allow(non_snake_case)]
	pub fn qs___inspect__(&self, args: Args) -> crate::Result<Text> {
		self.qs_at_text(args)
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
	#[inline]
	pub fn qs_at_bool(&self, _: Args) -> Result<Boolean, !> {
		Ok(Boolean::from(self))
	}

	/// Clones this object.
	///
	/// # Quest Examples
	/// ```quest
	/// $list = [1, 2, 3];
	/// $clone = list.$@list();
	///
	/// assert(clone == list);
	/// assert(list.$__id__ != clone.$__id__);
	///
	/// list.$push(4);
	///
	/// assert(clone != list);
	/// ```
	#[inline]
	pub fn qs_clone(&self, _: Args) -> Result<List, !> {
		Ok(self.clone())
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
	#[inline]
	pub fn qs_find(&self, args: Args) -> crate::Result<Object> {
		self.find(args.arg(0)?)
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
	#[inline]
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
	#[inline]
	pub fn qs_len(&self, _: Args) -> Result<usize, !> {
		Ok(self.len())
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
	pub fn qs_get(&self, args: Args) -> crate::Result<Object> {
		let start = args.arg(0)?.downcast_call::<Number>()?.floor() as isize;
		match args.arg(1) {
			Ok(stop) => Ok(self.get_rng(start, stop.downcast_call::<Number>()?.floor() as isize)),
			Err(_) => Ok(self.get(start))
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
	pub fn qs_join(&self, args: Args) -> crate::Result<Text> {
		if let Ok(arg) = args.arg(0) {
			self.join(Some(arg.downcast_call::<Text>()?.as_ref()))
		} else {
			self.join(None)
		}
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
	pub fn qs_eql(&self, args: Args) -> crate::Result<bool> {
		if let Some(rhs) = args.arg(0)?.downcast_ref::<List>() {
			self.eql(&rhs)
		} else {
			Ok(false)
		}
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
		let rhs = args.arg(0)?;
		this.try_downcast_mut::<Self>()?.push(rhs.clone());
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
	#[inline]
	pub fn qs_pop(&mut self, _: Args) -> Result<Object, !> {
		Ok(self.pop().unwrap_or_default())
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
		let rhs = args.arg(0)?;
		this.try_downcast_mut::<Self>()?.unshift(rhs.clone());
		Ok(this.clone())
	}

	/// Remove an element from the front of the list, returning [`Null`](crate::types::Null) if empty.
	///
	/// # Quest Examples
	/// ```quest
	/// $list = ["a", "b"];
	///
	/// assert(list.$pop() == "a");
	/// assert(list.$pop() == "b");
	/// assert(list.$pop() == null);
	/// assert(!list);
	/// ```
	#[inline]
	pub fn qs_shift(&mut self, _: Args) -> Result<Object, !> {
		Ok(self.shift().unwrap_or_default())
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
	pub fn qs_add(&self, args: Args) -> crate::Result<List> {
		let rhs = args.arg(0)?.downcast_call::<Self>()?;
		Ok(self + rhs)
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
		let rhs = args.arg(0)?.downcast_call::<Self>()?;

		*this.try_downcast_mut::<Self>()? += rhs;

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
	pub fn qs_sub(&self, args: Args) -> crate::Result<List> {
		let rhs = args.arg(0)?.downcast_call::<Self>()?;
		self.try_sub(rhs)
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
		let rhs = args.arg(0)?.downcast_call::<Self>()?;

		this.try_downcast_mut::<Self>()?.try_sub_assign(rhs)?;

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
	pub fn qs_bitand(&self, args: Args) -> crate::Result<List> {
		let rhs = args.arg(0)?.downcast_call::<Self>()?;
		self.try_bitand(rhs)
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
		let rhs = args.arg(0)?.downcast_call::<Self>()?;

		this.try_downcast_mut::<Self>()?.try_bitand_assign(rhs)?;

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
	pub fn qs_bitor(&self, args: Args) -> crate::Result<List> {
		let rhs = args.arg(0)?.downcast_call::<Self>()?;
		self.try_bitor(rhs)
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
		let rhs = args.arg(0)?.downcast_call::<Self>()?;

		this.try_downcast_mut::<Self>()?.try_bitor_assign(rhs)?;

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
	pub fn qs_bitxor(&self, args: Args) -> crate::Result<List> {
		let rhs = args.arg(0)?.downcast_call::<Self>()?;
		self.try_bitxor(rhs)
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
		let rhs = args.arg(0)?.downcast_call::<Self>()?;

		this.try_downcast_mut::<Self>()?.try_bitxor_assign(rhs)?;

		Ok(this.clone())
	}
}


impl_object_type!{
for List [(parents super::Basic) (convert "@list")]:
	"@text" => method List::qs_at_text,
	"__inspect__" => method List::qs___inspect__,
	"@bool" => method List::qs_at_bool,
	"@list" => function List::qs_at_list,
	"clone" => method List::qs_clone,

	"clear" => function List::qs_clear,
	"find" => method List::qs_find,
	"len" => method List::qs_len,

	"get" => method List::qs_get,
	"set" => function List::qs_set,
	"join" => method List::qs_join,

	"<<" => function List::qs_push,
	"push" => function List::qs_push,
	"pop" => method_mut List::qs_pop,
	"unshift" => function List::qs_unshift,
	"shift" => method_mut List::qs_shift,

	"=="    => method List::qs_eql,
	"+" => method List::qs_add,
	"+=" => function List::qs_add_assign,
	"-" => method List::qs_sub,
	"-=" => function List::qs_sub_assign,
	"&" => method List::qs_bitand,
	"&=" => function List::qs_bitand_assign,
	"|" => method List::qs_bitor,
	"|=" => function List::qs_bitor_assign,
	"^" => method List::qs_bitxor,
	"^=" => function List::qs_bitxor_assign,
}


use crate::{Object, Args};
use crate::literals::{__INSPECT__};
use crate::types::{Text, Boolean, Number, Null};
use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt::{self, Debug, Formatter};

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
	pub fn iter(&self) -> impl Iterator<Item=&Object> {
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

	/// Find an element in the list, or return `null` if it doesn't exist.
	pub fn find(&self, needle: &Object) -> crate::Result<Object> {
		for (idx, val) in self.iter().enumerate() {
			if val.eq_obj(needle)? {
				return Ok(idx.into());
			}
		}
		Ok(Null::new().into())
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
			t.push(item.call_attr(&__INSPECT__, &[])?.downcast_call::<Text>()?.to_string());
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
	#[inline]
	fn add(self, other: List) -> Self::Output {
		let mut dup = self.clone();
		dup += other;
		dup
	}
}

impl std::ops::AddAssign for List {
	#[inline]
	fn add_assign(&mut self, mut other: List)  {
		self.0.to_mut().append(other.0.to_mut());
	}
}

impl std::ops::Sub<List> for &'_ List {
	type Output = List;
	fn sub(self, _other: List) -> Self::Output {
		todo!()
	}
}

impl std::ops::SubAssign for List {
	#[inline]
	fn sub_assign(&mut self, _other: List)  {
		todo!()
	}
}

impl std::ops::BitAnd<List> for &'_ List {
	type Output = List;
	fn bitand(self, _other: List) -> Self::Output {
		todo!()
	}
}

impl std::ops::BitAndAssign for List {
	#[inline]
	fn bitand_assign(&mut self, _other: List)  {
		todo!()
	}
}

impl std::ops::BitOr<List> for &'_ List {
	type Output = List;
	fn bitor(self, _other: List) -> Self::Output {
		todo!()
	}
}

impl std::ops::BitOrAssign for List {
	#[inline]
	fn bitor_assign(&mut self, _other: List)  {
		todo!()
	}
}

impl std::ops::BitXor<List> for &'_ List {
	type Output = List;
	fn bitxor(self, _other: List) -> Self::Output {
		todo!()
	}
}

impl std::ops::BitXorAssign for List {
	#[inline]
	fn bitxor_assign(&mut self, _other: List)  {
		todo!()
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
	/// If the object isn't in the list, [`Null`] is returned.
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
	///
	/// - [`List::clear`](#method.clear)
	#[inline]
	pub fn qs_clear(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_mut::<List>()?.clear();
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
	/// If the element is out of range, [`Null`](crate::types::Null) is returned. When using the
	/// range form, out-of-bounds `start` and `stop` values are converted to the min/max of the list
	/// (respectively).
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
		this.try_downcast_mut::<List>()?.push(rhs.clone());
		Ok(this.clone())
	}

	#[inline]
	pub fn qs_pop(&mut self, _: Args) -> Result<Object, !> {
		Ok(self.pop().unwrap_or_default())
	}

	pub fn qs_unshift(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?;
		this.try_downcast_mut::<List>()?.unshift(rhs.clone());
		Ok(this.clone())
	}

	#[inline]
	pub fn qs_shift(&mut self, _: Args) -> Result<Object, !> {
		Ok(self.shift().unwrap_or_default())
	}
}

macro_rules! impl_qs_operators {
	($($fn:ident $fn_mut:ident $op:tt $mut_op:tt)*) => {
		impl List {
			$(
				#[inline]
				pub fn $fn(&self, args: Args) -> crate::Result<List> {
					let rhs = args.arg(0)?.downcast_call::<List>()?;
					Ok(self $op rhs)
				}

				pub fn $fn_mut(this: &Object, args: Args) -> crate::Result<Object> {
					let rhs = args.arg(0)?.downcast_call::<List>()?;

					*this.try_downcast_mut::<List>()? $mut_op rhs;

					Ok(this.clone())
				}
			)*
		}
	};
}

impl_qs_operators! { 
	qs_add qs_add_assign + +=
	qs_sub qs_sub_assign - -=
	qs_bitand qs_bitand_assign & &=
	qs_bitor qs_bitor_assign | |=
	qs_bitxor qs_bitxor_assign ^ ^=
}


impl_object_type!{
for List [(parents super::Basic) (convert "@list")]:
	"@text" => method List::qs_at_text,
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


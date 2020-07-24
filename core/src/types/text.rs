use crate::{Object, Args};
use crate::error::ValueError;
use crate::literal::{Literal, __THIS__, __STACK__};
use crate::types::{Number, List, Boolean, Regex};
use crate::Binding;
use std::borrow::Cow;
use std::fmt::{self, Debug, Display, Formatter};
use std::convert::TryFrom;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Text(Cow<'static, str>);

impl Debug for Text {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			f.debug_tuple("Text").field(&self.as_ref()).finish()
		} else {
			Debug::fmt(&self.as_ref(), f)
		}
	}
}

impl Display for Text {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl Text {
	#[inline]
	pub fn new(txt: String) -> Self {
		Self(Cow::Owned(txt))
	}

	#[inline]
	pub const fn new_static(txt: Literal) -> Self {
		Self(Cow::Borrowed(txt))
	}

	pub fn evaluate(&self) -> crate::Result<Object> {
		match self.as_ref() {
			__THIS__ => Ok(Binding::instance().as_ref().clone()),
			__STACK__ => Ok(Binding::stack().into_iter().map(Object::from).collect::<Vec<_>>().into()),
			_ => Binding::instance().as_ref().dot_get_attr(&self.to_string().into())
		}
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.0.len()
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}


	#[inline]
	pub fn into_inner(self) -> Cow<'static, str> {
		self.0
	}
}

impl PartialEq<str> for Text {
	#[inline]
	fn eq(&self, rhs: &str) -> bool {
		self.as_ref() == rhs
	}
}


impl From<Literal> for Text {
	#[inline]
	fn from(txt: Literal) -> Self {
		Self::new_static(txt)
	}
}

impl From<Text> for String {
	#[inline]
	fn from(txt: Text) -> Self {
		txt.into_inner().into_owned()
	}
}

impl From<char> for Text {
	#[inline]
	fn from(c: char) -> Self {
		c.to_string().into()
	}
}

impl From<char> for Object {
	#[inline]
	fn from(c: char) -> Self {
		Self::from(c.to_string())
	}
}

impl From<String> for Text {
	#[inline]
	fn from(txt: String) -> Self {
		Self::new(txt)
	}
}

impl From<String> for Object {
	#[inline]
	fn from(txt: String) -> Self {
		Text::from(txt).into()
	}
}

impl From<Literal> for Object {
	#[inline]
	fn from(txt: Literal) -> Self {
		Text::from(txt).into()
	}
}

impl AsRef<str> for Text {
	#[inline]
	fn as_ref(&self) -> &str {
		self.0.as_ref()
	}
}

impl std::ops::Add<&Self> for Text {
	type Output = Self;

	fn add(self, rhs: &Self) -> Self {
		Self::from(self.0.into_owned() + rhs.as_ref())
	}
}

impl std::ops::AddAssign<&Self> for Text {
	fn add_assign(&mut self, rhs: &Self) {
		*self.0.to_mut() += rhs.as_ref();
	}
}

impl From<&'_ Text> for List {
	fn from(text: &Text) -> Self {
		text.as_ref()
			.chars()
			.map(|chr| chr.to_string().into())
			.collect()
	}
}

impl From<&'_ Text> for Boolean {
	fn from(text: &Text) -> Self {
		(!text.as_ref().is_empty()).into()
	}
}

impl From<&'_ Text> for Text {
	fn from(text: &'_ Text) -> Self {
		text.clone()
	}
}

impl<'a> TryFrom<&'a Text> for Number {
	type Error = <Number as TryFrom<&'a str>>::Error;
	fn try_from(text: &Text) -> Result<Self, Self::Error> {
		if text.as_ref().is_empty() {
			Ok(Self::ZERO)
		} else {
			Self::try_from(text.as_ref())
		}
	}
}

impl Text {
	fn shift(&mut self) -> Option<char> {
		if self.is_empty() {
			None
		} else {
			Some(self.0.to_mut().remove(0))
		}
	}

	fn inspect(&self) -> Self {
		format!("{:?}", self.0).into()
	}

	// fn unshift(&mut self, val: Object) {
	// 	self.0.to_mut().insert(0, val);
	// }

	fn pop(&mut self) -> Option<char> {
		self.0.to_mut().pop()
	}

	fn push_str(&mut self, s: &str) {
		self.0.to_mut().push_str(s);
	}

	fn clear(&mut self) {
		self.0.to_mut().clear()
	}
}

impl Text {
	pub fn qs_at_text(this: &Object, _: Args) -> crate::Result<Object> {
		Ok(this.clone())
	}

	pub fn qs_at_regex(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_and_then(|this: &Self| {
			Regex::try_from(this.as_ref())
				.map(Object::from)
				.map_err(|err| crate::Error::Messaged(err.to_string()))
		})
	}

	pub fn qs_inspect(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_map(Self::inspect).map(Object::from)
	}

	pub fn qs_at_list(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_map(|this: &Self| List::from(this)).map(Object::from)
	}

	pub fn qs_at_bool(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_map(|this: &Self| Boolean::from(this)).map(Object::from)
	}

	pub fn qs_at_num(this: &Object, args: Args) -> crate::Result<Object> {
		this.try_downcast_and_then(|this: &Self|
			if let Ok(radix) = args.arg(0) {
				let radix = radix.call_downcast_map(Number::clone)?;
				let radix = u32::try_from(radix)
					.map_err(|err| ValueError::Messaged(format!("bad radix '{}': {}", radix, err)))?;

				Number::from_str_radix(this.as_ref(), radix)
					.map(Object::from)
					.map_err(|err| crate::Error::from(ValueError::Messaged(
						format!("cant convert: {}", err))))
			} else {
				Number::try_from(this)
					.map(Object::from)
					.map_err(|err| crate::Error::from(ValueError::Messaged(err.to_string())))
			}
		)
	}

	pub fn qs_call(this: &Object, _: Args) -> crate::Result<Object> {
		this.downcast_and_then(Self::evaluate)
			.unwrap_or_else(|| Binding::instance().as_ref().dot_get_attr(this))
	}

	pub fn qs_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.clone();

		if this.downcast_and_then(|this: &Self| this == __THIS__).unwrap_or(false) {
			return Ok(Binding::set_binding(rhs).into())
		}

		Binding::instance().set_attr(this.clone(), rhs.clone()).and(Ok(rhs))
	}

	pub fn qs_eql(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?;
		this.try_downcast_map(|this: &Self| {
			rhs.downcast_and_then(|rhs: &Self| this == rhs).unwrap_or(false).into()
		})
	}

	pub fn qs_cmp(this: &Object, args: Args) -> crate::Result<Object> {
		this.try_downcast_and_then(|this: &Self| {
			args.arg(0)?.call_downcast_map(|rhs: &Self| this.cmp(rhs).into())
		})
	}

	pub fn qs_add(this: &Object, args: Args) -> crate::Result<Object> {
		this.try_downcast_and_then(|this: &Self| {
			args.arg(0)?.call_downcast_map(|rhs: &Self| (this.clone() + rhs).into())
		})
	}

	pub fn qs_add_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?;

		this.try_downcast_mut_and_then(|this_text: &mut Self| {
			if this.is_identical(rhs) {
				*this_text += &this_text.clone();
				Ok(())
			} else {
				rhs.call_downcast_map(|rhs: &Self| {
					*this_text += rhs;
				})
			}
		}).map(|_| this.clone())
	}

	pub fn qs_len(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_map(Self::len).map(Object::from)
	}

	fn correct_index(&self, idx: isize) -> Option<usize> {
		if !idx.is_negative() {
			if (idx as usize) < self.len() {
				Some(idx as usize)
			} else {
				None
			}
		} else {
			let idx = (-idx) as usize;
			if idx <= self.len() {
				Some(self.len() - idx)
			} else {
				None
			}
		}
	}

	pub fn qs_get(this: &Object, args: Args) -> crate::Result<Object> {
		this.try_downcast_and_then(|this: &Self| {
			let start = args.arg(0)?
				.try_downcast_and_then(|n: &Number| Ok(isize::try_from(*n)?))?;

			let end = args.arg(1)
				.ok()
				.map(|n| n.call_downcast_map(Number::clone))
				.transpose()?
				.map(isize::try_from)
				.transpose()?;

			let start =
				if let Some(start) = this.correct_index(start) {
					start
				} else {
					return Ok(Object::default())
				};

			match end {
				None =>
					Ok(this.0.chars()
						.nth(start)
						.map(|x| x.to_string().into())
						.unwrap_or_default()),
				Some(end) => {
					let end = this.correct_index(end).map(|x| x + 1).unwrap_or_else(|| this.len());
					if end < start {
						Ok(Object::default())
					} else {
						Ok(this.0[start..end].to_owned().into())
					}
				}
			}
		})
	}

	pub fn qs_set(_this: &Object, _args: Args) -> crate::Result<Object> {
		todo!()
	}

	pub fn qs_push(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?;

		this.try_downcast_mut_and_then(|this_text: &mut Self| {
			if this.is_identical(rhs) {
				this_text.push_str(this_text.clone().as_ref());
				Ok(())
			} else {
				rhs.call_downcast_map(|rhs: &Self| this_text.push_str(rhs.as_ref()))
			}
		}).map(|_| this.clone())
	}

	pub fn qs_pop(this: &Object, _args: Args) -> crate::Result<Object> {
		this.try_downcast_mut_map(|this: &mut Self| {
			this.pop()
				.map(|c| c.to_string())
				.map(Object::from)
				.unwrap_or_default()
		})
	}

	pub fn qs_unshift(_this: &Object, _args: Args) -> crate::Result<Object> {
		todo!()
	}

	pub fn qs_shift(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_mut_map(Self::shift)
			.map(|x| x.map(Object::from).unwrap_or_default())
	}

	pub fn qs_clear(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_mut_map(Self::clear)
			.map(|_| this.clone())
	}

	pub fn qs_split(_this: &Object, _: Args) -> crate::Result<Object> { todo!("split") }
	pub fn qs_reverse(_this: &Object, _: Args) -> crate::Result<Object> { todo!("reverse") }
}

impl_object_type!{
for Text 
{
	fn new_object(self) -> Object {
		use lazy_static::lazy_static;
		use std::collections::HashMap;
		use parking_lot::RwLock;

		lazy_static! {
			static ref OBJECTS: RwLock<HashMap<Text, Object>> = RwLock::new(HashMap::new());
		}

		// this is a hack until I get `quest_core::init()` working
		if self.as_ref().starts_with(|x| 'A' <= x && x <= 'Z') {
			return Object::new_with_parent(self, vec![Text::mapping()]);
		}

		if let Some(obj) = OBJECTS.read().get(&self) {
			return obj.deep_clone();
		}

		let mut objs = OBJECTS.write();

		objs.entry(self.clone())
			.or_insert_with(|| Object::new_with_parent(self, vec![Text::mapping()]))
			.deep_clone()
	}
}
[(init_parent super::Basic super::Comparable) (parents super::Basic) (convert "@text")]:
	"@text" => function Text::qs_at_text,
	"@regex" => function Text::qs_at_regex,
	"inspect"  => function Text::qs_inspect,
	"@num"    => function Text::qs_at_num,
	"@list"   => function Text::qs_at_list,
	"@bool"   => function Text::qs_at_bool,
	"()"      => function Text::qs_call,

	"="       => function Text::qs_assign,
	"<=>"     => function Text::qs_cmp,
	"=="      => function Text::qs_eql,
	"+"       => function Text::qs_add,
	"+="      => function Text::qs_add_assign,

	"len"     => function Text::qs_len,
	"get"     => function Text::qs_get,
	"set"     => function Text::qs_set,
	"push"    => function Text::qs_push,
	"pop"     => function Text::qs_pop,
	"unshift" => function Text::qs_unshift,
	"shift"   => function Text::qs_shift,
	"clear"   => function Text::qs_clear,
	"split"   => function Text::qs_split,
	"reverse" => function Text::qs_reverse,
	// "strip"   => function Text::qs_strip,
}

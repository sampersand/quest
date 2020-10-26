use crate::{Object, Args, Literal};
use crate::error::ValueError;
use crate::types::{Number, List, Boolean, Regex};
use crate::Binding;
use std::borrow::Cow;
use std::fmt::{self, Debug, Display, Formatter};
use std::convert::TryFrom;
use tracing::instrument;

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
	pub fn new(txt: impl Into<String>) -> Self {
		Self(Cow::Owned(txt.into()))
	}

	#[inline]
	pub const fn const_new(txt: &'static str) -> Self {
		Self(Cow::Borrowed(txt))
	}

	pub fn evaluate(&self) -> crate::Result<Object> {
		match self.as_ref() {
			s if Literal::__STACK__ == s => Ok(Binding::stack().into_iter().map(Object::from).collect::<Vec<_>>().into()),
			_ => Binding::instance().as_ref().get_attr(&self.to_string().into())
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
		Self::const_new(txt.into_inner())
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

impl From<&str> for Object {
	#[inline]
	fn from(c: &str) -> Self {
		Text::from(c).into()
	}
}

impl From<String> for Text {
	#[inline]
	fn from(txt: String) -> Self {
		Self::new(txt)
	}
}

impl From<&str> for Text {
	#[inline]
	fn from(txt: &str) -> Self {
		Self::new(txt)
	}
}

impl From<String> for Object {
	#[inline]
	fn from(txt: String) -> Self {
		Text::from(txt).into()
	}
}

impl AsRef<str> for Text {
	#[inline]
	fn as_ref(&self) -> &str {
		self.0.as_ref()
	}
}

impl AsMut<String> for Text {
	fn as_mut(&mut self) -> &mut String {
		self.0.to_mut()
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
		*self.as_mut() += rhs.as_ref();
	}
}

impl From<&Text> for List {
	fn from(text: &Text) -> Self {
		text.as_ref()
			.chars()
			.map(|chr| chr.to_string().into())
			.collect()
	}
}

impl From<&Text> for Boolean {
	fn from(text: &Text) -> Self {
		(!text.as_ref().is_empty()).into()
	}
}

impl From<&Text> for Text {
	fn from(text: &Text) -> Self {
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
	pub fn shift(&mut self) -> Option<char> {
		if self.is_empty() {
			None
		} else {
			Some(self.as_mut().remove(0))
		}
	}

	pub fn inspect(&self) -> Self {
		format!("{:?}", self.0).into()
	}

	pub fn unshift(&mut self, val: &str) {
		self.as_mut().insert_str(0, val);
	}

	pub fn pop(&mut self) -> Option<char> {
		self.as_mut().pop()
	}

	pub fn push_str(&mut self, s: &str) {
		self.as_mut().push_str(s);
	}

	pub fn clear(&mut self) {
		self.as_mut().clear()
	}

	pub fn reverse(&self) -> Self {
		self.0.as_ref().chars().rev().collect()
	}

	pub fn strip(&self) -> Self {
		self.0.as_ref().trim().into()
	}

	pub fn replace(&mut self, with: &str) {
		self.as_mut().clear();
		self.as_mut().push_str(with);
	}

	pub fn split(&self, on: Option<&str>) -> Vec<String> {
		if let Some(on) = on {
			self.0.split(on).map(ToOwned::to_owned).collect()
		} else {
			self.0.chars().map(|c| c.to_string()).collect()
		}
	}
}

impl std::iter::FromIterator<char> for Text {
	fn from_iter<T: IntoIterator<Item=char>>(iter: T) -> Self {
		Self::new(iter.into_iter().collect::<String>())
	}
}

impl Text {
	#[instrument(name="Text::@text", level="trace", skip(this), fields(self=?this))]
	pub fn qs_at_text(this: &Object, _: Args) -> crate::Result<Object> {
		Ok(this.clone())
	}

	#[instrument(name="Text::@regex", level="trace", skip(this), fields(self=?this))]
	pub fn qs_at_regex(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Regex::try_from(this.as_ref())
			.map(Object::from)
			.map_err(|err| crate::Error::Messaged(err.to_string()))
	}

	#[instrument(name="Text::inspect", level="trace", skip(this), fields(self=?this))]
	pub fn qs_inspect(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(this.inspect().into())
	}

	#[instrument(name="Text::@list", level="trace", skip(this), fields(self=?this))]
	pub fn qs_at_list(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(List::from(&*this).into())
	}

	#[instrument(name="Text::@bool", level="trace", skip(this), fields(self=?this))]
	pub fn qs_at_bool(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(Boolean::from(&*this).into())
	}

	#[instrument(name="Text::@num", level="trace", skip(this), fields(self=?this))]
	pub fn qs_at_num(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		if let Some(radix) = args.arg(0) {
			let radix = *radix.call_downcast::<Number>()?;
			let radix = u32::try_from(radix)
				.map_err(|err| ValueError::Messaged(format!("bad radix '{}': {}", radix, err)))?;

			Number::from_str_radix(this.as_ref(), radix)
				.map(Object::from)
				.map_err(|err| crate::Error::from(ValueError::Messaged(
					format!("cant convert: {}", err))))
		} else {
			Number::try_from(&*this)
				.map(Object::from)
				.map_err(|err| crate::Error::from(ValueError::Messaged(err.to_string())))
		}
	}

	#[instrument(name="Text::()", level="trace", skip(this), fields(self=?this))]
	pub fn qs_call(this: &Object, _: Args) -> crate::Result<Object> {
		if let Some(this) = this.downcast::<Self>() {
			this.evaluate()
		} else {
			Binding::instance().as_ref().dot_get_attr(this)
		}
	}

	#[instrument(name="Text::=", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?.clone();

		// if this.downcast::<Self>().map(|this| Literal::__THIS__ == this.as_ref()).unwrap_or(false) {
		// 	return Ok(Binding::set_binding(rhs).into())
		// }

		Binding::instance().set_attr(this.clone(), rhs.clone()).and(Ok(rhs))
	}

	#[instrument(name="Text::==", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_eql(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?.downcast::<Self>();
		let this = this.try_downcast::<Self>()?;

		Ok(rhs.map(|rhs| *this == *rhs).unwrap_or(false).into())
	}

	#[instrument(name="Text::<=>", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_cmp(this: &Object, args: Args) -> crate::Result<Object> {
		let arg = args.try_arg(0)?.call_downcast::<Self>();
		let this = this.try_downcast::<Self>()?;

		Ok(arg.map(|a| this.cmp(&a).into()).unwrap_or_default())
	}

	#[instrument(name="Text::+", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_add(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?.call_downcast::<Self>()?;
		let this = this.try_downcast::<Self>()?;

		Ok((this.clone() + &rhs).into())
	}

	#[instrument(name="Text::+=", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_add_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?;
		let mut this_mut = this.try_downcast_mut::<Self>()?;

		if this.is_identical(rhs) {
			let dup = this_mut.clone();
			*this_mut += &dup;
		} else {
			*this_mut += &*rhs.call_downcast::<Self>()?;
		}

		Ok(this.clone())
	}

	#[instrument(name="Text::len", level="trace", skip(this), fields(self=?this))]
	pub fn qs_len(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(this.len().into())
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

	#[instrument(name="Text::get", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_get(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		let start: isize = isize::try_from(*args.try_arg(0)?.try_downcast::<Number>()?)?;

		let end = args.arg(1)
			.map(|n| n.call_downcast::<Number>().map(|n| *n))
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
	}

	#[instrument(name="Text::set", level="trace", skip(_this, _args), fields(self=?_this, args=?_args))]
	pub fn qs_set(_this: &Object, _args: Args) -> crate::Result<Object> {
		todo!()
	}

	#[instrument(name="Text::push", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_push(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?;
		let mut this_mut = this.try_downcast_mut::<Self>()?;

		if this.is_identical(rhs) {
			let dup = this_mut.clone();
			this_mut.push_str(dup.as_ref());
		} else {
			this_mut.push_str(rhs.call_downcast::<Self>()?.as_ref());
		}

		Ok(this.clone())
	}

	#[instrument(name="Text::pop", level="trace", skip(this), fields(self=?this))]
	pub fn qs_pop(this: &Object, _: Args) -> crate::Result<Object> {
		Ok(this.try_downcast_mut::<Self>()?
			.pop()
			.map(|c| c.to_string())
			.map(Object::from)
			.unwrap_or_default())
	}

	#[instrument(name="Text::unshift", level="trace", skip(this, args), fields(self=?this, args=?args))]
	pub fn qs_unshift(this: &Object, args: Args) -> crate::Result<Object> {
		let arg = args.try_arg(0)?;

		let is_identical = arg.is_identical(this);

		{
			let mut this = this.try_downcast_mut::<Self>()?;
			if is_identical {
				let dup = this.to_string();
				this.unshift(&dup);
			} else {
				this.unshift(arg.call_downcast::<Self>()?.as_ref());
			}
		}

		Ok(this.clone())
	}

	#[instrument(name="Text::shift", level="trace", skip(this), fields(self=?this))]
	pub fn qs_shift(this: &Object, _: Args) -> crate::Result<Object> {
		Ok(this.try_downcast_mut::<Self>()?
			.shift()
			.map(Object::from)
			.unwrap_or_default())
	}

	#[instrument(name="Text::clear", level="trace", skip(this), fields(self=?this))]
	pub fn qs_clear(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_mut::<Self>()?.clear();

		Ok(this.clone())
	}

	#[instrument(name="Text::split", level="trace", skip(this, args), fields(self=?this, args=?args))]
	pub fn qs_split(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(
			if let Some(on) = args.arg(0) {
				this.split(Some(on.call_downcast::<Self>()?.as_ref()))
			} else {
				this.split(None)
			}.into_iter().map(Object::from).collect::<List>().into()
		)
	}

	#[instrument(name="Text::reverse", level="trace", skip(this), fields(self=?this))]
	pub fn qs_reverse(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(this.reverse().into())
	}

	#[instrument(name="Text::strip", level="trace", skip(this), fields(self=?this))]
	pub fn qs_strip(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(this.strip().into())
	}

	#[instrument(name="Text::replace", level="trace", skip(this), fields(self=?this))]
	pub fn qs_replace(this: &Object, args: Args) -> crate::Result<Object> {
		let arg = args.try_arg(0)?;

		if this.is_identical(arg) {
			return Ok(this.clone());
		}

		this.try_downcast_mut::<Self>()?.replace(arg.call_downcast::<Self>()?.as_ref());

		Ok(this.clone())
	}

	#[instrument(name="Text::each", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_each(this: &Object, args: Args) -> crate::Result<Object> {
		let block = args.try_arg(0)?;

		for idx in 0.. {
			// so as to not lock the object, we check the index each and every time.
			// this allows it to be modified during the `each` invocation.
			let obj = {
				let this = this.try_downcast::<Self>()?;
				if let Some(c) = this.as_ref().chars().nth(idx) {
					c.into()
				} else {
					break
				}
			};

			block.call_attr_lit(&Literal::CALL, &[&obj])?;
		}

		Ok(this.clone())
	}
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
[(init_parent super::Basic super::Comparable super::Iterable) (parents super::Basic) (convert "@text")]:
	"@text" => method Text::qs_at_text,
	"@regex" => method Text::qs_at_regex,
	"inspect"  => method Text::qs_inspect,
	"@num"    => method Text::qs_at_num,
	"@list"   => method Text::qs_at_list,
	"@bool"   => method Text::qs_at_bool,
	"()"      => method Text::qs_call,

	"="       => method Text::qs_assign,
	"<=>"     => method Text::qs_cmp,
	"=="      => method Text::qs_eql,
	"+"       => method Text::qs_add,
	"+="      => method Text::qs_add_assign,

	"len"     => method Text::qs_len,
	"get"     => method Text::qs_get,
	"set"     => method Text::qs_set,
	"push"    => method Text::qs_push,
	"pop"     => method Text::qs_pop,
	"unshift" => method Text::qs_unshift,
	"shift"   => method Text::qs_shift,
	"clear"   => method Text::qs_clear,
	"split"   => method Text::qs_split,
	"reverse" => method Text::qs_reverse,
	"each"    => method Self::qs_each,
	"strip"   => method Text::qs_strip,
	"replace" => method Text::qs_replace,

	"includes" => method |this, args| {
		let this = this.try_downcast::<Self>()?;
		let rhs = args.try_arg(0)?.call_downcast::<Self>()?;

		Ok(this.as_ref().contains(rhs.as_ref()).into())
	}
}

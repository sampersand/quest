use crate::{Object, Args};
use crate::error::{ValueError};
use crate::literals::{__THIS__, __STACK__};
use crate::types::{Number, List, Boolean};
use crate::Binding;
use std::borrow::Cow;
use std::fmt::{self, Debug, Display, Formatter};
use std::convert::TryFrom;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Text(Cow<'static, str>);

impl Debug for Text {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			write!(f, "Text({:?})", self.as_ref())
		} else {
			Debug::fmt(&self.as_ref(), f)
		}
	}
}

impl Display for Text {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl Text {
	#[inline]
	pub fn new(txt: String) -> Self {
		Text(Cow::Owned(txt))
	}

	#[inline]
	pub const fn new_static(txt: &'static str) -> Self {
		Text(Cow::Borrowed(txt))
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
	pub fn into_inner(self) -> Cow<'static, str> {
		self.0
	}
}

impl From<&'static str> for Text {
	#[inline]
	fn from(txt: &'static str) -> Self {
		Text::new_static(txt)
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

impl From<String> for Text {
	#[inline]
	fn from(txt: String) -> Self {
		Text::new(txt)
	}
}

impl From<String> for Object {
	#[inline]
	fn from(txt: String) -> Self {
		Text::from(txt).into()
	}
}

impl From<&'static str> for Object {
	#[inline]
	fn from(txt: &'static str) -> Self {
		Text::from(txt).into()
	}
}

impl crate::ToObject for str {
	fn to_object(&self) -> Object {
		Object::from(Text::from(self.to_string()))
	}
}

impl AsRef<str> for Text {
	#[inline]
	fn as_ref(&self) -> &str {
		self.0.as_ref()
	}
}

impl std::ops::Add for Text {
	type Output = Text;
	fn add(self, rhs: Text) -> Self {
		Self::from(self.0.into_owned() + rhs.as_ref())
	}
}

impl std::ops::AddAssign for Text {
	fn add_assign(&mut self, rhs: Text) {
		*self.0.to_mut() += rhs.as_ref();
	}
}

impl From<&'_ Text> for List {
	fn from(text: &Text) -> Self {
		text.as_ref()
			.chars()
			.map(|chr| chr.to_string().into())
			.collect::<Vec<Object>>()
			.into()
	}
}

impl From<&'_ Text> for Boolean {
	fn from(text: &'_ Text) -> Self {
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
	#[inline]
	pub fn qs_at_text(this: &Object, _: Args) -> Result<Object, !> {
		Ok(this.clone())
	}

	#[allow(non_snake_case)]
	pub fn qs___inspect__(&self, _: Args) -> Result<Self, !> {
		Ok(format!("{:?}", self).into())
	}

	#[inline]
	pub fn qs_at_list(&self, _: Args) -> Result<List, !> {
		Ok(List::from(self))
	}

	#[inline]
	pub fn qs_at_bool(&self, _: Args) -> Result<Boolean, !> {
		Ok(Boolean::from(self))
	}

	pub fn qs_at_num(&self, args: Args) -> crate::Result<Number> {
		if let Ok(radix) = args.arg(0) {
			let radix = radix.downcast_call::<Number>()?;
			let radix = u32::try_from(radix)
				.map_err(|err| ValueError::Messaged(format!("bad radix '{}': {}", radix, err)))?;

			Number::from_str_radix(self.as_ref(), radix)
				.map_err(|err| ValueError::Messaged(format!("cant convert: {}", err)).into())
		} else {
			Number::try_from(self)
				.map_err(|err| ValueError::Messaged(err.to_string()).into())
		}
	}

	#[inline]
	pub fn qs_clone(&self, _: Args) -> Result<Self, !> {
		Ok(self.clone())
	}

	pub fn qs_call(this: &Object, _: Args) -> crate::Result<Object> {
		if let Ok(this) = this.try_downcast_ref::<Self>() {
			this.evaluate()
		} else {
			Binding::instance().as_ref().dot_get_attr(this)
		}
	}

	pub fn qs_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.clone();

		if let Some(this) = this.downcast_ref::<Self>() {
			if this.as_ref() == __THIS__ {
				return Ok(Binding::set_binding(rhs).into())
			}
		}

		Binding::instance().set_attr(this.clone(), rhs.clone()).and(Ok(rhs))
	}

	pub fn qs_eql(&self, args: Args) -> Result<bool, crate::error::KeyError> {
		if let Some(rhs) = args.arg(0)?.downcast_ref::<Self>() {
			Ok(*self == *rhs)
		} else {
			Ok(false)
		}
	}

	#[inline]
	pub fn qs_cmp(&self, args: Args) -> crate::Result<std::cmp::Ordering> {
		let rhs = args.arg(0)?.downcast_call::<Self>()?;
		Ok(self.cmp(&rhs))
	}

	pub fn qs_add(&self, args: Args) -> crate::Result<Self> {
		let rhs = args.arg(0)?.downcast_call::<Self>()?;
		Ok(self.clone() + rhs)
	}

	pub fn qs_add_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.downcast_call::<Self>()?;

		this.try_with_mut(|txt: &mut Self| Ok(*txt += rhs))?;

		Ok(this.clone())
	}

	#[inline]
	pub fn qs_len(&self, _: Args) -> Result<usize, !> {
		Ok(self.len())
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

	pub fn qs_get(&self, args: Args) -> crate::Result<Object> {
		let start = args.arg(0)?
			.try_downcast_ref::<Number>()?
			.floor() as isize;

		let end = args.arg(1)
			.ok()
			.map(Object::downcast_call::<Number>)
			.transpose()?
			.map(|x| x.floor() as isize);

		let start =
			if let Some(start) = self.correct_index(start) {
				start
			} else {
				return Ok(Object::default())
			};

		match end {
			None =>
				Ok(self.0.chars()
					.nth(start)
					.map(|x| x.to_string().into())
					.unwrap_or_default()),
			Some(end) => {
				let end = self.correct_index(end).map(|x| x + 1).unwrap_or(self.len());
				if end < start {
					Ok(Object::default())
				} else {
					Ok(self.0[start..end].to_owned().into())
				}
			}
		}
	}

	pub fn qs_set(_this: &Object, _args: Args) -> crate::Result<Object> {
		todo!()
	}

	pub fn qs_push(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.downcast_call::<Self>()?;

		this.try_with_mut(|txt: &mut Self| Ok(txt.0.to_mut().push_str(rhs.as_ref())))?;

		Ok(this.clone())
	}

	pub fn qs_pop(&mut self, _args: Args) -> crate::Result<Object> {
		todo!()
	}

	pub fn qs_unshift(_this: &Object, _args: Args) -> crate::Result<Object> {
		todo!()
	}

	pub fn qs_shift(&mut self, _: Args) -> crate::Result<Text> {
		if self.len() == 0 {
			Ok(Text::default())
		} else {
			Ok(self.0.to_mut().remove(0).into())
		}
	}


	pub fn qs_clear(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_with_mut(|this: &mut Self| Ok(this.0.to_mut().clear()))?;

		Ok(this.clone())
	}

	pub fn qs_split(&self, _: Args) -> crate::Result<Object> { todo!("split") }
	pub fn qs_reverse(&self, _: Args) -> crate::Result<Object> { todo!("reverse") }

	pub fn qs_match(&self, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.downcast_call::<Self>()?;
		let re = regex::Regex::new(rhs.as_ref()).expect("bad regex");
		Ok(re.is_match(self.as_ref()).into())
	}
}

impl_object_type!{
for Text 
{
	fn new_object(self) -> Object where Self: Sized {
		use lazy_static::lazy_static;
		use std::collections::HashMap;
		use std::sync::RwLock;

		lazy_static! {
			static ref OBJECTS: RwLock<HashMap<Text, Object>> = RwLock::new(HashMap::new());
		}

		// this is a hack until I get `quest_core::init()` working
		if self.as_ref().starts_with(|x| 'A' <= x && x <= 'Z') {
			return Object::new_with_parent(self, vec![Text::mapping()]);
		}

		if let Some(obj) = OBJECTS.read().unwrap().get(&self) {
			return obj.deep_clone();
		}

		let mut objs = OBJECTS.write().unwrap();

		objs.entry(self.clone())
			.or_insert_with(|| Object::new_with_parent(self, vec![Text::mapping()]))
			.deep_clone()
	}
}
[(init_parent super::Basic super::Comparable) (parents super::Basic) (convert "@text")]:
	"@text" => function Text::qs_at_text,
	"__inspect__"  => method Text::qs___inspect__,
	"@num"    => method Text::qs_at_num,
	"@list"   => method Text::qs_at_list,
	"@bool"   => method Text::qs_at_bool,
	"clone"   => method Text::qs_clone,
	"()"      => function Text::qs_call,

	"="       => function Text::qs_assign,
	"<=>"     => method Text::qs_cmp,
	"=="      => method Text::qs_eql,
	"+"       => method Text::qs_add,
	"+="      => function Text::qs_add_assign,

	"len"     => method Text::qs_len,
	"get"     => method Text::qs_get,
	"set"     => function Text::qs_set,
	"push"    => function Text::qs_push,
	"pop"     => method_mut Text::qs_pop,
	"unshift" => function Text::qs_unshift,
	"shift"   => method_mut Text::qs_shift,
	"clear"   => function Text::qs_clear,
	"split"   => method_mut Text::qs_split,
	"reverse" => method Text::qs_reverse,
	"match" => method Text::qs_match
	// "strip"   => function Text::qs_strip,
}

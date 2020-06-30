use crate::{Object, Args};
use crate::error::{ValueError};
use crate::literals::__THIS__;
use crate::types::{Number, List, Boolean};
use crate::Binding;
use std::borrow::Cow;
use std::fmt::{self, Debug, Display, Formatter};
use std::convert::TryFrom;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
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
			"__this__" => Ok(Binding::instance().as_ref().clone()),
			"__args__" => Binding::instance().get_attr_old("__args__"),
			"__stack__" => Ok(Binding::with_stack(|s| {
				let mut stack = s.read().expect("couldn't read stack")
					.iter()
					.map(|x| x.as_ref().clone())
					.collect::<Vec<_>>();
				stack.reverse();
				stack.into()
			})),
			_ => Binding::instance().as_ref().dot_get_attr(self.as_ref())
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
		Self::try_from(text.as_ref())
	}
}


impl Text {
	#[inline]
	pub fn qs_at_text(this: &Object, _: Args) -> Result<Object, !> {
		Ok(this.clone())
	}

	#[allow(non_snake_case)]
	pub fn qs___inspect__(&self, _: Args) -> Result<Text, !> {
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
	pub fn qs_clone(&self, _: Args) -> Result<Text, !> {
		Ok(self.clone())
	}

	pub fn qs_call(this: &Object, _: Args) -> crate::Result<Object> {
		if let Ok(this) = this.try_downcast_ref::<Text>() {
			this.evaluate()
		} else {
			Binding::instance().as_ref().dot_get_attr(this)
		}
	}

	pub fn qs_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.clone();

		if let Some(this) = this.downcast_ref::<Text>() {
			if this.as_ref() == __THIS__ {
				return Ok(Binding::set_binding(rhs).as_ref().clone())
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
		let rhs = args.arg(0)?.downcast_call::<Text>()?;
		Ok(self.cmp(&rhs))
	}

	pub fn qs_add(&self, args: Args) -> crate::Result<Text> {
		let rhs = args.arg(0)?.downcast_call::<Text>()?;
		Ok(self.clone() + rhs)
	}

	pub fn qs_add_assign(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?.downcast_call::<Text>()?;
		*this.try_downcast_mut::<Text>()? += rhs;
		Ok(this.clone())
	}

	#[inline]
	pub fn qs_len(&self, _: Args) -> Result<usize, !> {
		Ok(self.len())
	}
}

mod impls {
	use super::*;
	use crate::{Object, Result, ArgsOld, types};
	// "[]~" => (|args| todo!("[]~")),
	// "clear" => (|args| todo!("clear")),

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

	pub fn index(args: ArgsOld) -> Result<Object> {
		let this = args.this()?.try_downcast_ref::<Text>()?;

		let len = this.0.len();
		let start = args.arg(0)?
			.try_downcast_ref::<types::Number>()?
			.floor() as isize;

		let end = args.arg(1)
			.ok()
			.map(Object::downcast_call::<types::Number>)
			.transpose()?
			.map(|x| x.floor() as isize);

		let start =
			if let Some(start) = correct_index(start, len) {
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
				let end = correct_index(end, len).map(|x| x + 1).unwrap_or(len);
				if end < start {
					Ok(Object::default())
				} else {
					Ok(this.0[start..end].to_owned().into())
				}
			}
		}
	}

	pub fn shift(args: ArgsOld) -> Result<Object> {
		let this = &mut args.this()?.try_downcast_mut::<types::Text>()?.0;
		if this.len() == 0 {
			Ok(Object::default())
		} else {
			Ok(this.to_mut().remove(0).to_string().into())
		}
	}

	pub fn push(args: ArgsOld) -> Result<Object> {
		let this_obj = args.this()?;
		let arg = args.arg(0)?.downcast_call::<types::Text>()?.0;
		this_obj.try_downcast_mut::<types::Text>()?.0.to_mut().push_str(arg.as_ref());
		Ok(this_obj.clone())
	}

	pub fn clear(args: ArgsOld) -> Result<Object> {
		let this_obj = args.this()?;
		this_obj.try_downcast_mut::<types::Text>()?.0.to_mut().clear();
		Ok(this_obj.clone())
	}

	pub fn index_assign(_args: ArgsOld) -> Result<Object> { todo!("[]=") } // "[]=
	pub fn index_of(_args: ArgsOld) -> Result<Object> { todo!("index_of") } // "index_of"
	pub fn split(_args: ArgsOld) -> Result<Object> { todo!("split") } // "split"
	pub fn reverse(_args: ArgsOld) -> Result<Object> { todo!("reverse") } // "reverse"
}

impl_object_type!{
for Text [(init_parent super::Basic super::Comparable) (parents super::Basic) (convert "@text")]:
	"@text" => function Text::qs_at_text,
	"__inspect__"  => method Text::qs___inspect__,
	"@num"  => method Text::qs_at_num,
	"@list" => method Text::qs_at_list,
	"@bool" => method Text::qs_at_bool,
	"clone" => method Text::qs_clone,
	"()"    => function Text::qs_call,
	"="     => function Text::qs_assign,
	"<=>"   => method Text::qs_cmp,
	"=="    => method Text::qs_eql,
	"+"     => method Text::qs_add,
	"+="    => function Text::qs_add_assign,
	"len"   => method Text::qs_len,
	"get"   => (impls::index),
	"shift" => (impls::shift),
	"push" => (impls::push),
	"clear" => (impls::clear),
	"[]" => (impls::index),
	"[]=" => (impls::index_assign),
	"index_of" => (impls::index_of),
	"split" => (impls::split),
	"reverse" => (impls::reverse),
}

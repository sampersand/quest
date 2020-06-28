use crate::{Object, Result, Args};
use crate::error::{ValueError, KeyError};
use crate::literals::__THIS__;
use crate::types::Number;
use crate::Binding;
use std::borrow::Cow;
use std::fmt::{self, Debug, Display, Formatter};

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
	pub fn new(txt: String) -> Self {
		Text(Cow::Owned(txt))
	}

	pub const fn new_static(txt: &'static str) -> Self {
		Text(Cow::Borrowed(txt))
	}

	pub fn evaluate(&self) -> Result<Object> {
		match self.as_ref() {
			"__this__" => Ok(Binding::instance().as_ref().clone()),
			"__args__" => Binding::instance().get_attr("__args__"),
			"__stack__" => Ok(Binding::with_stack(|s| {
				let mut stack = s.read().expect("couldn't read stack")
					.iter()
					.map(|x| x.as_ref().clone())
					.collect::<Vec<_>>();
				stack.reverse();
				stack.into()
			})),
			_ => Binding::instance().as_ref().call_attr_old(".", &[self.clone().into()])
		}
	}
}

impl From<&'static str> for Text {
	fn from(txt: &'static str) -> Self {
		Text::new_static(txt)
	}
}

impl From<Text> for String {
	fn from(txt: Text) -> Self {
		txt.0.to_owned().to_string()
	}
}

impl From<String> for Text {
	fn from(txt: String) -> Self {
		Text::new(txt)
	}
}

impl From<String> for Object {
	fn from(txt: String) -> Self {
		Text::from(txt).into()
	}
}

impl From<&'static str> for Object {
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
	fn as_ref(&self) -> &str {
		self.0.as_ref()
	}
}

impl Text {
	#[inline]
	pub fn qs_at_text(&self, args: Args) -> Result<Object> {
		self.qs_clone(args)
	}


	pub fn qs_at_list(&self, _: Args) -> Result<Object> {
		Ok(self.as_ref()
			.chars()
			.map(|chr| chr.to_string().into())
			.collect::<Vec<Object>>()
			.into())
	}

	#[inline]
	pub fn qs_at_bool(&self, _: Args) -> Result<Object> {
		Ok((!self.as_ref().is_empty()).into())
	}

	pub fn qs_at_num(&self, args: Args) -> Result<Object> {
		use std::convert::TryFrom;

		if let Ok(radix) = args.arg(0) {
			let radix = radix.downcast_call::<Number>()?;
			let radix = i64::try_from(radix) // convert from number to i64
				.map_err(|err| err.to_string())
				.and_then(|num| u32::try_from(num).map_err(|err| err.to_string()))
				.map_err(|err| ValueError::Messaged(format!("invalid radix: {}", err)))?;

			Number::from_str_radix(self.as_ref(), radix)
				.map(Into::into)
				.map_err(|err| ValueError::Messaged(format!("cant convert: {}", err)).into())
		} else {
			Number::try_from(self.as_ref())
				.map(Into::into)
				.map_err(|err| ValueError::Messaged(err.to_string()).into())
		}
	}

	#[inline]
	pub fn qs_clone(&self, _: Args) -> Result<Object> {
		Ok(self.clone().into())
	}

	pub fn qs_call(this: &Object, _: Args) -> Result<Object> {
		if let Ok(this) = this.try_downcast_ref::<Text>() {
			this.evaluate()
		} else {
			Binding::instance().as_ref().dot_get_attr(this)
		}
	}

	pub fn qs_assign(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.clone();

		if let Some(this) = this.downcast_ref::<Text>() {
			use crate::obj::EqKey;
			if this.as_ref().eq_key(&__THIS__)? {
				return Ok(Binding::set_binding(rhs).as_ref().clone())
			}
		}

		Binding::instance().set_attr(this.clone(), rhs.clone()).and(Ok(rhs))
	}

	pub fn qs_eql(&self, args: Args) -> Result<Object> { // "=="
		if let Some(rhs) = args.arg(0)?.downcast_ref::<Text>() {
			Ok((*self == *rhs).into())
		} else {
			Ok(false.into())
		}
	}

	#[inline]
	pub fn qs_cmp(&self, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.downcast_call::<Text>()?;
		Ok(self.cmp(&rhs).into())
	}

	pub fn qs_add(&self, args: Args) -> Result<Object> {
		let mut clone = self.clone();
		clone.qs_add_assign(args)?;
		Ok(clone.into())
	}

	#[inline]
	pub fn qs_add_assign(&mut self, args: Args) -> Result<()> {
		let rhs = args.arg(0)?.downcast_call::<Text>()?;
		self.0.to_mut().push_str(rhs.as_ref());
		// let mut this = args.this()?.try_downcast_mut::<Text>()?;
		// this.0.to_mut().push_str(rhs.as_ref());
		// }

		Ok(())
	}
}

mod impls {
	use super::*;
	use crate::{Object, Result, ArgsOld, types};
	// "[]~" => (|args| todo!("[]~")),
	// "clear" => (|args| todo!("clear")),

	pub fn len(args: ArgsOld) -> Result<Object> {
		let this = args.this()?.try_downcast_ref::<Text>()?;
		Ok(this.0.len().into())
	}

	fn correct_index(idx: isize, len: usize) -> Result<Option<usize>> {
		if idx.is_positive() {
			let idx = (idx - 1) as usize;
			if idx < len {
				Ok(Some(idx))
			} else {
				Ok(None)
			}
		} else if idx.is_negative() {
			let idx = (-idx) as usize;
			if idx < len {
				Ok(Some(len - idx))
			} else {
				Ok(None)
			}
		} else {
			Err(KeyError::CantIndexByZero.into())
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
			if let Some(start) = correct_index(start, len)? {
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
				let end = correct_index(end, len)?.map(|x| x + 1).unwrap_or(len);
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
	"@text" => method Text::qs_at_text,
	"@num"  => method Text::qs_at_num,
	"@list" => method Text::qs_at_list,
	"@bool" => method Text::qs_at_bool,
	"clone" => method Text::qs_clone,
	"()"    => method Text::qs_call,
	"="     => method Text::qs_assign,
	"<=>"   => method Text::qs_cmp,
	"=="    => method Text::qs_eql,
	"+"     => method Text::qs_add,
	"+"     => method_assign Text::qs_add_assign,
	"len"   => (impls::len),
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

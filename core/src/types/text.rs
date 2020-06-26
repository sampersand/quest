use crate::{Object, error::{ValueError, KeyError}, types::rustfn::Binding};
use std::borrow::Cow;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone, PartialEq, Eq)]
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

impl AsRef<str> for Text {
	fn as_ref(&self) -> &str {
		self.0.as_ref()
	}
}

mod impls {
	use super::*;
	use crate::{Object, Result, Args, types};
	// "[]~" => (|args| todo!("[]~")),
	// "clear" => (|args| todo!("clear")),
	pub fn at_text(args: Args) -> Result<Object> { // "@text"
		let this = args.this()?;
		this.call_attr("clone", args.clone())
	}

	pub fn at_num(args: Args) -> Result<Object> { // "@num"
		use std::convert::TryFrom;

		let this = args.this()?.try_downcast_ref::<Text>()?;
		if let Ok(radix) = args.arg(0) {
			let radix = radix.downcast_call::<types::Number>()?;
			let radix = i64::try_from(radix) // convert from number to i64
				.map_err(|err| err.to_string())
				.and_then(|num| u32::try_from(num).map_err(|err| err.to_string()))
				.map_err(|err| ValueError::Messaged(format!("invalid radix: {}", err)))?;

			types::Number::from_str_radix(this.as_ref(), radix)
				.map(Into::into)
				.map_err(|err| ValueError::Messaged(format!("cant convert: {}", err)).into())
		} else {
			types::Number::try_from(this.as_ref())
				.map(Into::into)
				.map_err(|err| ValueError::Messaged(err.to_string()).into())
		}
	}

	pub fn call(args: Args) -> Result<Object> { // "()"
		let this = args.this()?;
		if let Ok(this) = this.try_downcast_ref::<Text>() {
			match this.as_ref() {
				"__this__" => return Ok(Binding::instance().as_ref().clone()),
				"__args__" => return Binding::instance().get_attr("__args__"),
				"__stack__" => return Ok(Binding::with_stack(|s| {
					let mut stack = s.read().expect("couldn't read stack")
						.iter()
						.map(|x| x.as_ref().clone())
						.collect::<Vec<_>>();
					stack.reverse();
					stack.into()
				})),
				_ => {}
			}
		}
		
		Binding::instance().as_ref().call_attr(".", vec![this.clone()])
	}

	pub fn assign(args: Args) -> Result<Object> { // "=" 
		let this = args.this()?;
		let rhs = args.arg(0)?;
		if this.downcast_ref::<Text>().map(|x| x.as_ref() == "__this__").unwrap_or(false) {
			Ok(Binding::set_binding(rhs.clone()).as_ref().clone())
		} else {
			args.binding().unwrap().set_attr_possibly_parents(this.clone(), rhs.clone())
		}
	}

	pub fn at_list(args: Args) -> Result<Object> {
		let this = args.this()?.try_downcast_ref::<Text>()?;
		Ok(this.as_ref()
			.chars()
			.map(|chr| chr.to_string().into())
			.collect::<Vec<Object>>()
			.into())
	}

	pub fn at_bool(args: Args) -> Result<Object> { // "@bool"
		let this = args.this()?.try_downcast_ref::<Text>()?;
		Ok((!this.as_ref().is_empty()).into())
	}

	pub fn clone(args: Args) -> Result<Object> { // "clone"
		let this = args.this()?.try_downcast_ref::<Text>()?;
		Ok(this.clone().into())
	}

	pub fn eql(args: Args) -> Result<Object> { // "=="
		let this = args.this()?.try_downcast_ref::<Text>()?;
		let rhs_opt = args.arg(0)?.downcast_ref::<Text>();
		match rhs_opt {
			Some(rhs_txt) => Ok((*this == *rhs_txt).into()),
			None => Ok(types::Boolean::FALSE.into())
		}
	}

	pub fn cmp(args: Args) -> Result<Object> {
		let this = args.this()?.try_downcast_ref::<Text>()?;
		let rhs = args.arg(0)?.downcast_call::<Text>()?;
		Ok(this.as_ref().cmp(rhs.as_ref()).into())
	}

	pub fn plus(args: Args) -> Result<Object> { // "+"
		let this = args.this()?;
		this.call_attr("clone", vec![])?
			.call_attr("+=", args.args(..)?)
	}

	pub fn plus_assign(args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.downcast_call::<Text>()?;
		{
			let mut this = args.this()?.try_downcast_mut::<Text>()?;
			this.0.to_mut().push_str(rhs.as_ref());

		}

		args.this().map(Clone::clone)
	}

	pub fn len(args: Args) -> Result<Object> {
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

	pub fn index(args: Args) -> Result<Object> {
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

	pub fn shift(args: Args) -> Result<Object> {
		let this = &mut args.this()?.try_downcast_mut::<types::Text>()?.0;
		if this.len() == 0 {
			Ok(Object::default())
		} else {
			Ok(this.to_mut().remove(0).to_string().into())
		}
	}

	pub fn push(args: Args) -> Result<Object> {
		let this_obj = args.this()?;
		let arg = args.arg(0)?.downcast_call::<types::Text>()?.0;
		this_obj.try_downcast_mut::<types::Text>()?.0.to_mut().push_str(arg.as_ref());
		Ok(this_obj.clone())
	}

	pub fn clear(args: Args) -> Result<Object> {
		let this_obj = args.this()?;
		this_obj.try_downcast_mut::<types::Text>()?.0.to_mut().clear();
		Ok(this_obj.clone())
	}

	pub fn index_assign(_args: Args) -> Result<Object> { todo!("[]=") } // "[]=
	pub fn index_of(_args: Args) -> Result<Object> { todo!("index_of") } // "index_of"
	pub fn split(_args: Args) -> Result<Object> { todo!("split") } // "split"
	pub fn reverse(_args: Args) -> Result<Object> { todo!("reverse") } // "reverse"
}

impl_object_type!{
for Text [(init_parent super::Basic super::Comparable) (parents super::Basic) (convert "@text")]:
	"@text" => (impls::at_text),
	"@num" => (impls::at_num),
	"@list" => (impls::at_list),
	"@bool" => (impls::at_bool),
	"clone" => (impls::clone),
	"()" => (impls::call),
	"="  => (impls::assign),
	"<=>"  => (impls::cmp),
	"==" => (impls::eql),
	"+"  => (impls::plus),
	"+="  => (impls::plus_assign),
	"len" => (impls::len),
	"get" => (impls::index),
	"shift" => (impls::shift),
	"push" => (impls::push),
	"clear" => (impls::clear),
	"[]" => (impls::index),
	"[]=" => (impls::index_assign),
	"index_of" => (impls::index_of),
	"split" => (impls::split),
	"reverse" => (impls::reverse),
}

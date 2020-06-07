use crate::obj::{Mapping, Object, types::{self, rustfn::Binding}};
use std::sync::{Arc, RwLock};
use std::borrow::Cow;
use std::fmt::{self, Debug, Formatter};

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
	use crate::obj::{Object, Result, Args, types};
	// "[]~" => (|args| todo!("[]~")),
	// "clear" => (|args| todo!("clear")),
	pub fn at_text(args: Args) -> Result<Object> { // "@text"
		let this = args.this()?;
		this.call_attr("clone", args.clone())
	}

	pub fn at_num(args: Args) -> Result<Object> { // "@num"
		use std::convert::TryFrom;

		let this = args.this()?.try_downcast_ref::<Text>()?;
		let radix = 
			args.arg(0)
				.ok()
				.map(|obj| obj.downcast_call::<types::Number>()
						.and_then(|n| n.try_to_int())
						.and_then(|r| u32::try_from(r)
								.map_err(|err| format!("invalid radix {}: {}", r, err).into())));
		match radix {
			Some(Ok(radix)) => 
				types::Number::from_str_radix(this.as_ref(), radix)
					.map(Into::into)
					.map_err(|err| err.to_string().into()),
			Some(Err(err)) => Err(err),
			None => 
				types::Number::from_str(this.as_ref())
					.map(Into::into)
					.map_err(|err| err.to_string().into())
		}
	}

	pub fn call(args: Args) -> Result<Object> { // "()"
		let this = args.this()?;
		if let Ok(this) = this.try_downcast_ref::<Text>() {
			match this.as_ref() {
				"__this__" => return Ok(Binding::instance().as_ref().clone()),
				"__args__" => return Binding::instance().get_attr("__args__"),
				// num if num.chars().next() == Some('_') && num.chars().skip(1).all(char::is_numeric) => {
				// 	use std::str::FromStr;
				// 	return Binding::instance().get_attr("__args__")?
				// 		.call_attr("[]", vec![
				// 			types::Number::from_str(&num.chars().skip(1).collect::<String>())
				// 				.expect("bad string?")	
				// 				.into()
				// 		])
				// },
				_ => {}
			}
		}
		
		Binding::instance().as_ref().call_attr(".", vec![this.clone().into()])
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
		Ok(this.as_ref().is_empty().into())
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

	pub fn plus(args: Args) -> Result<Object> { // "+" 
		let mut this_str = args.this()?.try_downcast_ref::<Text>()?.0.to_owned().to_string();
		let rhs = args.arg(0)?.downcast_call::<Text>()?;
		this_str.push_str(rhs.as_ref());
		Ok(this_str.into())
	}

	pub fn chr(args: Args) -> Result<Object> { todo!("chr") } // "chr"
	pub fn len(args: Args) -> Result<Object> { todo!("len") } // "len"
	pub fn index(args: Args) -> Result<Object> { todo!("[]") } // "[]"
	pub fn index_assign(args: Args) -> Result<Object> { todo!("[]=") } // "[]=
	pub fn is_empty(args: Args) -> Result<Object> { todo!("is_empty") } // "is_empty"
	pub fn index_of(args: Args) -> Result<Object> { todo!("index_of") } // "index_of"
	pub fn split(args: Args) -> Result<Object> { todo!("split") } // "split"
	pub fn reverse(args: Args) -> Result<Object> { todo!("reverse") } // "reverse"
}

impl_object_type!{
for Text [(parent super::Basic) (convert "@text")]:
	"@text" => (impls::at_text),
	"@num" => (impls::at_num),
	"()" => (impls::call),
	"="  => (impls::assign),
	"@list" => (impls::at_list),
	"@bool" => (impls::at_bool),
	"clone" => (impls::clone),
	"=="  => (impls::eql),
	"+"  => (impls::plus),
	"chr" => (impls::chr),
	"len" => (impls::len),
	"[]" => (impls::index),
	"[]=" => (impls::index_assign),
	"is_empty" => (impls::is_empty),
	"index_of" => (impls::index_of),
	"split" => (impls::split),
	"reverse" => (impls::reverse)
}

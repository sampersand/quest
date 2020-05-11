use crate::obj::{Mapping, Object, types::ObjectType};
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

	pub fn new_static(txt: &'static str) -> Self {
		Text(Cow::Borrowed(txt))
	}
}
impl From<&'static str> for Text {
	fn from(txt: &'static str) -> Self {
		Text::new_static(txt)
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

impl_object_type!{for Text, super::Basic;
	"@text" => (|args| {
		args.this_obj::<Text>()?.call("clone", &[])
	}),

	"@num" => (|args| {
		let this = args.this::<Text>()?;
		if let Ok(radix_obj) = args.get(1) {
			use std::convert::TryFrom;
			let r = radix_obj.call("@num", &[])?.try_downcast_ref::<Number>()?.try_to_int()?;
			match u32::try_from(r) {
				Ok(radix) => Number::from_str_radix(this.as_ref(), radix)
					.map(Into::into)
					.map_err(|err| err.to_string().into()),
				Err(err) => Err(format!("invalid radix {}: {}", r, err).into())
			}
		} else {
			Number::from_str(this.as_ref())
				.map(Into::into)
				.map_err(|err| err.to_string().into())
		}
	}),

	"@list" => (|args| todo!("@list")),

	"@bool" => (|args| {
		Ok(args.this::<Text>()?.as_ref().is_empty().into())
	}),

	"clone" => (|args| {
		Ok(args.this::<Text>()?.clone().into())
	}),

	"==" => (|args| {
		let this = args.this::<Text>()?;
		if let Ok(txt) = args.get_downcast::<Text>(1) {
			Ok((this.0 == txt.0).into())
		} else {
			Ok(false.into())
		}
	}),

	"chr" => (|args| todo!("chr")),
	"len" => (|args| todo!("len")),
	"[]" => (|args| todo!("[]")),
	"[]=" => (|args| todo!("[]=")),
	// "[]~" => (|args| todo!("[]~")),
	// "clear" => (|args| todo!("clear")),
	"is_empty" => (|args| todo!("is_empty")),
	"index" => (|args| todo!("index")),
	"split" => (|args| todo!("split")),
	"reverse" => (|args| todo!("reverse")),
}





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

impl_object_type!{for Text, super::Basic;
	"@text" => (|args| {
		args._this_obj::<Text>()?.call("clone", args.new_args_slice(&[]))
	}),

	"@num" => (|args| {
		let this = args._this_downcast::<Text>()?;
		if let Ok(radix_obj) = args.get(1) {
			use std::convert::TryFrom;
			let r = radix_obj.call("@num", args.new_args_slice(&[]))?
				.try_downcast_ref::<Number>()?
				.try_to_int()?;
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

	"()" => (|args| {
		match args._this_downcast::<Text>()?.as_ref() {
			"__this__" => Ok(args.binding().clone()),
			_ => args.binding()
				.get_attr(&args._this_obj::<Text>()?, args.binding())
		}
	}),

	"=" => (|args| {
		args.binding().set_attr(args._this_obj::<Text>()?, getarg!(Object; args), args.binding())
	}),

	"@list" => (|args| todo!("@list")),

	"@bool" => (|args| {
		Ok(args._this_downcast::<Text>()?.as_ref().is_empty().into())
	}),

	"clone" => (|args| {
		Ok(args._this_downcast::<Text>()?.clone().into())
	}),

	"==" => (|args| {
		let this = args._this_downcast::<Text>()?;
		if let Ok(txt) = args.get_downcast::<Text>(1) {
			Ok((this.0 == txt.0).into())
		} else {
			Ok(false.into())
		}
	}),
	"+" => (|args| {
		let mut this = args._this_downcast::<Text>()?.clone().0.into_owned();
		let rhs = args.get(1)?;
		this.push_str(
			rhs.call("@text", args.new_args_slice(&[]))?
			.try_downcast_ref::<Text>()?
			.as_ref()
		);
		Ok(this.into())
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





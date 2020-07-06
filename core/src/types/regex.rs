use crate::{Object, Args};
use std::fmt::{self, Debug, Display, Formatter};
use std::convert::TryFrom;
use crate::types::Text;

#[derive(Clone)]
pub struct Regex(regex::Regex);

impl Display for Regex {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "/{}/", self.as_str())
	}
}

impl Debug for Regex {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_tuple("Regex").field(&self.as_str()).finish()
	}
}

impl Eq for Regex {}
impl PartialEq for Regex {
	#[inline]
	fn eq(&self, rhs: &Regex) -> bool {
		self.as_str() == rhs.as_str()
	}
}

impl PartialEq<str> for Regex {
	#[inline]
	fn eq(&self, rhs: &str) -> bool {
		self.as_str() == rhs
	}
}

impl Regex {
	#[inline]
	pub fn new(re: &str) -> Result<Regex, regex::Error> {
		Self::try_from(re)
	}

	#[inline]
	pub fn as_str(&self) -> &str {
		self.as_ref()
	}
}

impl AsRef<str> for Regex {
	#[inline]
	fn as_ref(&self) -> &str {
		self.0.as_str()
	}
}

impl AsRef<regex::Regex> for Regex {
	#[inline]
	fn as_ref(&self) -> &regex::Regex {
		&self.0
	}
}

impl From<regex::Regex> for Regex {
	#[inline]
	fn from(re: regex::Regex) -> Self {
		Self(re)
	}
}

impl<'a> TryFrom<&'a str> for Regex {
	type Error = regex::Error;

	#[inline]
	fn try_from(re: &'a str) -> Result<Self, regex::Error> {
		regex::Regex::new(re).map(Self)
	}
}

impl From<Regex> for Text {
	#[inline]
	fn from(re: Regex) -> Self {
		Self::from(re.to_string())
	}
}

/// Quest functions
impl Regex {
	/// Inspects the [`Regex`].
	pub fn qs_call(_: &Object, args: Args) -> crate::Result<Object> {
		args.arg(0)?.try_downcast_and_then(|text: &Text| {
			Self::try_from(text.as_ref())
				.map(Object::from)
				.map_err(|err| crate::Error::Messaged(err.to_string()))
		})
	}

	/// Inspects the [`Regex`].
	#[inline]
	pub fn qs_inspect(this: &Object, args: Args) -> crate::Result<Object> {
		Self::qs_at_text(this, args)
	}

	/// Convert this into a [`Text`].
	#[inline]
	pub fn qs_at_text(this: &Object, _: Args) -> crate::Result<Object> {
		this.try_downcast_map(|this: &Self| Text::from(this.to_string()).into())
	}

	/// Compares two [`Regex`]s
	pub fn qs_eql(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?;
		this.try_downcast_and_then(|this: &Self| {
			rhs.try_downcast_map(|rhs: &Self| {
				(this == rhs).into()
			})
		})
	}

	/// Checks to see if the first argument matches.
	///
	/// The first argument is converted to a [`Text`] before matching
	pub fn qs_matches(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?;

		this.try_downcast_and_then(|this: &Self| {
			rhs.try_downcast_map(|rhs: &Text| {
				this.0.is_match(rhs.as_ref()).into()
			})
		})
	}
}

impl_object_type!{
for Regex [(parents super::Basic) (convert "@regex")]:
	"()"      => function Regex::qs_call,
	"@text"   => function Regex::qs_at_text,
	"inspect" => function Regex::qs_inspect,
	"=="      => function Regex::qs_eql,
	"matches" => function Regex::qs_matches,
}

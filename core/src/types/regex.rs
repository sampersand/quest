use crate::{Object, Args};
use std::fmt::{self, Debug, Display, Formatter};
use std::convert::TryFrom;
use crate::types::Text;

/// An error that is caused by a bad regex being parsed.
pub use ::regex::Error as RegexError;

// TODO: figure out how to get this from the library
/// Regex options. simply used for 
pub type RegexOptions = String;

#[derive(Clone)]
pub struct Regex(regex::Regex, RegexOptions);

impl Default for Regex {
	#[inline]
	fn default() -> Self {
		Self::new("").expect("default shouldn't fail")
	}
}

impl Display for Regex {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "/{}/{}", self.0.as_str(), self.1)
	}
}

impl Debug for Regex {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_tuple("Regex").field(&self.0.as_str()).field(&self.1).finish()
	}
}

impl Eq for Regex {}
impl PartialEq for Regex {
	#[inline]
	fn eq(&self, rhs: &Regex) -> bool {
		self.0.as_str() == rhs.0.as_str() && self.1 == rhs.1
	}
}

impl Regex {
	#[inline]
	pub fn new(re: &str) -> Result<Regex, RegexError> {
		Self::try_from(re)
	}

	#[inline]
	pub fn new_with_options(re: regex::Regex, opts: RegexOptions) -> Regex {
		Self(re, opts)
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
		Self::new_with_options(re, Default::default())
	}
}

impl<'a> TryFrom<&'a str> for Regex {
	type Error = RegexError;

	#[inline]
	fn try_from(re: &'a str) -> Result<Self, RegexError> {
		regex::Regex::new(re).map(Self::from)
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

	/// Returns an Array of matched values.
	///
	/// The first argument is converted to a [`Text`] before matching.
	pub fn qs_match(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?;

		this.try_downcast_and_then(|this: &Self| {
			rhs.call_downcast_map(|rhs: &Text| {
				this.0
					.captures(rhs.as_ref())
					.map(|x| x.iter().map(|m| {
							m.map(|m| Object::from(m.as_str().to_string()))
								.unwrap_or_default()
						}).collect::<Vec<_>>()
						.into()
					).unwrap_or_default()
			})
		})
	}

	/// Checks to see if the first argument matches.
	///
	/// The first argument is converted to a [`Text`] before matching.
	pub fn qs_does_match(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.arg(0)?;

		this.try_downcast_and_then(|this: &Self| {
			rhs.call_downcast_map(|rhs: &Text| {
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
	"does_match" => function Regex::qs_does_match,
	"match" => function Regex::qs_match,
}

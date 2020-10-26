use crate::{Object, Args};
use std::fmt::{self, Debug, Display, Formatter};
use std::convert::TryFrom;
use crate::types::Text;
use tracing::instrument;

bitflags::bitflags! {
	#[derive(Default)]
	pub struct Flags : u8 {
		const CASE_INSENSITIVE  = 1 << 1;
		const MULTI_LINE        = 1 << 2;
		const DOT_MATCH_NEWLINE = 1 << 3;
		const SWAP_GREEDY       = 1 << 4;
		const IGNORE_WHITESPACE = 1 << 5;
	}
}

impl Flags  {
	fn set_options(self, builder: &mut ::regex::RegexBuilder) {
		macro_rules! build_options {
			($($variant:ident $fn:ident)*) => {
				$(
					if self.contains(Flags::$variant) {
						builder.$fn(true);
					}
				)*
			};
		}

		build_options! {
			CASE_INSENSITIVE case_insensitive
			MULTI_LINE multi_line
			DOT_MATCH_NEWLINE dot_matches_new_line
			SWAP_GREEDY swap_greed
			IGNORE_WHITESPACE ignore_whitespace
		}
	}
}

impl Display for Flags {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		macro_rules! write_flag {
			($($variant:ident $lit:literal)*) => {
				$(
					if self.contains(Flags::$variant) {
						write!(f, $lit)?;
					}
				)*
			};
		}

		write_flag! {
			CASE_INSENSITIVE "i" MULTI_LINE "m" DOT_MATCH_NEWLINE "n"
			SWAP_GREEDY "U" IGNORE_WHITESPACE "x"
		}
		Ok(())
	}
}

/// An error that is caused by a bad regex being parsed.
pub use ::regex::Error as RegexError;

#[derive(Debug, Clone)]
pub struct Regex(regex::Regex, Flags);

impl Default for Regex {
	#[inline]
	fn default() -> Self {
		Self::new("").expect("default shouldn't fail")
	}
}

impl Display for Regex {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "/{}/{}", self.0.as_str(), self.1)
	}
}

impl Eq for Regex {}
impl PartialEq for Regex {
	#[inline]
	fn eq(&self, rhs: &Self) -> bool {
		self.0.as_str() == rhs.0.as_str() && self.1 == rhs.1
	}
}

impl Regex {
	#[inline]
	pub fn new(rxp: &str) -> Result<Self, RegexError> {
		Self::try_from(rxp)
	}

	pub fn new_with_options(rxp: &str, flags: Flags) -> Result<Self, RegexError> {
		let mut builder = ::regex::RegexBuilder::new(rxp);
		flags.set_options(&mut builder);
		Ok(Self(builder.build()?, flags))
	}
}

impl AsRef<regex::Regex> for Regex {
	#[inline]
	fn as_ref(&self) -> &regex::Regex {
		&self.0
	}
}

impl<'a> TryFrom<&'a str> for Regex {
	type Error = RegexError;

	#[inline]
	fn try_from(rxp: &'a str) -> Result<Self, RegexError> {
		Self::new_with_options(rxp, Flags::default())
	}
}

impl From<&Regex> for Text {
	#[inline]
	fn from(re: &Regex) -> Self {
		re.to_string().into()
	}
}

/// Quest functions
impl Regex {
	/// Inspects the [`Regex`].
	#[inline]
	#[instrument(name="Regex::inspect", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_inspect(this: &Object, args: Args) -> crate::Result<Object> {
		Self::qs_at_text(this, args)
	}

	/// Convert this into a [`Text`].
	#[inline]
	#[instrument(name="Regex::@text", level="trace", skip(this), fields(self=?this))]
	pub fn qs_at_text(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;

		Ok(Text::from(&*this).into())
	}

	/// Compares two [`Regex`]s
	#[instrument(name="Regex::==", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_eql(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?.try_downcast::<Self>();
		let this = this.try_downcast::<Self>()?;

		Ok(rhs.map(|rhs| *rhs == *this)
				.unwrap_or(false)
				.into())
	}

	/// Returns an Array of matched values.
	///
	/// The first argument is converted to a [`Text`] before matching.
	#[instrument(name="Regex::scan", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_scan(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?.call_downcast::<Text>()?;
		let this = this.try_downcast::<Self>()?;

		Ok(this.0
			.find_iter(rhs.as_ref())
			.map(|m| Object::from(m.as_str().to_string()))
			.collect::<crate::types::List>()
			.into())
	}

	/// Returns an Array of matched groups.
	///
	/// The first argument is converted to a [`Text`] before matching.
	#[instrument(name="Regex::match", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_match(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?.call_downcast::<Text>()?;
		let this = this.try_downcast::<Self>()?;

		Ok(this.0
			.captures(rhs.as_ref())
			.map(|x| x.iter().map(|m| {
					m.map(|m| Object::from(m.as_str().to_string())).unwrap_or_default()
				}).collect::<Vec<_>>()
				.into()
			).unwrap_or_default())
	}


	/// Checks to see if the first argument matches.
	///
	/// The first argument is converted to a [`Text`] before matching.
	#[instrument(name="Regex::does_match", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_does_match(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?.call_downcast::<Text>()?;
		let this = this.try_downcast::<Self>()?;

		Ok(this.0.is_match(rhs.as_ref()).into())
	}
}

impl_object_type!{
for Regex [(parents super::Basic) (convert "@regex")]:
	"inspect" => method Self::qs_inspect,
	"==" => method Self::qs_eql,
	"does_match" => method Self::qs_does_match,
	"match" => method Self::qs_match,
	"scan" => method Self::qs_scan,
}

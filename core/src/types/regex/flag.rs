use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Flag {
	CaseInsensitive  = 1 << 0,
	MultiLine        = 1 << 1,
	DotMatchNewLine  = 1 << 2,
	SwapGreedy       = 1 << 3,
	IgnoreWhitespace = 1 << 4,
}

impl Flag {
	pub(super) fn set_option(self, builder: &mut ::regex::RegexBuilder) {
		match self {
			Flag::CaseInsensitive  => builder.case_insensitive(true),
			Flag::MultiLine        => builder.multi_line(true),
			Flag::DotMatchNewLine  => builder.dot_matches_new_line(true),
			Flag::SwapGreedy       => builder.swap_greed(true),
			Flag::IgnoreWhitespace => builder.ignore_whitespace(true),
		};
	}
}

impl Display for Flag {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Flag::CaseInsensitive => Display::fmt(&'i', f),
			Flag::MultiLine => Display::fmt(&'m', f),
			Flag::DotMatchNewLine => Display::fmt(&'n', f),
			Flag::SwapGreedy => Display::fmt(&'U', f),
			Flag::IgnoreWhitespace => Display::fmt(&'x', f),
		}
	}
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(packed)]
pub struct Flags(u8);

impl Flags {
	#[inline]
	pub fn new() -> Self {
		Self::default()
	}

	#[inline]
	pub fn is(&self, flag: Flag) -> bool {
		(self.0 & flag as u8) != 0
	}

	#[inline]
	pub fn set(&mut self, flag: Flag) {
		self.0 |= flag as u8;
	}

	#[inline]
	pub fn unset(&mut self, flag: Flag) {
		self.0 &= !(flag as u8);
	}

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct FlagIter(Flags, Option<Flag>);

impl Iterator for FlagIter {
	type Item = Flag;
	fn next(&mut self) -> Option<Self::Item> {
		let flag = self.1?;
		self.1 =
			match self.1? {
				Flag::CaseInsensitive  => Some(Flag::MultiLine),
				Flag::MultiLine        => Some(Flag::DotMatchNewLine),
				Flag::DotMatchNewLine  => Some(Flag::SwapGreedy),
				Flag::SwapGreedy       => Some(Flag::IgnoreWhitespace),
				Flag::IgnoreWhitespace => None
			};

		if self.0.is(flag) {
			Some(flag)
		} else {
			self.next()
		}
	}
}

impl IntoIterator for Flags {
	type Item = Flag;
	type IntoIter = FlagIter;
	fn into_iter(self) -> Self::IntoIter { FlagIter(self, None) }
}

impl Debug for Flags {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let mut t = f.debug_tuple("Flags");
		for flag in *self {
			t.field(&flag);
		}

		t.finish()
	}
}

impl Display for Flags {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" | "), f)
	}
}


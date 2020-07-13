use std::fmt::{self, Display, Formatter};

bitflags::bitflags! {
	#[derive(Default)]
	pub struct Flags : u8 {
		const CASE_INSENSITIVE  = 1 << 0;
		const MULTILINE         = 1 << 1;
		const DOT_MATCH_NEWLINE = 1 << 2;
		const SWAP_GREEDY       = 1 << 3;
		const IGNORE_WHITESPACE = 1 << 4;
	}
}

impl Flags {
	#[inline]
	pub(super) fn set_option(self, builder: &mut ::regex::RegexBuilder) {
		if self.contains(Flags::CASE_INSENSITIVE) { 
			builder.case_insensitive(true);
		}

		if self.contains(Flags::MULTILINE) { 
			builder.multi_line(true);
		}

		if self.contains(Flags::DOT_MATCH_NEWLINE) { 
			builder.dot_matches_new_line(true);
		}

		if self.contains(Flags::SWAP_GREEDY) { 
			builder.swap_greed(true);
		}

		if self.contains(Flags::IGNORE_WHITESPACE) { 
			builder.ignore_whitespace(true);
		}
	}
}


impl Display for Flags {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if self.contains(Flags::CASE_INSENSITIVE) { 
			Display::fmt(&'i', f)?;
		}

		if self.contains(Flags::MULTILINE) { 
			Display::fmt(&'m', f)?;
		}

		if self.contains(Flags::DOT_MATCH_NEWLINE) { 
			Display::fmt(&'n', f)?;
		}

		if self.contains(Flags::SWAP_GREEDY) { 
			Display::fmt(&'U', f)?;
		}

		if self.contains(Flags::IGNORE_WHITESPACE) { 
			Display::fmt(&'x', f)?;
		}

		Ok(())
	}
}
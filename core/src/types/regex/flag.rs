use std::fmt::{self, Display, Formatter};

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
	pub(super) fn set_options(self, builder: &mut ::regex::RegexBuilder) {
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

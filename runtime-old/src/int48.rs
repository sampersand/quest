#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Int48(u64);

sa::assert_eq_align!(Int48, u64);
sa::assert_eq_size!(Int48, u64);

const NEGATIVE_BIT: u64 = 1 << 47;
const U64_MASK: u64 = 0x0000_ffff_ffff_ffff;

impl Int48 {
	pub const ZERO: Self = Self(0);

	/// Attempts to create a new [`Int48`], returning `None` if the passed value doesn't is too large.
	pub const fn new(val: i64) -> Option<Self> {
		if (val.abs() as u64) <= U64_MASK {
			Some(Self::new_unchecked(val))
		} else {
			None
		}
	}

	/// Converts `self` into an `i64`
	pub const fn into_i64(self) -> i64 {
		if self.0 & NEGATIVE_BIT == 0 {
			self.0 as i64
		} else {
			(self.0 | !U64_MASK) as i64
		}
	}

	/// Creates a new [`Int48`] without checking to see if `val` is valid.
	///
	/// While this function is not `unsafe`, it will discard any unused bits. Try [`new`] for a safer alternative.
	#[inline]
	pub const fn new_unchecked(val: i64) -> Self {
		Self((val as u64) & U64_MASK)
	}

	/// Converts `self` to its byte representation.
	pub const fn to_bytes(self) -> [u8; 6] {
		let [_, _, bytes @ ..] = self.0.to_be_bytes();
		bytes
	}

	/// Builds `self` from its byte representation.
	pub const fn from_bytes(bytes: [u8; 6]) -> Self {
		let [b1, b2, b3, b4, b5, b6] = bytes;

		Self(u64::from_be_bytes([0, 0, b1, b2, b3, b4, b5, b6]))
	}
}

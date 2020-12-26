use crate::value::{Value, ValueConvertable};
use std::fmt::{self, Display, Formatter};

/// Either true or false in Quest.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Boolean(pub bool);

impl Value {
	pub const FALSE: Self = unsafe { Self::from_inner_unchecked(2) };
	pub const TRUE: Self = unsafe { Self::from_inner_unchecked(4) };
}

impl Boolean {
	/// Create a new [`Boolean`].
	#[inline]
	pub const fn new(b: bool) -> Self {
		Self(b)
	}

	/// Converts `self` into a boolean.
	#[inline]
	pub const fn into_inner(self) -> bool {
		self.0
	}

	#[inline]
	fn convert(self) -> u64 {
		((self.0 as u64 + 1) << 1)
	}
}

#[test]
fn test_convert() {
	assert_eq!(Boolean::new(true).convert(), Value::TRUE.inner());
	assert_eq!(Boolean::new(false).convert(), Value::FALSE.inner());
}

// SAFETY: All pointers we allocate for [`Value`s] are 8-bit aligned, and so none of them will use the lowest block.Value
// We reserve `2` as false and `4` as true.
unsafe impl ValueConvertable for Boolean {
	#[inline]
	fn into_value(self) -> Value {
		// SAFETY: By definition, `convert` is correct.
		unsafe {
			Value::from_inner_unchecked(self.convert())
		}
	}

	#[inline]
	fn is_value(value: &Value) -> bool {
		value.inner() == Value::TRUE.0 || value.inner() == Value::FALSE.0
	}

	#[inline]
	unsafe fn from_value_unchecked(value: Value) -> Self {
		debug_assert!(Self::is_value(&value), "invalid value given: {:#?}", value);

		Self::new(value.inner() == Value::TRUE.0)
	}
}

impl AsRef<bool> for Boolean {
	#[inline]
	fn as_ref(&self) -> &bool {
		&self.0
	}
}

impl AsMut<bool> for Boolean {
	#[inline]
	fn as_mut(&mut self) -> &mut bool {
		&mut self.0
	}
}

impl Display for Boolean {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

// impl Boolean {
// 	pub fn qs_at_text(&self) -> Result<Self> {
		
// 	}
// }



// [(parents super::Basic) (no_convert)]:
// 	"@text"   => method Self::qs_at_text,
// 	"inspect" => method Self::qs_inspect,
// 	"@num"    => method Self::qs_at_num,
// 	"@bool"   => method Self::qs_at_bool,
// 	"=="      => method Self::qs_eql,
// 	"!"       => method Self::qs_not,
// 	"&"       => method Self::qs_bitand,
// 	"&="      => method Self::qs_bitand_assign,
// 	"|"       => method Self::qs_bitor,
// 	"|="      => method Self::qs_bitor_assign,
// 	"^"       => method Self::qs_bitxor,
// 	"^="      => method Self::qs_bitxor_assign,
// 	"<=>"     => method Self::qs_cmp,
// 	"hash"    => method Self::qs_hash,
// }




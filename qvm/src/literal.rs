use crate::{ValueType, Value, value::NamedType};
use std::fmt::{self, Display, Formatter};
use std::collections::hash_map::{HashMap, Entry};
use parking_lot::RwLock;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Once;

lazy_static::lazy_static! {
	static ref STR_TO_LITERAL: RwLock<HashMap<&'static str, Literal>> = RwLock::new(HashMap::new());
	static ref LITERAL_TO_STR: RwLock<HashMap<Literal, &'static str>> =
		RwLock::new(HashMap::with_capacity(Literal::ONE_PAST_MAX_BUILTIN as usize));
}

static NEXT_ID: AtomicU32 = AtomicU32::new(Literal::ONE_PAST_MAX_BUILTIN);

pub fn initialize() {
	static ONCE: Once = Once::new();

	ONCE.call_once(|| {
		let mut map = STR_TO_LITERAL.write();

		for (lit, repr) in BUILTIN_REPRS.iter().enumerate() {
			map.insert(repr, Literal(lit as u32 + 1));
		}
	})
}

/// Literals are used to represent identifiers within Quest.
// NOTE: Literals must start at one; a literal with id 0 is actually a different type's representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Literal(u32);

impl Literal {
	const fn is_builtin(self) -> bool {
		self.0 < Self::ONE_PAST_MAX_BUILTIN
	}

	/// Creates a new [`Literal`] with the given identifier.
	pub fn new(repr: &'static str) -> Self {
		match STR_TO_LITERAL.write().entry(repr) {
			Entry::Occupied(entry) => *entry.get(),
			Entry::Vacant(entry) => {
				let literal = Self(NEXT_ID.fetch_add(1, Ordering::Relaxed));
				entry.insert(literal);
				LITERAL_TO_STR.write().insert(literal, repr);
				literal
			}
		}
	}

	/// Creates a new `Literal` from its bits, without checking to make sure those bits are valid.
	///
	/// # Safety
	/// The caller must ensure that the bits are valid.
	pub unsafe fn from_bits_unchecked(bits: u32) -> Self {
		let literal = Self(bits);

		debug_assert!(literal.is_builtin() || LITERAL_TO_STR.read().contains_key(&literal),
			"invalid unchecked bits: {:?}", bits);

		literal
	}

	#[must_use="this will leak memory if not used."]
	pub fn intern(repr: impl AsRef<str>) -> Self {
		let repr = repr.as_ref();

		if let Some(literal) = STR_TO_LITERAL.read().get(repr) {
			return *literal;
		}

		Self::new(Box::leak(repr.to_string().into_boxed_str()))
	}

	/// Gets the representation of this literal.
	pub fn repr(self) -> &'static str {
		if self.is_builtin() {
			BUILTIN_REPRS[self.0 as usize]
		} else {
			LITERAL_TO_STR
				.read()
				.get(&self)
				.expect("somehow got an unmapped literal?")
		}
	}

	#[cfg(test)]
	pub(crate) fn bits(&self) -> u32 { self.0 }
}


impl Display for Literal {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.repr(), f)
	}
}

impl NamedType for Literal {
	#[inline(always)]
	fn typename() -> &'static str {
		"Literal"
	}
}

const LITERAL_TAG: u64   = 0b0010;
const LITERAL_MASK: u64  = 0b0111;
const LITERAL_SHIFT: u64 = 3;

unsafe impl ValueType for Literal {
	fn into_value(self) -> Value {
		debug_assert_ne!(self.0, 0);
		// SAFETY: we're defining what it means to be a literal here.
		unsafe {
			Value::from_bits_unchecked(((self.0 as u64) << LITERAL_SHIFT) | LITERAL_TAG)
		}
	}

	fn is_value_a(value: &Value) -> bool {
		// dbg!(value.bits());
		// dbg!((value.bits() & LITERAL_MASK) == LITERAL_TAG, value.bits() != LITERAL_TAG);
		(value.bits() & LITERAL_MASK) == LITERAL_TAG && value.bits() != LITERAL_TAG
	}

	unsafe fn value_into_unchecked(value: Value) -> Self {
		debug_assert!(Self::is_value_a(&value), "invalid value given to `value_into_unchecked`: {:?}", value);

		let literal_bits = value.bits() >> LITERAL_SHIFT;
		debug_assert_eq!(literal_bits as u32 as u64, literal_bits); // ie it's only the bottom 32 bits.

		Self(literal_bits as u32)
	}
}

macro_rules! declare_literals {
	($($name:ident($string:literal))+) => {
		static BUILTIN_REPRS: [&'static str; Literal::ONE_PAST_MAX_BUILTIN as usize] = ["", $($string),*];


		impl Literal {
			const __DONTUSE: Self = Self(0);

			declare_literals!(1, $($name)+);
		}
	};
	($num:expr, $name:ident $($rest:ident)*) => {
		pub const $name: Self = Self($num);
		declare_literals!($num+1, $($rest)*);
	};

	($num:expr,) => {
		const ONE_PAST_MAX_BUILTIN: u32 = $num;
	}
}

declare_literals! {
	AT_BOOL("@bool") AT_NUM("@num") AT_TEXT("@text")
	AT_LIST("@list") AT_MAP("@map")

	// Operators
	OP_POS("+@") OP_NEG("-@")
	OP_ADD("+") OP_ADD_EQ("+=") OP_SUB("-") OP_SUB_EQ("-=") OP_MUL("*")  OP_MUL_EQ("*=")
	OP_DIV("/") OP_DIV_EQ("/=") OP_MOD("%") OP_MOD_EQ("%=") OP_POW("**") OP_POW_EQ("**=")

	OP_BNOT("~")
	OP_BAND("&") OP_BAND_EQ("&=") OP_BOR("|")  OP_BOR_EQ("|=") OP_BXOR("^") OP_BXOR_EQ("^=")
	OP_SHL("<<") OP_SHL_EQ("<<=") OP_SHR(">>") OP_SHR_EQ(">>=")

	OP_LNOT("!")
	OP_EQL("==") OP_NEQ("!=") OP_LTH("<") OP_GTH(">") OP_LEQ("<=") OP_GEQ(">=") OP_CMP("<=>")

	OP_ASN("=") OP_CALL("()") OP_INDEX("[]") OP_INDEX_ASN("[]=")
	OP_SCOPED("::") OP_DOT(".") OP_DOT_ASN(".=") OP_DOT_QRY(".?")

	// Builtin Symbols
	BI_KEYS("__keys__") BI_ID("__id__") BI_PARENTS("__parents__")

	// Class Names
	// CL_OBJECT("Object") CL_
}


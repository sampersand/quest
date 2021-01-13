#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Literal(u32);

impl Literal {
	pub fn new(word: &'static str) -> Self {
		let _ = word;
		todo!();
	}
}

macro_rules! declare_literals {
	($($name:ident($string:literal))+) => {
		impl Literal {
			declare_literals!(0,$($name)+);

			pub fn repr(self) -> &'static str {
				const BUILTIN_REPRS: [&'static str; Literal::ONE_PAST_MAX_BUILTIN as usize] = [$($string),*];

				if self.0 < Self::ONE_PAST_MAX_BUILTIN {
					BUILTIN_REPRS[self.0 as usize]
				} else {
					todo!()
				}
			}
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

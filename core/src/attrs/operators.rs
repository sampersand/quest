use crate::{Literal, Object, Args, Result};

macro_rules! define_operator_traits {
	($($trait:ident $method:literal $fn:ident)*) => {
		$(
			#[doc="The `"]
			#[doc=$method]
			#[doc="` operator."]
			pub trait $trait {
				/// The method name within quest
				const METHOD: Literal = $method;

				#[doc="Performs the `"]
				#[doc=$method]
				#[doc="` operation."]
				fn $fn(this: &Object, args: Args) -> Result<Object>;
			}
		)*
	};
}

define_operator_traits! {
	Scoped "::" qs_scoped RootScope "::@" qs_root_scope
	Dot "." qs_dot DotAssign ".=" qs_dot_assign
	Call "()" qs_call

	Pos "+@" qs_pos 
	Neg "-@" qs_neg
	Add "+"  qs_add AddAssign "+="  qs_add_assign
	Sub "-"  qs_sub SubAssign "-="  qs_sub_assign
	Mul "*"  qs_mul MulAssign "*="  qs_mul_assign
	Div "/"  qs_div DivAssign "/="  qs_div_assign
	Mod "%"  qs_mod ModAssign "/="  qs_mod_assign
	Pow "**" qs_pow PowAssign "**=" qs_pow_assign

	Not "!"   qs_not
	Eql "=="  qs_eql Neq "!=" qs_neq
	Lth "<"   qs_lth Leq "<=" qs_leq
	Gth ">"   qs_gth Geq ">=" qs_geq
	Cmp "<=>" qs_cmp
	And "&&"  qs_and Or "||" qs_or

	BitNot "~"  qs_bitnot
	BitAnd "&"  qs_bitand BitAndAssign "&="  qs_bitand_assign
	BitOr  "|"  qs_bitor  BitOrAssign  "|="  qs_bitor_assign
	BitXor "^"  qs_bitxor BitXorAssign "^="  qs_bitxor_assign
	Shl    "<<" qs_shl    ShlAssign    "<<=" qs_shl_assign
	Shr    ">>" qs_shr    ShrAssign    ">>=" qs_shr_assign
}

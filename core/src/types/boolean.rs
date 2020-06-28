use crate::{Object, Result, Args, types};
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Boolean(bool);

impl Boolean {
	#[inline]
	pub const fn new(t: bool) -> Self {
		Boolean(t)
	}

	pub const FALSE: Boolean = Boolean::new(false);
	pub const TRUE: Boolean = Boolean::new(true);

	pub fn into_inner(self) -> bool {
		self.0
	}
}

impl PartialEq<bool> for Boolean {
	fn eq(&self, rhs: &bool) -> bool {
		self.0 == *rhs
	}
}

impl Debug for Boolean {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			write!(f, "Boolean({:?})", self.0)
		} else {
			Display::fmt(self, f)
		}
	}
}

impl Display for Boolean {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl From<bool> for Object {
	fn from(inp: bool) -> Self {
		Boolean::new(inp).into()
	}
}

impl From<bool> for Boolean {
	fn from(b: bool) -> Self {
		Boolean::new(b)
	}
}

impl From<Boolean> for bool {
	fn from(b: Boolean) -> Self {
		b.0
	}
}

impl AsRef<bool> for Boolean {
	fn as_ref(&self) -> &bool {
		&self.0
	}
}

impl From<Boolean> for types::Number {
	fn from(b: Boolean) -> Self {
		const TRUE_NUMBER: types::Number = types::Number::ONE;
		const FALSE_NUMBER: types::Number = types::Number::ZERO;
		if b.0 { TRUE_NUMBER } else { FALSE_NUMBER }
	}
}

impl From<Boolean> for types::Text {
	fn from(b: Boolean) -> Self {
		const TRUE_TEXT: types::Text = types::Text::new_static("true");
		const FALSE_TEXT: types::Text = types::Text::new_static("false");
		if b.0 { TRUE_TEXT } else { FALSE_TEXT }
	}
}

macro_rules! impl_bitwise_ops {
	($($trait:ident $trait_assign:ident $fn:ident $fn_assign:ident)*) => {
		$(
			impl std::ops::$trait for Boolean {
				type Output = Self;
				#[inline]
				fn $fn(mut self, rhs: Self) -> Self {
					use std::ops::$trait_assign;
					self.$fn_assign(rhs);
					self
				}
			}

			impl std::ops::$trait_assign for Boolean {
				#[inline]
				fn $fn_assign(&mut self, rhs: Self)  {
					(self.0).$fn_assign(rhs.0);
				}
			}
		)*
	};
}

impl_bitwise_ops! {
	BitAnd BitAndAssign bitand bitand_assign
	BitOr BitOrAssign bitor bitor_assign
	BitXor BitXorAssign bitxor bitxor_assign
}

impl std::ops::Not for Boolean {
	type Output = Self;

	fn not(self) -> Self {
		Self(!self.0)
	}
}


impl Boolean {
	#[inline]
	pub fn qs_at_num(&self, _: Args) -> Result<Object> {
		Ok(types::Number::from(*self).into())
	}

	#[inline]
	pub fn qs_at_text(&self, _: Args) -> Result<Object> {
		Ok(types::Text::from(*self).into())
	}

	#[inline]
	pub fn qs_at_bool(&self, args: Args) -> Result<Object> {
		self.qs_clone(args)
	}

	#[inline]
	pub fn qs_clone(&self, _: Args) -> Result<Object> {
		Ok((*self).into())
	}

	pub fn qs_eql(&self, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.try_downcast_ref::<Boolean>();

		Ok(rhs.map(|rhs| *self == *rhs).unwrap_or(false).into())
	}

	#[inline]
	pub fn qs_not(&self, _: Args) -> Result<Object> {
		Ok((!*self).into())
	}

	pub fn qs_cmp(&self, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.downcast_call::<Boolean>()?;

		Ok(self.cmp(&rhs).into())
	}

	#[inline]
	pub fn qs_hash(&self, _args: Args) -> Result<Object> {
		todo!("hash for Boolean")
	}
}

macro_rules! define_bitwise_fns {
	($($qs_method:ident $method:ident $qs_method_assign:ident $method_assign:ident)*) => {
		impl Boolean {
			$(
				pub fn $qs_method(&self, args: Args) -> Result<Object> {
					let rhs = args.arg(0)?.downcast_call::<Boolean>()?;

					#[allow(unused)]
					use std::ops::*;

					Ok(self.$method(rhs).into())
				}

				pub fn $qs_method_assign(&mut self, args: Args) -> Result<()> {
					let rhs = args.arg(0)?.downcast_call::<Boolean>()?;

					#[allow(unused)]
					use std::ops::*;
					self.$method_assign(rhs);
					Ok(())
				}
			)*
		}
	};
}
define_bitwise_fns!{
	qs_bitand bitand qs_bitand_assign bitand_assign
	qs_bitor bitor qs_bitor_assign bitor_assign
	qs_bitxor bitxor qs_bitxor_assign bitxor_assign
}

impl_object_type!{
for Boolean [(parents super::Basic) (convert "@bool")]:
	"@num"  => method Boolean::qs_at_num,
	"@text" => method Boolean::qs_at_text,
	"@bool" => method Boolean::qs_at_bool,
	"clone" => method Boolean::qs_clone,
	"=="    => method Boolean::qs_eql,
	"!"     => method Boolean::qs_not,
	"&"     => method Boolean::qs_bitand,
	"clone1" => method Boolean::qs_clone,
	"&="    => method_assign Boolean::qs_bitand_assign,
	"clone2" => method Boolean::qs_clone,
	"|"     => method Boolean::qs_bitor,
	"|="    => method_assign Boolean::qs_bitor_assign,
	"^"     => method Boolean::qs_bitxor,
	"^="    => method_assign Boolean::qs_bitxor_assign,
	"<=>"   => method Boolean::qs_cmp,
	"hash"  => method Boolean::qs_hash,
}

#[cfg(test)]
mod tests {
	use super::*;

	macro_rules! assert_call_eq {
		($obj:ident $fn:ident $({$($args:expr),*})? $into:ty, $expected:expr) => {
			assert_eq!(
				*Boolean::$obj.$fn(args!($($($args),*)?))
					.unwrap().downcast_ref::<$into>().unwrap(), $expected);
		};
	}

	#[test]
	fn at_num() {
		assert_call_eq!(TRUE qs_at_num types::Number, types::Number::ONE);
		assert_call_eq!(FALSE qs_at_num types::Number, types::Number::ZERO);
	}

	#[test]
	fn at_text() {
		assert_call_eq!(TRUE qs_at_text types::Text, types::Text::from("true"));
		assert_call_eq!(FALSE qs_at_text types::Text, types::Text::from("false"));
	}

	#[test]
	fn at_bool() {
		assert_call_eq!(TRUE qs_at_bool types::Boolean, types::Boolean::TRUE);
		assert_call_eq!(FALSE qs_at_bool types::Boolean, types::Boolean::FALSE);
	}

	#[test]
	fn clone() {
		assert_call_eq!(TRUE qs_clone types::Boolean, types::Boolean::TRUE);
		assert_call_eq!(FALSE qs_clone types::Boolean, types::Boolean::FALSE);
	}

	#[test]
	fn eql() {
		assert_call_eq!(TRUE qs_eql {true} types::Boolean, types::Boolean::TRUE);
		assert_call_eq!(TRUE qs_eql {false} types::Boolean, types::Boolean::FALSE);
		assert_call_eq!(FALSE qs_eql {true} types::Boolean, types::Boolean::FALSE);
		assert_call_eq!(FALSE qs_eql {false} types::Boolean, types::Boolean::TRUE);
	}

	#[test]
	fn not() {
		assert_call_eq!(TRUE qs_not types::Boolean, types::Boolean::FALSE);
		assert_call_eq!(FALSE qs_not types::Boolean, types::Boolean::TRUE);
	}

	#[test]
	fn bitand() {
		assert_call_eq!(TRUE qs_bitand {true} types::Boolean, types::Boolean::TRUE);
		assert_call_eq!(TRUE qs_bitand {false} types::Boolean, types::Boolean::FALSE);
		assert_call_eq!(FALSE qs_bitand {true} types::Boolean, types::Boolean::FALSE);
		assert_call_eq!(FALSE qs_bitand {false} types::Boolean, types::Boolean::FALSE);
	}

	#[test]
	#[ignore]
	fn bitand_assign() { todo!() }

	#[test]
	fn bitor() {
		assert_call_eq!(TRUE qs_bitor {true} types::Boolean, types::Boolean::TRUE);
		assert_call_eq!(TRUE qs_bitor {false} types::Boolean, types::Boolean::TRUE);
		assert_call_eq!(FALSE qs_bitor {true} types::Boolean, types::Boolean::TRUE);
		assert_call_eq!(FALSE qs_bitor {false} types::Boolean, types::Boolean::FALSE);
	}

	#[test]
	#[ignore]
	fn bitor_assign() { todo!() }

	#[test]
	fn bitxor() {
		assert_call_eq!(TRUE qs_bitxor {true} types::Boolean, types::Boolean::FALSE);
		assert_call_eq!(TRUE qs_bitxor {false} types::Boolean, types::Boolean::TRUE);
		assert_call_eq!(FALSE qs_bitxor {true} types::Boolean, types::Boolean::TRUE);
		assert_call_eq!(FALSE qs_bitxor {false} types::Boolean, types::Boolean::FALSE);
	}

	#[test]
	#[ignore]
	fn bitxor_assign() { todo!() }


	#[test]
	#[ignore]
	fn cmp() { todo!(); }

	#[test]
	#[ignore]
	fn hash() { todo!(); }
}
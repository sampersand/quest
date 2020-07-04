use crate::{Object, Args};

use crate::literals::{EQL, AT_BOOL, NOT,  __INSPECT__};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Basic;

impl Basic {
	#[inline]
	pub fn qs_at_bool(_: &Object, _: Args) -> crate::Result<bool> {
		Ok(true.into())
	}

	#[inline]
	pub fn qs_at_text(this: &Object, args: Args) -> crate::Result<Object> {
		this.call_attr_lit(__INSPECT__, args)
	}

	#[inline]
	pub fn qs_eql(this: &Object, args: Args) -> crate::Result<bool> {
		Ok(this.is_identical(args.arg(0)?).into())
	}

	#[inline]
	pub fn qs_neq(this: &Object, args: Args) -> crate::Result<Object> {
		this.call_attr_lit(EQL, args)?.call_attr_lit(NOT, &[])
	}

	#[inline]
	pub fn qs_not(this: &Object, args: Args) -> crate::Result<Object> {
		this.call_attr_lit(AT_BOOL, args)?.call_attr_lit(NOT, &[])
	}

	#[inline]
	pub fn qs_clone(this: &Object, _: Args) -> Result<Object, !> {
		Ok(this.deep_clone())
	}
}

impl_object_type!{
for Basic [(parents super::Kernel)]:
	"@bool" => function Basic::qs_at_bool,
	"@text" => function Basic::qs_at_text,
	"clone" => function Basic::qs_clone,
	"==" => function Basic::qs_eql,
	"!=" => function Basic::qs_neq,
	"!" => function Basic::qs_not,
	// "||"    => impls::or,
	// "&&"    => impls::and,
}


#[cfg(test)]
mod tests {
	use super::*;
	// use crate::{Object};

	dummy_object!(struct Dummy;);

	#[test]
	fn at_bool() {
		assert_eq!(Basic::qs_at_bool(&Dummy.into(), args!()).unwrap(), true);
	}

	#[test]
	fn at_text() {
		/* we don't test this, as the output is unspecified in general */
	}

	#[test]
	fn eql() {
		// let dummy: Object = Dummy.into();
		// use super::super::ObjectType;
		// Dummy::_wait_for_setup_to_finish();
		// Basic::_wait_for_setup_to_finish();
		// crate::types::Number::_wait_for_setup_to_finish();
		// assert_call_eq!(for Basic;
		// 	Boolean::TRUE, eql(dummy.clone(), dummy.clone()) -> Boolean,
		// 	Boolean::FALSE, eql(dummy.clone(), Dummy) -> Boolean,
		// 	Boolean::FALSE, eql(Dummy, Dummy) -> Boolean,
		// );
	}

	#[test]
	// #[should_panic]
	fn eql_no_arg() {
		// call_impl!(eql(Dummy) -> Boolean);
	}

	#[test]
	fn neq() {
		// dummy_object!(struct DummyEqlOverride(i32, bool); {
		// 	"==" => function (|this: &DummyEqlOverride, args| Ok({
		// 		if this.1 {
		// 			this.0 == args.arg(0)?.try_downcast_ref::<DummyEqlOverride>()?.0
		// 		} else {
		// 			false
		// 		}
		// 	}.into()))
		// });

		// let _dummy: Object = Dummy.into();

		// // TODO: remove the need to `_wait_for_setup_to_finish`...
		// use super::super::ObjectType;
		// DummyEqlOverride::_wait_for_setup_to_finish();
		// Dummy::_wait_for_setup_to_finish();
		// crate::types::Number::_wait_for_setup_to_finish();

		// assert_call_eq!(for Basic;
		// 	Boolean::FALSE, neq(dummy.clone(), dummy.clone()) -> Boolean,
		// 	Boolean::TRUE, neq(dummy.clone(), Dummy) -> Boolean,
		// 	Boolean::TRUE, neq(Dummy, Dummy) -> Boolean,
		// 	Boolean::FALSE, neq(DummyEqlOverride(0x1EE7, true), DummyEqlOverride(0x1EE7, true)) -> Boolean,
		// 	Boolean::TRUE, neq(DummyEqlOverride(0x1EE7, true), DummyEqlOverride(0, true)) -> Boolean,
		// 	Boolean::TRUE, neq(DummyEqlOverride(0x1EE7, false), DummyEqlOverride(0, true)) -> Boolean,
		// );
	}

	#[test]
	// #[should_panic]
	fn neq_no_arg() {
		// call_impl!(neq(Dummy) -> Boolean);
	}

	#[test]
	fn not() {
		// dummy_object!(struct DummyBoolOverride(bool); crate::types::Basic {
		// 	"@bool" => (|args| {
		// 		Ok(args.this()?.try_downcast_ref::<DummyBoolOverride>()?.0.into())
		// 	})
		// });

		// use super::super::ObjectType;
		// DummyBoolOverride::_wait_for_setup_to_finish();
		// Dummy::_wait_for_setup_to_finish();

		// assert_call_eq!(for Basic;
		// 	Boolean::FALSE, not(Dummy) -> Boolean,
		// 	Boolean::FALSE, not(DummyBoolOverride(true)) -> Boolean,
		// 	Boolean::TRUE, not(DummyBoolOverride(false)) -> Boolean
		// );
	}
}
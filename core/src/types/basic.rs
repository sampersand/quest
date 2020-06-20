#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Basic;

mod impls {
	use crate::{Object, Result, Args, types, literals};

	pub fn at_bool(_: Args) -> Result<Object> {
		Ok(true.into())
	}

	// This function needs to be updated
	pub fn at_text(args: Args) -> Result<Object> {
		let this = args.this()?;
		let name = 
			this.get_attr(literals::PARENTS)?
				.get_attr("name")
				.and_then(|x| x.downcast_call::<types::Text>())
				.unwrap_or_else(|_| "<unknown name>".into());

		Ok(format!("<{}:{}>", name.as_ref(), this.id()).into())
	}

	pub fn eql(args: Args) -> Result<Object> {
		// TODO: do we want the `id` here to be overridable?
		let this_id = args.this()?.id();
		let other_id = args.arg(0)?.id();

		Ok((this_id == other_id).into())
	}

	pub fn neq(args: Args) -> Result<Object> {
		args.this()?
			.call_attr(literals::EQL, args.args(..)?)?
			.call_attr(literals::NOT, args.new_args_slice(&[]))
	}

	pub fn not(args: Args) -> Result<Object> {
		args.this()?.downcast_convert::<types::Boolean>()?
			.call_attr(literals::NOT, args.new_args_slice(&[]))
	}
}

impl_object_type!{
for Basic [(parents super::Kernel)]:
	"@bool" => impls::at_bool,
	"@text" => impls::at_text,
	"==" => impls::eql,
	"!=" => impls::neq,
	"!" => impls::not,
	// "||"    => impls::or,
	// "&&"    => impls::and,
	"ancestors" => (|_args| todo!()) // this is just a reminder to update `__parent__`...
}


#[cfg(test)]
mod tests {
	use super::*;
	use crate::Object;

	dummy_object!(struct Dummy;);

	#[test]
	fn at_bool() {
		assert_call_eq!(for Basic;
			Boolean::TRUE, at_bool(Dummy) -> Boolean
		);
	}

	#[test]
	fn at_text() {
		/* we don't test this, as the output is unspecified in general */
	}

	#[test]
	fn eql() {
		let dummy: Object = Dummy.into();
		use super::super::ObjectType;
		Dummy::_wait_for_setup_to_finish();
		Basic::_wait_for_setup_to_finish();
		crate::types::Number::_wait_for_setup_to_finish();
		assert_call_eq!(for Basic;
			Boolean::TRUE, eql(dummy.clone(), dummy.clone()) -> Boolean,
			Boolean::FALSE, eql(dummy.clone(), Dummy) -> Boolean,
			Boolean::FALSE, eql(Dummy, Dummy) -> Boolean,
		);
	}

	#[test]
	#[should_panic]
	fn eql_no_arg() {
		call_impl!(eql(Dummy) -> Boolean);
	}

	#[test]
	fn neq() {
		dummy_object!(struct DummyEqlOverride(i32, bool); {
			"==" => (|args| Ok({
				let this = args.this()?.try_downcast_ref::<DummyEqlOverride>()?;
				if this.1 {
					this.0 == args.arg(0)?.try_downcast_ref::<DummyEqlOverride>()?.0
				} else {
					false
				}
			}.into()))
		});

		let dummy: Object = Dummy.into();

		// TODO: remove the need to `_wait_for_setup_to_finish`...
		use super::super::ObjectType;
		DummyEqlOverride::_wait_for_setup_to_finish();
		Dummy::_wait_for_setup_to_finish();
		crate::types::Number::_wait_for_setup_to_finish();

		assert_call_eq!(for Basic;
			Boolean::FALSE, neq(dummy.clone(), dummy.clone()) -> Boolean,
			Boolean::TRUE, neq(dummy.clone(), Dummy) -> Boolean,
			Boolean::TRUE, neq(Dummy, Dummy) -> Boolean,
			Boolean::FALSE, neq(DummyEqlOverride(0x1EE7, true), DummyEqlOverride(0x1EE7, true)) -> Boolean,
			Boolean::TRUE, neq(DummyEqlOverride(0x1EE7, true), DummyEqlOverride(0, true)) -> Boolean,
			Boolean::TRUE, neq(DummyEqlOverride(0x1EE7, false), DummyEqlOverride(0, true)) -> Boolean,
		);
	}

	#[test]
	#[should_panic]
	fn neq_no_arg() {
		call_impl!(neq(Dummy) -> Boolean);
	}

	#[test]
	fn not() {
		dummy_object!(struct DummyBoolOverride(bool); crate::types::Basic {
			"@bool" => (|args| {
				Ok(args.this()?.try_downcast_ref::<DummyBoolOverride>()?.0.into())
			})
		});

		use super::super::ObjectType;
		DummyBoolOverride::_wait_for_setup_to_finish();
		Dummy::_wait_for_setup_to_finish();

		assert_call_eq!(for Basic;
			Boolean::FALSE, not(Dummy) -> Boolean,
			Boolean::FALSE, not(DummyBoolOverride(true)) -> Boolean,
			Boolean::TRUE, not(DummyBoolOverride(false)) -> Boolean
		);
	}
}
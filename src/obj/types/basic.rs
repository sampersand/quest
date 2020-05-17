#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Basic;

mod impls {
	use super::Basic;
	use crate::obj::{Object, Result, Args, types};

	pub fn at_bool(_: Args) -> Result<Object> {
		Ok(types::boolean::TRUE.into())
	}

	pub fn at_text(args: Args) -> Result<Object> {
		let this = args._this()?;
		Ok(format!("<{}:{}>",
			this.get_attr(&"__parent__".into(), args.binding())?
				.get_attr(&"name".into(), args.binding())
				.and_then(|x| x.call("@text", args.new_args_slice(&[])))
				.unwrap_or_else(|_| "<unknown name>".into())
				.try_downcast_ref::<types::Text>()?
				.as_ref(),
			this.id()
		).into())
	}

	pub fn eql(args: Args) -> Result<Object> {
		// TODO: do we want the `id` here to be overridable?
		let lhs_id = args.this()?.call("__id__", args.new_args_slice(&[]))?;
		let rhs_id = args.arg(0)?.call("__id__", args.new_args_slice(&[]))?;
		lhs_id.call("==", args.new_args_slice(&[rhs_id]))
	}

	pub fn neq(args: Args) -> Result<Object> {
		args.this()?
			.call("==", args.args(..)?)?
			.call("!", args.new_args_slice(&[]))
	}

	pub fn not(args: Args) -> Result<Object> {
		args.this()?
			.call("@bool", args.args(..)?)?
			.call("!", args.new_args_slice(&[]))
	}
}

impl_object_type_!{for Basic, super::Pristine;
	"@bool" => (impls::at_bool),
	"@text" => (impls::at_text),
	"=="    => (impls::eql),
	"!="    => (impls::neq),
	"!"     => (impls::not),
	"ancestors" => (|args| todo!()) // this is just a reminder to update `__parent__`...
}


#[cfg(test)]
mod tests {
	use super::*;
	use crate::obj::Object;

	dummy_object!(struct Dummy;);

	#[test]
	fn at_bool() {
		assert_call_eq!(for Basic;
			boolean::TRUE, at_bool(Dummy) -> Boolean
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
		crate::obj::types::Number::_wait_for_setup_to_finish();
		assert_call_eq!(for Basic;
			boolean::TRUE, eql(dummy.clone(), dummy.clone()) -> Boolean,
			boolean::FALSE, eql(dummy.clone(), Dummy) -> Boolean,
			boolean::FALSE, eql(Dummy, Dummy) -> Boolean,
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
				let this = args.this_downcast::<DummyEqlOverride>()?;
				if this.1 {
					this.0 == args.arg_downcast::<DummyEqlOverride>(0)?.0
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
		crate::obj::types::Number::_wait_for_setup_to_finish();

		assert_call_eq!(for Basic;
			boolean::FALSE, neq(dummy.clone(), dummy.clone()) -> Boolean,
			boolean::TRUE, neq(dummy.clone(), Dummy) -> Boolean,
			boolean::TRUE, neq(Dummy, Dummy) -> Boolean,
			boolean::FALSE, neq(DummyEqlOverride(0x1EE7, true), DummyEqlOverride(0x1EE7, true)) -> Boolean,
			boolean::TRUE, neq(DummyEqlOverride(0x1EE7, true), DummyEqlOverride(0, true)) -> Boolean,
			boolean::TRUE, neq(DummyEqlOverride(0x1EE7, false), DummyEqlOverride(0, true)) -> Boolean,
		);
	}

	#[test]
	#[should_panic]
	fn neq_no_arg() {
		call_impl!(neq(Dummy) -> Boolean);
	}

	#[test]
	fn not() {
		dummy_object!(struct DummyBoolOverride(bool); crate::obj::types::Basic {
			"@bool" => (|args| {
				Ok(args.this_downcast::<DummyBoolOverride>()?.0.into())
			})
		});

		use super::super::ObjectType;
		DummyBoolOverride::_wait_for_setup_to_finish();
		Dummy::_wait_for_setup_to_finish();

		assert_call_eq!(for Basic;
			boolean::FALSE, not(Dummy) -> Boolean,
			boolean::FALSE, not(DummyBoolOverride(true)) -> Boolean,
			boolean::TRUE, not(DummyBoolOverride(false)) -> Boolean
		);
	}
}
use crate::{Object, Result, Args};

/// Represents the ability of a type to act as a function.
///
/// This was created really early on, but never actually fleshed out. I'm not sure if it'll stay
/// around forever, but it could come in handy for checking if things should become
/// [`BoundObject`](crate::types::BoundObject)s.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Function;

impl Function {
	/// TODO
	pub fn qs_lsh(_this: &Object, _args: Args) -> Result<Object> {
		todo!("<<")
	}

	/// TODO
	pub fn qs_rsh(_this: &Object, _args: Args) -> Result<Object> {
		todo!(">>")
	}

	/// TODO
	pub fn qs_curry(_this: &Object, _args: Args) -> Result<Object> {
		todo!("curry")
	}
}

impl_object_type!{
for Function [(parents super::Basic)]:
	"<<" => function Self::qs_lsh,
	">>" => function Self::qs_rsh,
	"curry" => function Self::qs_curry
}

mod tests {
	#[test]
	#[ignore]
	fn lsh() { todo!() }

	#[test]
	#[ignore]
	fn rsh() { todo!() }

	#[test]
	#[ignore]
	fn curry() { todo!() }
}

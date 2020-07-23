use crate::{Object, Result, Args};
use crate::literal::CALL;

/// A type representing a bound function.
///
/// This may be removed in the future, as all it's used for is wrapping callable objects.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoundFunction;

impl BoundFunction {
	/// Call this function with the specified args, passing them on to the unbound object.
	pub fn qs_call(this: &Object, args: Args) -> Result<Object> {
		let bound_owner = &this.get_attr_lit("__bound_object_owner__")?;
		let bound_object = this.get_attr_lit("__bound_object__")?;
		let args: Args = std::iter::once(bound_owner).chain(args.into_iter()).collect();
		bound_object.call_attr_lit(CALL, args)
	}
}

impl_object_type!{
for BoundFunction [(parents super::Basic)]:
	"()" => function Self::qs_call,
}




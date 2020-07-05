use crate::{Object, Args, Result};
use crate::literals::CMP;
use crate::types::{Number, Null};
use std::cmp::Ordering;

/// A mixin which supplies the [`<`](#qs_lth), [`<=`](#qs_leq), [`>`](#qs_gth), and
/// [`>=`](#qs_geq) methods via `<=>`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Comparable;

#[inline]
fn compare<F: FnOnce(Ordering) -> bool>(lhs: &Object, rhs: &Object, f: F) -> Result<Object> {
	let cmp = lhs.call_attr_lit(CMP, &[rhs])?;
	if cmp.is_a::<Null>() {
		Ok(cmp)
	} else {
		cmp.call_downcast_map(|n: &Number| f(n.cmp(&Number::ZERO)).into())
	}

}

/// Quest functions
impl Comparable {
	/// The `<` operator.
	///
	/// Returns [`true`](crate::Boolean::TRUE) when `(this <=> rhs) < 0`.
	#[inline]
	pub fn qs_lth(this: &Object, args: Args) -> Result<Object> {
		compare(this, args.arg(0)?, |ord| ord < Ordering::Equal)
	}

	/// The `<=` operator.
	///
	/// Returns [`true`](crate::Boolean::TRUE) when `(this <=> rhs) <= 0`.
	#[inline]
	pub fn qs_leq(this: &Object, args: Args) -> Result<Object> {
		compare(this, args.arg(0)?, |ord| ord <= Ordering::Equal)
	}

	/// The `>` operator.
	///
	/// Returns [`true`](crate::Boolean::TRUE) when `(this <=> rhs) > 0`.
	#[inline]
	pub fn qs_gth(this: &Object, args: Args) -> Result<Object> {
		compare(this, args.arg(0)?, |ord| ord > Ordering::Equal)
	}

	/// The `>=` operator.
	///
	/// Returns [`true`](crate::Boolean::TRUE) when `(this <=> rhs) >= 0`.
	#[inline]
	pub fn qs_geq(this: &Object, args: Args) -> Result<Object> {
		compare(this, args.arg(0)?, |ord| ord >= Ordering::Equal)
	}
}

impl_object_type!{
for Comparable [(parents super::Pristine)]:
	"<" => function Comparable::qs_lth,
	">" => function Comparable::qs_gth,
	"<=" => function Comparable::qs_leq,
	">=" => function Comparable::qs_geq,
}


impl From<Ordering> for Number {
	/// Convert from an Ordering to a Number
	///
	/// - [`Less`](Ordering::Less) becomes `-1`
	/// - [`Equal`](Ordering::Equal) becomes `0`
	/// - [`Greater`](Ordering::Greater) becomes `1`
	#[inline]
	fn from(ord: Ordering) -> Self {
		match ord {
			Ordering::Less => -Number::ONE,
			Ordering::Equal => Number::ZERO,
			Ordering::Greater => Number::ONE,
		}
	}
}

impl From<Ordering> for Object {
	#[inline]
	fn from(ord: Ordering) -> Self {
		Number::from(ord).into()
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use crate::types::{Boolean, Null};

	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
	struct Dummy(i32);

	impl_object_type! {
		for Dummy [(parents crate::types::Basic)]:
		"<=>" => function (|this: &Object, args: crate::Args| -> crate::Result<Object> {
			let this: Self = this.try_downcast_map(Self::clone)?;

			Ok(args.arg(0)?
				.downcast_and_then(|rhs: &Self| this.cmp(rhs).into())
				.unwrap_or_default())
		})
	}

	#[derive(Debug, Clone)]
	struct Other;
	impl_object_type!(for Other [(parents crate::types::Basic) (setup SETUP_OTHER)]: );

	#[test]
	fn lth() {
		assert_contains!(Comparable, "<");
		assert_contains!(Dummy, "<=>");

		assert_downcast_eq!(Boolean;
			Comparable::qs_lth(&Dummy(12).into(), args!(Dummy(12))).unwrap(), false);

		assert_downcast_eq!(Boolean;
			Comparable::qs_lth(&Dummy(12).into(), args!(Dummy(13))).unwrap(), true);

		assert_downcast_eq!(Boolean;
			Comparable::qs_lth(&Dummy(13).into(), args!(Dummy(12))).unwrap(), false);

		assert!(Comparable::qs_lth(&Dummy(13).into(), args!(Other)).unwrap().is_a::<Null>());
		assert_missing_parameter!(Comparable::qs_lth(&Dummy(13).into(), args!()), 0);

		assert_downcast_eq!(Boolean;
			Comparable::qs_lth(&Dummy(13).into(), args!(Dummy(12), Dummy(14))).unwrap(), false);
	}

	#[test]
	fn gth() {
		assert_contains!(Comparable, ">");
		assert_contains!(Dummy, "<=>");

		assert_downcast_eq!(Boolean;
			Comparable::qs_gth(&Dummy(12).into(), args!(Dummy(12))).unwrap(), false);

		assert_downcast_eq!(Boolean;
			Comparable::qs_gth(&Dummy(12).into(), args!(Dummy(13))).unwrap(), false);

		assert_downcast_eq!(Boolean;
			Comparable::qs_gth(&Dummy(13).into(), args!(Dummy(12))).unwrap(), true);

		assert!(Comparable::qs_gth(&Dummy(13).into(), args!(Other)).unwrap().is_a::<Null>());
		assert_missing_parameter!(Comparable::qs_gth(&Dummy(13).into(), args!()), 0);

		assert_downcast_eq!(Boolean;
			Comparable::qs_gth(&Dummy(13).into(), args!(Dummy(14), Dummy(12))).unwrap(), false);
	}

	#[test]
	fn leq() {
		assert_contains!(Comparable, "<=");
		assert_contains!(Dummy, "<=>");

		assert_downcast_eq!(Boolean;
			Comparable::qs_leq(&Dummy(12).into(), args!(Dummy(12))).unwrap(), true);

		assert_downcast_eq!(Boolean;
			Comparable::qs_leq(&Dummy(12).into(), args!(Dummy(13))).unwrap(), true);

		assert_downcast_eq!(Boolean;
			Comparable::qs_leq(&Dummy(13).into(), args!(Dummy(12))).unwrap(), false);

		assert!(Comparable::qs_leq(&Dummy(13).into(), args!(Other)).unwrap().is_a::<Null>());
		assert_missing_parameter!(Comparable::qs_leq(&Dummy(13).into(), args!()), 0);

		assert_downcast_eq!(Boolean;
			Comparable::qs_leq(&Dummy(13).into(), args!(Dummy(12), Dummy(14))).unwrap(), false);
	}

	#[test]
	fn geq() {
		assert_contains!(Comparable, ">=");
		assert_contains!(Dummy, "<=>");

		assert_downcast_eq!(Boolean;
			Comparable::qs_geq(&Dummy(12).into(), args!(Dummy(12))).unwrap(), true);

		assert_downcast_eq!(Boolean;
			Comparable::qs_geq(&Dummy(12).into(), args!(Dummy(13))).unwrap(), false);

		assert_downcast_eq!(Boolean;
			Comparable::qs_geq(&Dummy(13).into(), args!(Dummy(12))).unwrap(), true);

		assert!(Comparable::qs_geq(&Dummy(13).into(), args!(Other)).unwrap().is_a::<Null>());
		assert_missing_parameter!(Comparable::qs_geq(&Dummy(13).into(), args!()), 0);

		assert_downcast_eq!(Boolean;
			Comparable::qs_geq(&Dummy(13).into(), args!(Dummy(14), Dummy(12))).unwrap(), false);
	}
}

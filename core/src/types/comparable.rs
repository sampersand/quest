use crate::{Object, Args, Literal};
use crate::types::Number;
use std::cmp::Ordering;
use tracing::instrument;

/// A mixinable class that defines comparison operators based on `<=>`.
///
/// More specifically, the [`<`](Comparable::qs_lth), [`<=`](Comparable::qs_leq), [`>`](Comparable::qs_gth), and
/// [`>=`](Comparable::qs_geq) functions are defined based on the return value of `<=>`:
/// - if `lhs <=> rhs` is a negative [`Number`], then `lhs < rhs` and `lhs <= rhs`.
/// - if `lhs <=> rhs` is the number [zero](Number::ZERO), then `lhs <= rhs` and `lhs >= rhs`.
/// - if `lhs <=> rhs` is a positive [`Number`],  then `lhs > rhs` and `lhs >= rhs`.
/// - otherwise, all comparisons will return false.
///
/// Notably the `==` and `!=` operators aren't defined here---they may have different semantics, such as not
/// automatically coercing types.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Comparable;

/// Compare the two sides.
fn compare(lhs: &Object, rhs: &Object) -> crate::Result<Ordering> {
	let num = lhs.call_attr_lit(&Literal::CMP, &[rhs])?;
	let num = num.call_downcast::<Number>()?;
	Ok(num.cmp(&Number::ZERO))
}

impl Comparable {
	/// Check to see if `this` is less than the first argument in `args`.
	///
	/// This returns `true` if the result of `this <=> rhs` is a negative [`Number`].
	#[instrument(name="Comparable::<", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_lth(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?;
		let cmp = compare(this, rhs)?;

		Ok((cmp == Ordering::Less).into())
	}

	/// Check to see if `this` is greater than the first argument in `args`.
	///
	/// This returns `true` if the result of `this <=> rhs` is a positive [`Number`].
	#[instrument(name="Comparable::>", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_gth(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?;
		let cmp = compare(this, rhs)?;

		Ok((cmp == Ordering::Greater).into())
	}

	/// Check to see if `this` is less than or equal to the first argument in `args`.
	///
	/// This returns `true` if the result of `this <=> rhs` is either a negative [`Number`] or [zero](Number::ZERO).
	#[instrument(name="Comparable::<=", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_leq(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?;
		let cmp = compare(this, rhs)?;

		Ok((cmp != Ordering::Greater).into())
	}

	/// Check to see if `this` is less than or equal to the first argument in `args`.
	///
	/// This returns `true` if the result of `this <=> rhs` is either a positive [`Number`] or [zero](Number::ZERO).
	#[instrument(name="Comparable::>=", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_geq(this: &Object, args: Args) -> crate::Result<Object> {
		let rhs = args.try_arg(0)?;
		let cmp = compare(this, rhs)?;

		Ok((cmp != Ordering::Less).into())
	}
}

impl_object_type!{
for Comparable [(parents super::Basic)]:
	"<" => function Self::qs_lth,
	">" => function Self::qs_gth,
	"<=" => function Self::qs_leq,
	">=" => function Self::qs_geq,
}

impl From<Ordering> for crate::Object {
	/// Simply converts the [`Ordering`] to a [`Number`] and then into an [`Object`].
	#[inline]
	fn from(ord: Ordering) -> Self {
		Number::from(ord).into()
	}
}

impl From<Ordering> for Number {
	/// Converts [`Less`](Ordering::Less) to negative one, [`Equal`](Ordering::Equal) to zero and
	/// [`Greater`](Ordering::Greater) to one.
	fn from(ord: Ordering) -> Self {
		match ord {
			Ordering::Less => -Self::ONE,
			Ordering::Equal => Self::ZERO,
			Ordering::Greater => Self::ONE,
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
	struct DummyCmp(u8);

	impl_object_type! { for DummyCmp [(parents crate::types::Basic)]:
		"<=>" => function |this: &Object, args: Args| {
			let this = this.try_downcast::<DummyCmp>()?;
			let rhs = args.try_arg(0)?.try_downcast::<DummyCmp>()?;
			Ok(this.0.cmp(&rhs.0).into())
		}
	}

	#[test]
	fn lth() {
		<DummyCmp as crate::types::ObjectType>::initialize().unwrap();

		// assert_call_eq!(Comparable::qs_lth(DummyCmp(0), Number::default()) -> Boolean, false);
		assert_call_eq!(Comparable::qs_lth(DummyCmp(1), DummyCmp(1)) -> Boolean, false);
		assert_call_eq!(Comparable::qs_lth(DummyCmp(1), DummyCmp(0)) -> Boolean, false);
		assert_call_eq!(Comparable::qs_lth(DummyCmp(1), DummyCmp(2)) -> Boolean, true);
	}

	#[test]
	fn gth() {
		<DummyCmp as crate::types::ObjectType>::initialize().unwrap();

		// assert_call_eq!(Comparable::qs_gth(DummyCmp(0), Number::default()) -> Boolean, false);
		assert_call_eq!(Comparable::qs_gth(DummyCmp(1), DummyCmp(1)) -> Boolean, false);
		assert_call_eq!(Comparable::qs_gth(DummyCmp(1), DummyCmp(0)) -> Boolean, true);
		assert_call_eq!(Comparable::qs_gth(DummyCmp(1), DummyCmp(2)) -> Boolean, false);
	}

	#[test]
	fn leq() {
		<DummyCmp as crate::types::ObjectType>::initialize().unwrap();

		// assert_call_eq!(Comparable::qs_leq(DummyCmp(0), Number::default()) -> Boolean, false);
		assert_call_eq!(Comparable::qs_leq(DummyCmp(1), DummyCmp(1)) -> Boolean, true);
		assert_call_eq!(Comparable::qs_leq(DummyCmp(1), DummyCmp(0)) -> Boolean, false);
		assert_call_eq!(Comparable::qs_leq(DummyCmp(1), DummyCmp(2)) -> Boolean, true);
	}

	#[test]
	fn geq() {
		<DummyCmp as crate::types::ObjectType>::initialize().unwrap();

		// assert_call_eq!(Comparable::qs_geq(DummyCmp(0), Number::default()) -> Boolean, false);
		assert_call_eq!(Comparable::qs_geq(DummyCmp(1), DummyCmp(1)) -> Boolean, true);
		assert_call_eq!(Comparable::qs_geq(DummyCmp(1), DummyCmp(0)) -> Boolean, true);
		assert_call_eq!(Comparable::qs_geq(DummyCmp(1), DummyCmp(2)) -> Boolean, false);
	}
}

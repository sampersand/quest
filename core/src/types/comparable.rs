use crate::{Object, Args};
use crate::literal::CMP;
use crate::types::Number;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Comparable;

#[inline]
fn compare(lhs: &Object, rhs: &Object) -> crate::Result<Ordering> {
	let num = lhs.call_attr_lit(CMP, &[rhs])?.call_downcast_map(Number::clone)?;
	Ok(num.cmp(&Number::ZERO))
}

impl Comparable {
	pub fn qs_lth(this: &Object, args: Args) -> crate::Result<Object> {
		compare(this, args.arg(0)?).map(|ord| (ord == Ordering::Less).into())
	}

	pub fn qs_gth(this: &Object, args: Args) -> crate::Result<Object> {
		compare(this, args.arg(0)?).map(|ord| (ord == Ordering::Greater).into())
	}

	pub fn qs_leq(this: &Object, args: Args) -> crate::Result<Object> {
		compare(this, args.arg(0)?).map(|ord| (ord != Ordering::Greater).into())
	}

	pub fn qs_geq(this: &Object, args: Args) -> crate::Result<Object> {
		compare(this, args.arg(0)?).map(|ord| (ord != Ordering::Less).into())
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
	fn from(ord: Ordering) -> Self {
		crate::types::Number::from(ord).into()
	}
}

impl From<Ordering> for crate::types::Number {
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
			let rhs = args.arg(0)?.try_downcast::<DummyCmp>()?;
			Ok(this.0.cmp(&rhs.0).into())
		}
	}

	#[test]
	fn lth() {
		<DummyCmp as crate::types::ObjectType>::initialize().unwrap();

		assert_call_eq!(Comparable::qs_lth(DummyCmp(1), DummyCmp(1)) -> Boolean, false);
		assert_call_eq!(Comparable::qs_lth(DummyCmp(1), DummyCmp(0)) -> Boolean, false);
		assert_call_eq!(Comparable::qs_lth(DummyCmp(1), DummyCmp(2)) -> Boolean, true);
	}

	#[test]
	fn gth() {
		<DummyCmp as crate::types::ObjectType>::initialize().unwrap();

		assert_call_eq!(Comparable::qs_gth(DummyCmp(1), DummyCmp(1)) -> Boolean, false);
		assert_call_eq!(Comparable::qs_gth(DummyCmp(1), DummyCmp(0)) -> Boolean, true);
		assert_call_eq!(Comparable::qs_gth(DummyCmp(1), DummyCmp(2)) -> Boolean, false);
	}

	#[test]
	fn leq() {
		<DummyCmp as crate::types::ObjectType>::initialize().unwrap();

		assert_call_eq!(Comparable::qs_leq(DummyCmp(1), DummyCmp(1)) -> Boolean, true);
		assert_call_eq!(Comparable::qs_leq(DummyCmp(1), DummyCmp(0)) -> Boolean, false);
		assert_call_eq!(Comparable::qs_leq(DummyCmp(1), DummyCmp(2)) -> Boolean, true);
	}

	#[test]
	fn geq() {
		<DummyCmp as crate::types::ObjectType>::initialize().unwrap();

		assert_call_eq!(Comparable::qs_geq(DummyCmp(1), DummyCmp(1)) -> Boolean, true);
		assert_call_eq!(Comparable::qs_geq(DummyCmp(1), DummyCmp(0)) -> Boolean, true);
		assert_call_eq!(Comparable::qs_geq(DummyCmp(1), DummyCmp(2)) -> Boolean, false);
	}
}

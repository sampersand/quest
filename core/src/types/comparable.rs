use crate::{Object, Args, Result};
use crate::literals::CMP;
use crate::types::Number;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Comparable;

fn compare(lhs: &Object, rhs: &Object) -> Result<Ordering> {
	let num = lhs.call_attr_lit(CMP, &[rhs])?.call_downcast_map(Number::clone)?;
	Ok(num.cmp(&Number::ZERO))
}

impl Comparable {
	#[inline]
	pub fn qs_lth(this: &Object, args: Args) -> Result<bool> {
		compare(this, args.arg(0)?).map(|ord| ord == Ordering::Less)
	}

	#[inline]
	pub fn qs_gth(this: &Object, args: Args) -> Result<bool> {
		compare(this, args.arg(0)?).map(|ord| ord == Ordering::Greater)
	}

	#[inline]
	pub fn qs_leq(this: &Object, args: Args) -> Result<bool> {
		compare(this, args.arg(0)?).map(|ord| ord != Ordering::Greater)
	}

	#[inline]
	pub fn qs_geq(this: &Object, args: Args) -> Result<bool> {
		compare(this, args.arg(0)?).map(|ord| ord != Ordering::Less)
	}
}


impl_object_type!{
for Comparable [(parents super::Basic)]:
	"<" => function Comparable::qs_lth,
	">" => function Comparable::qs_gth,
	"<=" => function Comparable::qs_leq,
	">=" => function Comparable::qs_geq,
}

impl From<Ordering> for crate::Object {
	#[inline]
	fn from(ord: Ordering) -> Self {
		match ord {
			Ordering::Less => -1,
			Ordering::Equal => 0,
			Ordering::Greater => 1,
		}.into()
	}
}


#[cfg(test)]
mod tests {
	// use super::*;

	// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
	// struct DummyCmp(f32);

	// impl_object_type!(f)


	// dummy_object_old!(struct DummyCmp(f32); {
	// 	"<=>" => function (|this, args| {
	// 		let this = this.downcast_and_then(DummyCmp::clone)?;
	// 		let rhs = rhs.downcast_and_then(DummyCmp::clone)?;

	// 		Ok(this.cmp(other))
	// 		Ok(this.0.partial_cmp(&other.0)
	// 			.map(Into::into)
	// 			.unwrap_or_default())
	// 	})
	// });

	#[test]
	#[ignore]
	fn lth() {
		// let _obj = Object::from(DummyCmp(12.0));
		// assert_eq!(, );
		// Ok((cmp(args)? == Ordering::Less).into())
		unimplemented!()
	}

	#[test]
	#[ignore]
	fn gth() {
		// Ok((cmp(args)? == Ordering::Greater).into())
		unimplemented!()
	}

	#[test]
	#[ignore]
	fn leq() {
		// Ok((cmp(args)? != Ordering::Greater).into())
		unimplemented!()
	}

	#[test]
	#[ignore]
	fn geq() {
		// Ok((cmp(args)? != Ordering::Less).into())
		unimplemented!()
	}
}









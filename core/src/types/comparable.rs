use crate::{Object, Args, Result};
use crate::types::Number;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Comparable;

fn compare(lhs: &Object, rhs: &Object) -> Result<Ordering> {
	let num = lhs.call_attr("<=>", &[rhs])?.downcast_call::<Number>()?;
	if num < Number::ZERO {
		Ok(Ordering::Less)
	} else if num > Number::ZERO {
		Ok(Ordering::Greater)
	} else {
		Ok(Ordering::Equal)
	}
}

impl Comparable {
	pub fn qs_lth(this: &Object, args: Args) -> Result<bool> {
		Ok(compare(this, args.arg(0)?)? == Ordering::Less)
	}

	pub fn qs_gth(this: &Object, args: Args) -> Result<bool> {
		Ok(compare(this, args.arg(0)?)? == Ordering::Greater)
	}

	pub fn qs_leq(this: &Object, args: Args) -> Result<bool> {
		Ok(compare(this, args.arg(0)?)? != Ordering::Greater)
	}

	pub fn qs_geq(this: &Object, args: Args) -> Result<bool> {
		Ok(compare(this, args.arg(0)?)? != Ordering::Less)
	}
}


impl_object_type!{
for Comparable [(parents super::Basic)]:
	"<" => function Comparable::qs_lth,
	">" => function Comparable::qs_gth,
	"<=" => function Comparable::qs_leq,
	">=" => function Comparable::qs_geq,
	// "==" => impls::eql,
	// "!=" => impls::neq,
}

impl From<Ordering> for crate::Object {
	fn from(ord: Ordering) -> Self {
		match ord {
			Ordering::Less => -1,
			Ordering::Equal => 0,
			Ordering::Greater => 1,
		}.into()
	}
}

#[allow(unused)]
#[cfg(test)]
mod tests {
	use crate::Object;
	dummy_object!(struct DummyCmp(f32); {
		"<=>" => (|args| {
			let this = args.this()?.try_downcast_ref::<DummyCmp>()?;
			let other = args.arg(0)?.try_downcast_ref::<DummyCmp>()?;

			Ok(this.0.partial_cmp(&other.0)
				.map(Into::into)
				.unwrap_or_default())
		})
	});

	#[test]
	#[ignore]
	fn lth() {
		let _obj = Object::from(DummyCmp(12.0));
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









use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Comparable;

mod impls {
	use super::Ordering;
	use crate::{Object, Result, ArgsOld, types};

	fn cmp(args: ArgsOld) -> Result<Ordering> {
		let this = args.this()?;
		let rhs = args.arg(0)?;
		let num = this.call_attr_old("<=>", vec![rhs.clone()])?.downcast_call::<types::Number>()?;

		if num < types::Number::ZERO {
			Ok(Ordering::Less)
		} else if num > types::Number::ZERO {
			Ok(Ordering::Greater)
		} else {
			Ok(Ordering::Equal)
		}
	}

	pub fn lth(args: ArgsOld) -> Result<Object> {
		Ok((cmp(args)? == Ordering::Less).into())
	}

	pub fn gth(args: ArgsOld) -> Result<Object> {
		Ok((cmp(args)? == Ordering::Greater).into())
	}

	// pub fn eql(args: ArgsOld) -> Result<Object> {
	// 	Ok((cmp(args)? == Ordering::Equal).into())
	// }

	pub fn leq(args: ArgsOld) -> Result<Object> {
		Ok((cmp(args)? != Ordering::Greater).into())
	}

	pub fn geq(args: ArgsOld) -> Result<Object> {
		Ok((cmp(args)? != Ordering::Less).into())
	}

	// pub fn neq(args: ArgsOld) -> Result<Object> {
	// 	Ok((cmp(args)? != Ordering::Equal).into())
	// }

}

impl_object_type!{
for Comparable [(parents super::Basic)]:
	"<" => impls::lth,
	">" => impls::gth,
	"<=" => impls::leq,
	">=" => impls::geq,
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
	fn lth() {
		let _obj = Object::from(DummyCmp(12.0));
		// assert_eq!(, );
		// Ok((cmp(args)? == Ordering::Less).into())
		unimplemented!()
	}

	#[test]
	fn gth() {
		// Ok((cmp(args)? == Ordering::Greater).into())
		unimplemented!()
	}

	#[test]
	fn eql() {
		// Ok((cmp(args)? == Ordering::Equal).into())
		unimplemented!()
	}

	#[test]
	fn leq() {
		// Ok((cmp(args)? != Ordering::Greater).into())
		unimplemented!()
	}

	#[test]
	fn geq() {
		// Ok((cmp(args)? != Ordering::Less).into())
		unimplemented!()
	}

	#[test]
	fn neq() {
		// Ok((cmp(args)? != Ordering::Equal).into())
		unimplemented!()
	}
}









#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Function;

mod impls {
	use crate::{Object, Result, ArgsOld};
	pub fn lsh(_args: ArgsOld) -> Result<Object> {
		todo!("<<")
	}

	pub fn rsh(_args: ArgsOld) -> Result<Object> {
		todo!(">>")
	}

	pub fn curry(_args: ArgsOld) -> Result<Object> {
		todo!("curry")
	}
}

impl_object_type!{
for Function [(parents super::Basic)]:
	"<<" => impls::lsh,
	">>" => impls::rsh,
	"curry" => impls::curry
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Function;

mod impls {
	use crate::obj::{Object, Result, Args, types};
	pub fn lsh(_args: Args) -> Result<Object> {
		todo!("<<")
	}

	pub fn rsh(_args: Args) -> Result<Object> {
		todo!(">>")
	}

	pub fn curry(_args: Args) -> Result<Object> {
		todo!("curry")
	}
}

impl_object_type_!{for Function, super::Basic;
	"<<"    => (impls::lsh),
	">>"    => (impls::rsh),
	"curry" => (impls::curry)
}

mod tests {
	use super::*;

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

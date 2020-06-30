use crate::{Object, Result, Args};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Function;

impl Function {
	pub fn qs_lsh(_this: &Object, _args: Args) -> Result<Object> {
		todo!("<<")
	}

	pub fn qs_rsh(_this: &Object, _args: Args) -> Result<Object> {
		todo!(">>")
	}

	pub fn qs_curry(_this: &Object, _args: Args) -> Result<Object> {
		todo!("curry")
	}
}

impl_object_type!{
for Function [(parents super::Basic)]:
	"<<" => function Function::qs_lsh,
	">>" => function Function::qs_rsh,
	"curry" => function Function::qs_curry
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

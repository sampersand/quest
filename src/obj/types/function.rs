#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Function;

impl_object_type!{for Function, super::Basic;
	"<<" => (|args| todo!("<<")),
	">>" => (|args| todo!(">>")),
	"curry" => (|args| todo!("curry"))
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scope;

mod impls {
	use crate::obj::{Object, Result, Args, types};
	pub fn at_text(args: Args) -> Result<Object> {
		let this = args.this()?;
		if let Ok(name) = this.get_attr("name") {
			Ok(name)
		} else {
			Ok("<unnamed scope>".into())
		}
	}

}

impl_object_type!{
for Scope [(parent super::Basic)]:
	"@text" => impls::at_text,
}
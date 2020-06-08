#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scope;

mod impls {
	use crate::{Object, Result, Args};

	pub fn at_text(args: Args) -> Result<Object> {
		let this = args.this()?;
		if let Ok(name) = this.get_attr("name") {
			Ok(name)
		} else {
			Ok("<unnamed scope>".into())
		}
	}

	pub fn super_(args: Args) -> Result<Object> {
		let this = args.this()?;
		let _attr = args.arg(0)?;
		let mut args = args.args(1..)?;
		args.add_this(this.clone());

		unimplemented!();
		// this.get_attr("__parent__")?
		// 	.get_attr("__parent__")?
		// 	.call_attr(attr, args)
	}

}

impl_object_type!{
for Scope [(parents super::Basic)]:
	"@text" => impls::at_text,
	"super" => impls::super_,
}
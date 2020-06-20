#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scope;

mod impls {
	use crate::{Object, Result, Args, types};

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
		let attr = args.arg(0)?;
		let mut args = args.args(1..)?;
		args.add_this(this.clone());

		let default_attr = this.get_attr(attr)?;

		for parent in this.get_attr("__parents__")?.downcast_call::<types::List>()?.as_ref().iter() {
			if parent.has_attr(attr)? {
				// TODO: this
				let attr = parent.get_attr(attr)?;
				if attr.is_identical(&default_attr) {
					continue;
				}
				return parent.get_attr(&attr)?.call_attr("()", args);
			}
		}

		Err(crate::Error::Messaged(
			format!("attr {:?} does not exist for {:?} or its parents", attr, this).into()))
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
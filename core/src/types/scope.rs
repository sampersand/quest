#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scope;

mod impls {
	use crate::{Object, Result, ArgsOld, types};

	pub fn at_text(args: ArgsOld) -> Result<Object> {
		let this = args.this()?;
		if let Ok(name) = this.get_attr_old("name") {
			Ok(name)
		} else {
			Ok("<unnamed scope>".into())
		}
	}

	pub fn super_(args: ArgsOld) -> Result<Object> {
		let this = args.this()?;
		let attr = args.arg(0)?;
		let mut args = args.args(1..)?;
		args.add_this(this.clone());

		let default_attr = this.get_attr_old(attr)?;

		for parent in this.get_attr_old("__parents__")?.downcast_call::<types::List>()?.as_ref().iter() {
			if parent.has_attr_old(attr)? {
				// TODO: this
				let attr = parent.get_attr_old(attr)?;
				if attr.is_identical(&default_attr) {
					continue;
				}
				return parent.get_attr_old(&attr)?.call_attr_old_old("()", args);
			}
		}

		Err(crate::Error::Messaged(
			format!("attr {:?} does not exist for {:?} or its parents", attr, this).into()))
		// this.get_attr_old("__parent__")?
		// 	.get_attr_old("__parent__")?
		// 	.call_attr_old_old(attr, args)
	}

}

impl_object_type!{
for Scope [(parents super::Basic)]:
	"@text" => impls::at_text,
	"super" => impls::super_,
}
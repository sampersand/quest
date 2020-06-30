use crate::{Object, Result, Args};
use crate::types::Text;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Scope;

impl Scope {
	pub fn qs_at_text(this: &Object, _: Args) -> crate::Result<Text> {
		if let Ok(name) = this.get_attr_lit("name") {
			name.downcast_call::<Text>()
		} else {
			Ok(Text::new_static("<unnamed scope>"))
		}
	}

	pub fn qs_super(_this: &Object, _args: Args) -> Result<Object> {
		// let attr = args.arg(0)?;
		// let mut args = args.args(1..)?;
		// args.add_this(this.clone());

		// let default_attr = this.get_attr_old(attr)?;

		// for parent in this.get_attr_old("__parents__")?.downcast_call::<List>()?.as_ref().iter() {
		// 	if parent.has_attr_old(attr)? {
		// 		// TODO: this
		// 		let attr = parent.get_attr_old(attr)?;
		// 		if attr.is_identical(&default_attr) {
		// 			continue;
		// 		}
		// 		return parent.get_attr_old(&attr)?.call_attr_old_old("()", args);
		// 	}
		// }

		todo!()
		// Err(crate::Error::Messaged(
			// format!("attr {:?} does not exist for {:?} or its parents", attr, this).into()))
		// this.get_attr_old("__parent__")?
		// 	.get_attr_old("__parent__")?
		// 	.call_attr_old_old(attr, args)
	}

}

impl_object_type!{
for Scope [(parents super::Basic)]:
	"@text" => function Scope::qs_at_text,
	"super" => function Scope::qs_super,
}
use crate::{Object, Result, Args};
use crate::types::Text;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Scope;

impl Scope {
	pub fn qs_at_text(this: &Object, _: Args) -> crate::Result<Object> {
		const UNNAMED_SCOPE: Text = Text::new_static("<unnamed scope>");

		Ok(this.get_attr_lit("name").unwrap_or_else(|_| UNNAMED_SCOPE.into()))
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
for Scope {
	#[inline]
	fn new_object(self) -> Object {
		// use lazy_static::lazy_static;
		// use crate::types::ObjectType;

		// lazy_static! {
			// static ref SCOPE: Object = Object::new_with_parent(Scope, vec![Scope::mapping()]);
		// }
		Object::new_with_parent(crate::types::Class::new("Scope"), vec![
			Scope::mapping(),
			crate::types::Kernel::mapping()
		])
		// SCOPE.deep_clone()
	}
}
[(init_parents super::Kernel super::Basic) (parents super::Basic)]:
	"@text" => function Self::qs_at_text,
	"super" => function Self::qs_super,
}

use crate::{Object, Result, Args};

use tracing::instrument;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Scope(String);

impl Scope {
	pub fn new(stackframe: String) -> Self {
		Self(stackframe)
	}
}

impl Scope {
	#[instrument(name="Scope::@text", level="trace", skip(this), fields(self=?this))]
	pub fn qs_at_text(this: &Object, _: Args) -> crate::Result<Object> {
		this.get_attr_lit("name")
			.or_else(|_| Ok(this.try_downcast::<Self>()?.0.clone().into()))
	}

	#[instrument(name="Scope::super", level="trace", skip(_this, _args), fields(self=?_this, args=?_args))]
	pub fn qs_super(_this: &Object, _args: Args) -> Result<Object> {
		// let attr = args.try_arg(0)?;
		// let mut args = argstry_.args(1..)?;
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
	"@text" => method Self::qs_at_text,
	"super" => method Self::qs_super,
	"=" => method |_, args| {
		Ok(crate::Binding::set_binding(args.try_arg(0)?.clone()).into())
	}
}

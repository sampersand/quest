use crate::{Object, Args};
use crate::types::{Text, Number, Boolean};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pristine;

impl Pristine {
	#[inline]
	#[allow(non_snake_case)]
	pub fn qs___id__(this: &Object, _: Args) -> Result<Number, !> {
		Ok(this.id().into())
	}

	#[inline]
	#[allow(non_snake_case)]
	pub fn qs___call_attr__(this: &Object, args: Args) -> crate::Result<Object> {
		let attr = args.arg(0)?;
		let rest = args.args(1..).unwrap_or_default();
		this.call_attr_old_old(attr, rest)
	}

	#[inline]
	#[allow(non_snake_case)]
	pub fn qs___get_attr__(this: &Object, args: Args) -> crate::Result<Object> {
		let attr = args.arg(0)?;
		this.get_attr(attr)
	}

	#[inline]
	#[allow(non_snake_case)]
	pub fn qs___set_attr__(this: &Object, args: Args) -> crate::Result<Object> {
		let attr = args.arg(0)?;
		let val = args.arg(1)?;
		this.set_attr(attr.clone(), val.clone())?;
		Ok(val.clone())
	}

	#[inline]
	#[allow(non_snake_case)]
	pub fn qs___has_attr__(this: &Object, args: Args) -> crate::Result<bool> {
		let attr = args.arg(0)?;
		this.has_attr(attr)
	}

	#[inline]
	#[allow(non_snake_case)]
	pub fn qs___del_attr__(this: &Object, args: Args) -> crate::Result<Object> {
		let attr = args.arg(0)?;
		this.del_attr(attr)
	}

	#[inline]
	pub fn qs_dot_get_attr(this: &Object, args: Args) -> crate::Result<Object> {
		let attr = args.arg(0)?;
		this.dot_get_attr(attr)
	}

	#[allow(non_snake_case)]
	pub fn qs___keys__(this: &Object, args: Args) -> crate::Result<Object> {
		let include_parents = args.arg(0)
			.ok()
			.and_then(|x| x.downcast_call::<Boolean>().ok())
			.map(bool::from)
			.unwrap_or(false);

		Ok(this.mapping_keys(include_parents)
			.into_iter()
			.map(Object::from)
			.collect::<Vec<_>>()
			.into())
	}
}

impl Pristine {
	
	#[allow(non_snake_case)]
	pub fn qs___inspect__(this: &Object, _: Args) -> Result<Text, !> {
		Ok(format!("<{}:{}>", this.typename(), this.id()).into())
	}
}

impl_object_type!{
for Pristine [(init_parent) (parents Pristine)]:
	"__id__" => function Pristine::qs___id__,
	"__keys__" => function Pristine::qs___keys__,
	"__call_attr__" => function Pristine::qs___call_attr__,
	"__get_attr__" => function Pristine::qs___get_attr__,
	"__set_attr__" => function Pristine::qs___set_attr__,
	"__has_attr__" => function Pristine::qs___has_attr__,
	"__del_attr__" => function Pristine::qs___del_attr__,
	"__inspect__" => function Pristine::qs___inspect__,
	"::" => function Pristine::qs___get_attr__,
	"." => function Pristine::qs_dot_get_attr,
	".=" => function Pristine::qs___set_attr__
}




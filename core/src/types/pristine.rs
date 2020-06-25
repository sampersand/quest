#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pristine;

mod impls {
	use crate::{Object, Result, Args, types};

	pub fn __id__(args: Args) -> Result<Object> {
		let this = args.this()?;
		Ok(types::Number::from(this.id()).into())
	}

	pub fn __call_attr__(args: Args) -> Result<Object> {
		let this = args.this()?;
		let attr = args.arg(0)?;
		let rest = args.args(1..).unwrap_or_default();
		this.call_attr(attr, rest)
	}

	pub fn __get_attr__(args: Args) -> Result<Object> {
		let this = args.this()?;
		let attr = args.arg(0)?;
		this.get_attr(attr)
	}

	pub fn __set_attr__(args: Args) -> Result<Object> {
		let this = args.this()?;
		let attr = args.arg(0)?;
		let val = args.arg(1)?;
		this.set_attr_possibly_parents(attr.clone(), val.clone())
	}

	pub fn __has_attr__(args: Args) -> Result<Object> {
		let this = args.this()?;
		let attr = args.arg(0)?;
		this.has_attr(attr).map(Into::into)
	}

	pub fn __del_attr__(args: Args) -> Result<Object> {
		let this = args.this()?;
		let attr = args.arg(0)?;
		this.del_attr(attr)
	}

	pub fn dot_get_attr(args: Args) -> Result<Object> {
		let this = args.this()?.clone();
		this.dot_get_attr(args.arg(0)?)
	}

	pub fn __keys__(args: Args) -> Result<Object> {
		let this = args.this()?;
		let include_parents = args.arg(0)
			.and_then(|x| x.downcast_call::<types::Boolean>())
			.map(bool::from)
			.unwrap_or(false);

		Ok(this.mapping_keys(include_parents)
			.into_iter()
			.map(Object::from)
			.collect::<Vec<_>>()
			.into())
	}
}

impl_object_type!{
for Pristine [(init_parent) (parents Pristine)]:
	"__id__" => impls::__id__,
	"__keys__" => impls::__keys__,
	"__call_attr__" => impls::__call_attr__,
	"__get_attr__" => impls::__get_attr__,
	"__set_attr__" => impls::__set_attr__,
	"__has_attr__" => impls::__has_attr__,
	"__del_attr__" => impls::__del_attr__,
	"::" => impls::__get_attr__,
	"." => impls::dot_get_attr,
	".=" => impls::__set_attr__
}




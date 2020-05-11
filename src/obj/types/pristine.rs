#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pristine;

impl_object_type!{for Pristine, Pristine,; // trailing comma here is required 
	"__id__" => (|args| {
		Ok(Number::from(args.get(0)?.id()).into())
	}),

	"__call_attr__" => (|args| {
		args.get(0)?.call_attr(args.get(1)?, args.get_rng(2..).unwrap_or_default())
	}),

	"__get_attr__" => (|args| {
		args.get(0)?.get_attr(args.get(1)?)
	}),

	"__set_attr__" => (|args| {
		args.get(0)?.set_attr((*args.get(1)?).clone(), (*args.get(2)?).clone())
	}),

	"__del_attr__" => (|args| {
		args.get(0)?.del_attr(args.get(1)?)
	})
}
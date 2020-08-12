#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Class(&'static str);

impl Class {
	pub const fn new(name: &'static str) -> Self {
		Self(name)
	}
}

impl_object_type!{
for Class [(parents super::Basic) (no_convert)]:
}

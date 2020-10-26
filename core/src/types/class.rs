use crate::{Object, Args};
use crate::types::Text;
use tracing::instrument;

/// The base class for all "Classes" in quest.
///
/// This allows the classes to have helpful names when outputting messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Class(&'static str);

impl Class {
	/// Create a new [`Class`] with the given name.
	pub const fn new(name: &'static str) -> Self {
		Self(name)
	}
}

impl From<Class> for Text {
	fn from(class: Class) -> Self {
		Self::const_new(class.0)
	}
}

impl Class {
	/// Gets the name of this Class.
	#[instrument(name="Class::name", level="trace", skip(this), fields(self=?this))]
	pub fn qs_name(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.try_downcast::<Self>()?;
		Ok(Text::from(*this).into())
	}
}

impl_object_type!{
for Class [(parents super::Basic) (no_convert)]:
	"name" => method Class::qs_name
}

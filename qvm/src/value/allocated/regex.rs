use crate::value::NamedType;


/// Regular expressions within Quest.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct Regex {
	// todo
}

impl NamedType for Regex {
	const TYPENAME: &'static str = "Regex";
}

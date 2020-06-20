use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub enum ValueError {
	BadValue { expected: String, got: String },
	Messaged(String)
}

impl From<ValueError> for super::Error {
	fn from(key_error: ValueError) -> Self {
		Self::ValueError(key_error)
	}
}

impl Display for ValueError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "type error: ")?;
		match self {
			ValueError::BadValue { expected, got } => 
				write!(f, "expected type '{}' but got type '{}'", expected, got),
			ValueError::Messaged(msg) => Display::fmt(&msg, f),
		}
	}
}

use std::fmt::{self, Debug, Display, Formatter};
use std::convert::TryFrom;

pub type Integer = i64;
pub type Rational = f64;

#[derive(Clone, Copy, PartialEq)]
pub enum NumberType {
	Integer(Integer),
	Rational(Rational)
}

impl Eq for NumberType {}

impl Debug for NumberType {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			NumberType::Integer(n) => Debug::fmt(&n, f),
			NumberType::Rational(n) => Debug::fmt(&n, f)
		}
	}
}

impl Display for NumberType {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			NumberType::Integer(n) => Display::fmt(&n, f),
			NumberType::Rational(n) => Display::fmt(&n, f)
		}
	}
}

// impl TryFrom<Rational> for NumberType {

// }






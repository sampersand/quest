use crate::{Literal, Value};
use std::collections::HashMap;

/// A [`Literal`] map.
#[derive(Debug, Default)]
pub struct LMap {
	map: HashMap<Literal, Value>
}

impl LMap {
	pub fn new() -> Self {
		Self::default()
	}
}



use crate::eval::ByteCode;
use crate::Value;

#[derive(Debug, Clone)]
pub struct Block {
	source_location: &'static str,
	data: Box<[u8]>
}


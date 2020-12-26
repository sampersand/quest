use crate::eval::Block;
use crate::Value;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Frame {
	block: Block,
	idx: usize,
	stack: Vec<Value>,
	parent: Option<Arc<Frame>>
}

impl super::ByteCode {

}

use crate::Value;

#[derive(Debug, Clone)]
pub enum ByteCode {
	PushValue(Value),
	Pop,
	GetAttr,
	SetAttr,
	DelAttr,
	CallAttr(usize),
}

impl ByteCode {
	fn run(&self, stack: &mut Vec<Value>) {
		match self {
			Self::PushValue(value) => stack.push(value),
			Self::Pop => stack.pop().expect("Popped from an empty stack!"),
			SElf::GetAttr => 
		}
	}
}

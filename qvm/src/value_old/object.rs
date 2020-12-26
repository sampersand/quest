use crate::value::Basic;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Object {
	basic: Basic
}

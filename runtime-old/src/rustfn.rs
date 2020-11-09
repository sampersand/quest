use std::borrow::Cow;
use crate::Value;

type Inner = for<'a, 'b, 'c> fn(Cow<'a, Value>, &'b [Cow<'c, Value>]) -> crate::Result<Cow<'a, Value>>;

#[derive(Debug, Clone, Copy)]
pub struct RustFn(u8);



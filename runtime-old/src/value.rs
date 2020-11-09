#[cfg(feature = "nanbox")]
mod nanbox;
#[cfg(feature = "nanbox")]
pub use nanbox::Value;

// sa::assert_impl_all!(Value: std::fmt::Debug, Clone, PartialEq, Default);
// macro_rules! assert_from_tryfrom {
// 	($($type:ident),+) => {
// 		sa::assert_impl_all!(Value: $(From<quest_core::types::$type>),+);
// 		$(
// 			sa::assert_impl_all!(quest_core::types::$type: std::convert::TryFrom<Value>);
// 		)*
// 	};
// }

// assert_from_tryfrom!(Null, Boolean, Number, Text);

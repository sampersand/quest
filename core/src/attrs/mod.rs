//! Traits for methods within quest

/// Converting from one quest type to another
mod convert;

/// Miscellaneous extra traits
mod misc;

/// Operators from quest
mod operators;

pub use self::convert::*;
pub use self::misc::*;
pub use self::operators::*;
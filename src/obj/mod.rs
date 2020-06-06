mod obj;
mod result;
pub mod traits;
pub mod mapping;

pub mod literals;
pub mod types;

use self::mapping::Mapping;
pub use self::traits::*;
pub use self::obj::Object;
pub use self::types::rustfn::{Args, Binding};
pub use self::result::Result;
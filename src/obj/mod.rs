mod obj;
mod mapping;
mod result;

pub mod types;

use self::mapping::Mapping;
pub use self::obj::Object;
pub use self::types::rustfn::{Args, Binding};
pub use self::result::Result;
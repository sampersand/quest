#[macro_use]
mod macros;
mod convert;
mod negative_slice_index;
mod indexer;

pub use convert::Convertible;
pub use indexer::{Indexer, IndexResult};
pub use negative_slice_index::{NegativeSliceIndex, SliceIndexVerify};

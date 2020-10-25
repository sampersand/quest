//! Shared utilities across quest code.

mod slice_index;

pub use slice_index::SliceIndex;

/// Attempts to clone a resource, which can possibly fail.
pub trait TryClone : Sized {
	type Err;

	fn try_clone(&self) -> Result<Self, Self::Err>;
}

impl<T: Clone> TryClone for T {
	type Err = std::convert::Infallible;

	fn try_clone(&self) -> Result<Self, Self::Err> {
		Ok(self.clone())
	}
}

/// Hash  an object by including its type id first.
pub fn hash<T: std::hash::Hash + 'static>(data: &T) -> u64 {
	use std::collections::hash_map::DefaultHasher;
	use std::hash::{Hash, Hasher};
	use std::any::TypeId;

	let hasher = &mut DefaultHasher::new();

	TypeId::of::<T>().hash(hasher);
	data.hash(hasher);

	hasher.finish()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndexError {
	TooPositive,
	TooNegative
}

pub fn correct_index(idx: isize, len: usize) -> Result<usize, IndexError> {
	if idx.is_negative() {
		let idx = !idx as usize + 1;
		if idx <= len {
			Ok(len - idx)
		} else {
			Err(IndexError::TooNegative)
		}
	} else if (idx as usize) < len {
		Ok(idx as usize)
	} else {
		Err(IndexError::TooPositive)
	}
}

#[test]
fn test_correct_index() {
	assert_eq!(correct_index(2, 0), Err(IndexError::TooPositive));
	assert_eq!(correct_index(2, 1), Err(IndexError::TooPositive));
	assert_eq!(correct_index(2, 2), Err(IndexError::TooPositive));

	assert_eq!(correct_index(1, 0), Err(IndexError::TooPositive));
	assert_eq!(correct_index(1, 1), Err(IndexError::TooPositive));
	assert_eq!(correct_index(1, 2), Ok(1));

	assert_eq!(correct_index(0, 0), Err(IndexError::TooPositive));
	assert_eq!(correct_index(0, 1), Ok(0));
	assert_eq!(correct_index(0, 2), Ok(0));

	assert_eq!(correct_index(-1, 0), Err(IndexError::TooNegative));
	assert_eq!(correct_index(-1, 1), Ok(0));
	assert_eq!(correct_index(-1, 2), Ok(1));

	assert_eq!(correct_index(-2, 0), Err(IndexError::TooNegative));
	assert_eq!(correct_index(-2, 1), Err(IndexError::TooNegative));
	assert_eq!(correct_index(-2, 2), Ok(0));

	assert_eq!(correct_index(-3, 0), Err(IndexError::TooNegative));
	assert_eq!(correct_index(-3, 1), Err(IndexError::TooNegative));
	assert_eq!(correct_index(-3, 2), Err(IndexError::TooNegative));
}

#[test]
fn test_correct_index_edge_cases() {
	assert_eq!(correct_index(0, usize::MAX), Ok(0));
	assert_eq!(correct_index(-1, usize::MAX), Ok(usize::MAX - 1));
	assert_eq!(correct_index(isize::MAX, usize::MAX), Ok(isize::MAX as usize));
	assert_eq!(correct_index(isize::MIN, usize::MAX), Ok(usize::MAX - (!isize::MIN as usize + 1)));
}

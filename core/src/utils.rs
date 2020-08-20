//! Shared utilities across quest code.

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

pub fn correct_index(idx: isize, len: usize) -> Option<usize> {
	if !idx.is_negative() {
		if (idx as usize) < len {
			Some(idx as usize)
		} else {
			None
		}
	} else {
		let idx = (-idx) as usize;
		if idx <= len {
			Some(len - idx)
		} else {
			None
		}
	}
}


pub fn hash<T: std::hash::Hash + 'static>(data: &T) -> u64 {
	use std::collections::hash_map::DefaultHasher;
	use std::hash::{Hash, Hasher};
	use std::any::TypeId;

	let ref mut hasher = DefaultHasher::new();

	TypeId::of::<T>().hash(hasher);
	data.hash(hasher);

	hasher.finish()
}

pub fn correct_index(index: isize, len: usize) -> Option<usize> {
	if !index.is_negative() {
		if (index as usize) < len {
			Some(index as usize)
		} else {
			None
		}
	} else {
		let index = (-index) as usize;
		if index <= len {
			Some(len - index)
		} else {
			None
		}
	}
}
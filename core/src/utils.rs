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
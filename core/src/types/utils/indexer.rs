use std::cmp::Ordering;

enum OffsetResult {
	InBounds(usize),
	NegativeIndexInvalid,
	NotInBounds(usize)
}
fn offset(idx: isize, len: usize) -> OffsetResult {
	if idx.is_negative() {
		let idx = idx.abs() as usize;
		if idx < len {
			OffsetResult::InBounds(len - idx)
		} else {
			OffsetResult::NegativeIndexInvalid
		}
	} else {
		if (idx as usize) < len {
			OffsetResult::InBounds(idx as usize)
		} else {
			OffsetResult::NotInBounds(idx as usize)
		}
	}
}


pub enum IndexResult<T> {
	Ok,
	InvalidRange(T),
	BadInput(T)
}

pub trait Indexer<'a, T: 'a + ?Sized + ToOwned> : Clone {
	type Output: 'a;
	type Input: 'a;
	fn get(self, data: &'a T) -> Option<Self::Output>;
	fn set(self, data: &'a mut <T as ToOwned>::Owned, input: Self::Input)
		-> IndexResult<Self::Input>;
}

impl<'a, T: 'a + Clone + Default> Indexer<'a, [T]> for isize {
	type Output = &'a T;
	type Input = T;

	fn get(self, data: &'a [T]) -> Option<Self::Output> {
		match offset(self, data.len()) {
			OffsetResult::InBounds(idx) => Some(&data[idx]),
			_ => None
		}
	}

	fn set(self, data: &'a mut Vec<T>, input: Self::Input) -> IndexResult<Self::Input> {
		let index = 
			match offset(self, data.len()) {
				OffsetResult::InBounds(index) | OffsetResult::NotInBounds(index) => index,
				_ => return IndexResult::InvalidRange(input)
			};

		let ord = index.cmp(&data.len());

		// if we're greater than resize all the way up to the length but the last one, so we can
		// push the element in at the end.
		if ord == Ordering::Greater {
			data.resize_with(index, Default::default);
		}

		if ord == Ordering::Less {
			data[index] = input;
		} else {
			data.push(input);
		}

		IndexResult::Ok
	}
}


impl<'a, T: 'a + Clone + Default + std::fmt::Debug> Indexer<'a, [T]> for std::ops::Range<isize> {
	type Output = &'a [T];
	type Input = Vec<T>;

	fn get(self, data: &'a [T]) -> Option<Self::Output> {
		match (offset(self.start, data.len()), offset(self.end, data.len())) {
			(OffsetResult::InBounds(start), OffsetResult::InBounds(end)) => Some(&data[start..end]),
			_ => None
		}
	}

	fn set(self, data: &'a mut <[T] as ToOwned>::Owned, mut input: Self::Input)
		-> IndexResult<Self::Input>
	{
		let start = 
			match offset(self.start, data.len()) {
				OffsetResult::InBounds(start) | OffsetResult::NotInBounds(start) => start,
				_ => return IndexResult::InvalidRange(input)
			};

		let end = 
			match offset(self.end, data.len()) {
				OffsetResult::InBounds(end) | OffsetResult::NotInBounds(end) => end,
				_ => return IndexResult::InvalidRange(input)
			};

		if data.len() < start {
			data.resize_with(start, Default::default);
			data.extend(input);
		} else if (start..end).contains(&data.len()) {
			data.splice(start.., input.drain(..(data.len() - start).min(input.len())));
			data.extend(input);
		} else {
			data.splice(start..end, input.drain(..(end - start).min(input.len())));
		}

		IndexResult::Ok
	}
}

impl<'a> Indexer<'a, str> for isize {
	type Output = char;
	type Input = char;

	fn get(self, data: &'a str) -> Option<Self::Output> {
		match offset(self, data.len()) {
			OffsetResult::InBounds(idx) => data.chars().nth(idx),
			_ => None
		}
	}

	fn set(self, _data: &'a mut String, _input: Self::Input) -> IndexResult<Self::Input> {

		// let index = 
		// 	match offset(self, data.len()) {
		// 		OffsetResult::InBounds(index) => index,
		// 		OffsetResult::NotInBounds(index) if index == data.len() => index,
		// 		_=> return IndexResult::InvalidRange(input)
		// 	};

		// match index.cmp(&data.len()) {
		// 	Ordering::Greater => return IndexResult::InvalidRange(input),
		// 	Ordering::Equal => data.push(input),
		// 	Ordering::Less => data[index] = input
		// };

		IndexResult::Ok
	}
}


impl<'a> Indexer<'a, str> for std::ops::Range<isize> {
	type Output = &'a str;
	type Input = &'a str;

	fn get(self, data: &'a str) -> Option<Self::Output> {
		match (offset(self.start, data.len()), offset(self.end, data.len())) {
			(OffsetResult::InBounds(start), OffsetResult::InBounds(end)) => Some(&data[start..end]),
			_ => None
		}
	}

	fn set(self, _data: &'a mut String, mut _input: Self::Input)
		-> IndexResult<Self::Input>
	{
		// let start = 
		// 	match offset(self.start, data.len()) {
		// 		OffsetResult::InBounds(start) | OffsetResult::NotInBounds(start) => start,
		// 		OffsetResult::NegativeIndexInvalid => return IndexResult::InvalidRange(input)
		// 	};

		// let end = 
		// 	match offset(self.end, data.len()) {
		// 		OffsetResult::InBounds(end) | OffsetResult::NotInBounds(end) => end,
		// 		OffsetResult::NegativeIndexInvalid => return IndexResult::InvalidRange(input)
		// 	};

		// if data.len() < start {
		// 	data.resize_with(start, Default::default);
		// 	data.extend(input);
		// } else if (start..end).contains(&data.len()) {
		// 	data.splice(start.., input.drain(..(data.len() - start).min(input.len())));
		// 	data.extend(input);
		// } else {
		// 	data.splice(start..end, input.drain(..(end - start).min(input.len())));
		// }

		IndexResult::Ok
	}
}

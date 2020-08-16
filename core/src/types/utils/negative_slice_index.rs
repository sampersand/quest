use std::slice::SliceIndex;
use std::ops::Index;

fn offset(idx: isize, len: usize) -> Option<usize> {
	if idx.is_negative() {
		let idx = idx.abs() as usize;
		if idx < len {
			Some(len - idx)
		} else {
			None
		}
	} else {
		if (idx as usize) < len {
			Some(idx as usize)
		} else {
			None
		}
	}
}

// unsafe impl<T> SliceIndexVerify<[T]> for std::ops::RangeFull {}
// impl<T> NegativeSliceIndex<[T]> for std::ops::RangeFull {
// 	type Corrected = Self;

// 	#[inline]
// 	fn correct(self, _: usize) -> Option<Self::Corrected> {
// 		Some(self)
// 	}
// }

// macro_rules! impl_negative_slice_index {
// 	($len:ident $self:ident $($ty:ident($($arg:ident),*) $correct:block $($verify:block)?)*) => {
// 		$(
// 			impl<T> NegativeSliceIndex<[T]> for std::ops::$ty<isize> {
// 				type Corrected = std::ops::$ty<usize>;

// 				fn correct($self, $len: usize) -> Option<Self::Corrected> {
// 					$(let $arg = offset($self.$arg, $len)?;)*
// 					Some($correct)
// 				}
// 			}

// 			impl<T> NegativeSliceIndex<[T]> for std::ops::$ty<usize> {
// 				type Corrected = Self;

// 				#[inline]
// 				fn correct(self, _: usize) -> Option<Self::Corrected> {
// 					Some(self)
// 				}
// 			}

// 			unsafe impl<T> SliceIndexVerify<[T]> for std::ops::$ty<usize> {
// 				$(fn is_valid(&$self) -> bool $verify)?
// 			}
// 		)*
// 	};
// }

// impl_negative_slice_index! { len self
// 	Range(start, end) { start..end } { self.start <= self.end }
// 	RangeFrom(start) { start.. }
// 	RangeTo(end) { ..end }
// 	RangeInclusive() { offset(*self.start(), len)?..=offset(*self.end(), len)? }
// 		{ *self.start() <= *self.end() }
// 	RangeToInclusive(end) { ..=end } 
// }

////

pub unsafe trait SliceIndexVerify<T: ?Sized> : SliceIndex<T> {
	fn is_valid(&self) -> bool {
		true
	}
}

pub trait NegativeSliceIndex<T: ?Sized + Index<Self::Corrected>> : Sized {
	type Corrected: SliceIndexVerify<T>;

	fn correct(self, len: usize) -> Option<Self::Corrected>;
}

unsafe impl<T> SliceIndexVerify<[T]> for usize {}
impl<T> NegativeSliceIndex<[T]> for isize {
	type Corrected = usize;

	#[inline]
	fn correct(self, len: usize) -> Option<Self::Corrected> {
		offset(self, len)
	}
}

unsafe impl<T> SliceIndexVerify<[T]> for std::ops::RangeFull {}
impl<T> NegativeSliceIndex<[T]> for std::ops::RangeFull {
	type Corrected = Self;

	#[inline]
	fn correct(self, _: usize) -> Option<Self::Corrected> {
		Some(self)
	}
}

macro_rules! impl_negative_slice_index {
	($len:ident $self:ident $($ty:ident($($arg:ident),*) $correct:block $($verify:block)?)*) => {
		$(
			impl<T> NegativeSliceIndex<[T]> for std::ops::$ty<isize> {
				type Corrected = std::ops::$ty<usize>;

				fn correct($self, $len: usize) -> Option<Self::Corrected> {
					$(let $arg = offset($self.$arg, $len)?;)*
					Some($correct)
				}
			}

			impl<T> NegativeSliceIndex<[T]> for std::ops::$ty<usize> {
				type Corrected = Self;

				#[inline]
				fn correct(self, _: usize) -> Option<Self::Corrected> {
					Some(self)
				}
			}

			unsafe impl<T> SliceIndexVerify<[T]> for std::ops::$ty<usize> {
				$(fn is_valid(&$self) -> bool $verify)?
			}
		)*
	};
}

impl_negative_slice_index! { len self
	Range(start, end) { start..end } { self.start <= self.end }
	RangeFrom(start) { start.. }
	RangeTo(end) { ..end }
	RangeInclusive() { offset(*self.start(), len)?..=offset(*self.end(), len)? }
		{ *self.start() <= *self.end() }
	RangeToInclusive(end) { ..=end } 
}

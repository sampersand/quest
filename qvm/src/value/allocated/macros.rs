macro_rules! impl_allocated_type {
	(for $ty:ident) => { impl_allocated_type!(for $ty, $ty); };
	(for $ty:ty, $variant:ident) => {
		unsafe impl $crate::value::allocated::AllocatedType for $ty {
			fn into_alloc(self) -> $crate::value::allocated::Allocated {
				$crate::value::allocated::Allocated::new($crate::value::allocated::AllocType::$variant(self))
			}

			fn is_alloc_a(alloc: &$crate::value::allocated::Allocated) -> bool {
				matches!(alloc.inner().data, $crate::value::allocated::AllocType::$variant(_))
			}

			unsafe fn alloc_as_ref_unchecked(alloc: &$crate::value::allocated::Allocated) -> &Self {
				debug_assert!(Self::is_alloc_a(alloc), "invalid value given: {:#?}", alloc);

				if let $crate::value::allocated::AllocType::$variant(ref data) = alloc.inner().data {
					data
				} else {
					std::hint::unreachable_unchecked()
				}
			}

			unsafe fn alloc_as_mut_unchecked(alloc: &mut $crate::value::allocated::Allocated) -> &mut Self {
				debug_assert!(Self::is_alloc_a(alloc), "invalid value given: {:#?}", alloc);

				if let $crate::value::allocated::AllocType::$variant(ref mut data) = alloc.inner_mut().data {
					data
				} else {
					std::hint::unreachable_unchecked()
				}
			}
		}
	};

}
macro_rules! impl_allocated_value_type_ref {
	(for $ty:ident) => { impl_allocated_value_type_ref!(for $ty, $ty); };
	(for $ty:ty, $variant:ident) => {
		unsafe impl $crate::value::ValueTypeRef for $ty {
			#[inline]
			unsafe fn value_as_ref_unchecked(value: &$crate::value::Value) -> &Self {
				debug_assert!(value.is_a::<Self>(), "invalid value given: {:?}", value);
				let mut allocated = $crate::value::allocated::Allocated::value_into_unchecked(*value);

				match (*allocated.0.as_ptr()).data {
					$crate::value::allocated::AllocType::$variant(ref data) => data,
					#[cfg(debug_assertions)]
					ref other => unreachable!("`is_a` and `value_into_unchecked` do not match up? {:?}", other),
					#[cfg(not(debug_assertions))]
					_ => std::hint::unreachable_unchecked()
				}
			}

			#[inline]
			unsafe fn value_as_mut_unchecked<'a>(value: &'a mut $crate::value::Value) -> &'a mut Self {
				debug_assert!(value.is_a::<Self>(), "invalid value given: {:?}", value);
				let mut allocated = $crate::value::allocated::Allocated::value_into_unchecked(*value);

				match (*allocated.0.as_ptr()).data {
					$crate::value::allocated::AllocType::$variant(ref mut data) => data,
					#[cfg(debug_assertions)]
					ref other => unreachable!("`is_a` and `value_into_unchecked` do not match up? {:?}", other),
					#[cfg(not(debug_assertions))]
					_ => std::hint::unreachable_unchecked()
				}
			}
		}
	}
}

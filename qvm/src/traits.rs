/// Indicates the ability for a type to be shallowly copied
pub trait ShallowClone : Sized {
	fn shallow_clone(&self) -> crate::Result<Self>;
}

/// Indicates the ability for a type to be deeply copied
pub trait DeepClone : Sized {
	/// Copies the actual data of the object.
	///
	/// When you [`clone()`] a [`Value`], you're actually just creating another reference to the
	/// same object in memory. This actually creates another distinct object.
	fn deep_clone(&self) -> crate::Result<Self>;
}

pub trait TryPartialEq {
	fn try_eq(&self, rhs: &Self) -> crate::Result<bool>;
}

impl<T: PartialEq> TryPartialEq for T {
	fn try_eq(&self, rhs: &Self) -> crate::Result<bool> {
		Ok(self == rhs)
	}
}

impl<T: ShallowClone> ShallowClone for Option<T> {
	fn shallow_clone(&self) -> crate::Result<Self> {
		self.as_ref().map(T::shallow_clone).transpose()
	}
}

impl<T: DeepClone> DeepClone for Option<T> {
	fn deep_clone(&self) -> crate::Result<Self> {
		self.as_ref().map(T::deep_clone).transpose()
	}
}

impl<T: ShallowClone> ShallowClone for Vec<T> {
	fn shallow_clone(&self) -> crate::Result<Self> {
		self.iter().map(T::shallow_clone).collect()
	}
}

impl<T: DeepClone> DeepClone for Vec<T> {
	fn deep_clone(&self) -> crate::Result<Self> {
		self.iter().map(T::deep_clone).collect()
	}
}


// impl<K: Clone, T: ShallowClone> ShallowClone for std::collections::HashMap<K, T> {
// 	fn shallow_clone(&self) -> crate::Result<Self> {
// 		self.iter().map(T::shallow_clone).collect()
// 	}
// }

// // impl<T: DeepClone> DeepClone for Vec<T> {
// // 	fn deep_clone(&self) -> crate::Result<Self> {
// // 		self.iter().map(T::deep_clone).collect()
// // 	}
// // }

macro_rules! impl_primitive_clone {
	($($ty:ident)*) => {
		$(impl ShallowClone for $ty { fn shallow_clone(&self) -> crate::Result<Self> { Ok(*self) }})*
		$(impl DeepClone for $ty { fn deep_clone(&self) -> crate::Result<Self> { Ok(*self) }})*
	};
}

impl_primitive_clone!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64 bool char);

// temporarily pub
pub(super) mod heap_only;
mod nanbox;

#[derive(Debug, Clone)]
pub enum ObjectRepr {
	NanBox(nanbox::NanBox),
	HeapOnly(heap_only::HeapOnly)
}


impl ObjectRepr {
	#[inline]
	pub(super) fn from_parts(data: heap_only::Data, attrs: heap_only::Attributes) -> Self {
		Self::HeapOnly(heap_only::HeapOnly::new(data, attrs))
	}
}

macro_rules! delegate {
	($(
		fn $name:ident$([$($generic:tt)*] )?(&$($lt:lifetime)? self $(, $arg:ident: $argty:ty)*) -> $ret_ty:ty;
	)*) => {
		impl ObjectRepr {
			$(
				pub fn $name<$($($generic)*)?>(&$($lt)? self $(,$arg: $argty)*) -> $ret_ty {
					match self {
						Self::HeapOnly(heap) => heap.$name($($arg),*),
						Self::NanBox(_) => unimplemented!(stringify!($name))
					}
				}
			)*
		}
	};
}

impl ObjectRepr {
	pub fn is_a<'a, T: crate::types::ObjectType + 'a>(&self) -> bool {
		match self {
			Self::HeapOnly(heap) => heap.is_a::<T>(),
			Self::NanBox(_) => unimplemented!()
		}
	}

	pub fn is_identical(&self, rhs: &Self) -> bool {
		match (self, rhs) {
			(Self::HeapOnly(heap), Self::HeapOnly(rhs)) => heap.is_identical(rhs),
			_ => unimplemented!()
		}
	}

	pub fn deep_clone(&self) -> Self {
		match self {
			Self::HeapOnly(heap) => Self::HeapOnly(heap.deep_clone()),
			_ => unimplemented!()
		}
	}
}

delegate! {
	fn id(&self) -> usize;
	fn typename(&self) -> &'static str;
	fn downcast['a, T: crate::types::ObjectType](&'a self) -> Option<impl std::ops::Deref<Target=T> + 'a>;
	fn downcast_mut['a, T: crate::types::ObjectType](&'a self) -> Option<impl std::ops::DerefMut<Target=T> + 'a>;

	fn has_lit(&self, attr: &str) -> crate::Result<bool>;
	fn get_lit(&self, attr: &str) -> crate::Result<Option<heap_only::Value>>;
	fn set_lit(&self, attr: crate::Literal, value: heap_only::Value) -> crate::Result<()>;
	fn del_lit(&self, attr: &str) -> crate::Result<Option<heap_only::Value>>;
	fn has(&self, attr: &crate::Object) -> crate::Result<bool>;
	fn get(&self, attr: &crate::Object) -> crate::Result<Option<heap_only::Value>>;
	fn set(&self, attr: crate::Object, value: heap_only::Value) -> crate::Result<()>;
	fn del(&self, attr: &crate::Object) -> crate::Result<Option<heap_only::Value>>;
	fn add_parent(&self, val: crate::Object) -> crate::Result<()>;
	fn keys(&self, include_parents: bool) -> crate::Result<Vec<crate::Object>>;
}

// 	pub fn id(&self) -> usize {
// 		match self {
// 			ObjectRepr::HeapOnly(heap) => heap.id()
// 		}
// 	}

// 	pub fn typename(&self) -> &'static str {
// 		match self {
// 			ObjectRepr::HeapOnly(heap) => heap.typename()
// 		}
// 	}
// }

// temporarily pub
pub(super) mod heapdata;
mod nanbox;

pub use heapdata::HeapData;
pub use nanbox::NaNBox;

#[derive(Debug, Clone)]
pub enum ObjectRepr {
	NaNBox(nanbox::NaNBox),
	HeapData(heapdata::HeapData)
}


impl ObjectRepr {
	#[inline]
	pub(super) fn from_parts(data: heapdata::Data, attrs: heapdata::Attributes) -> Self {
		Self::HeapData(heapdata::HeapData::new_with_parents(data, attrs))
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
						Self::HeapData(heap) => heap.$name($($arg),*),
						Self::NaNBox(_) => unimplemented!(stringify!($name))
					}
				}
			)*
		}
	};
}

impl ObjectRepr {
	pub fn is_a<'a, T: crate::types::ObjectType + 'a>(&self) -> bool {
		match self {
			Self::HeapData(heap) => heap.is_a::<T>(),
			Self::NaNBox(nanbox) => unimplemented!("{:?}", nanbox)//nanbox.is_a::<T>()
		}
	}

	pub fn is_identical(&self, rhs: &Self) -> bool {
		match (self, rhs) {
			(Self::HeapData(heap), Self::HeapData(rhs)) => heap.is_identical(rhs),
			(Self::NaNBox(nanbox), Self::NaNBox(rhs)) => unimplemented!("{:?} {:?}", nanbox, rhs),//nanbox.is_identical(rhs),
			_ => false
		}
	}

	pub fn deep_clone(&self) -> Self {
		match self {
			Self::HeapData(heap) => Self::HeapData(heap.deep_clone()),
			Self::NaNBox(nanbox) => unimplemented!("{:?}", nanbox),//Self::NaNBox(nanbox.deep_clone()),
		}
	}
}

delegate! {
	fn id(&self) -> usize;
	fn typename(&self) -> &'static str;
	fn downcast['a, T: crate::types::ObjectType](&'a self) -> Option<impl std::ops::Deref<Target=T> + 'a>;
	fn downcast_mut['a, T: crate::types::ObjectType](&'a self) -> Option<impl std::ops::DerefMut<Target=T> + 'a>;

	fn has_lit(&self, attr: &str) -> crate::Result<bool>;
	fn get_lit(&self, attr: &str) -> crate::Result<Option<heapdata::Value>>;
	fn set_lit(&self, attr: crate::Literal, value: heapdata::Value) -> crate::Result<()>;
	fn del_lit(&self, attr: &str) -> crate::Result<Option<heapdata::Value>>;
	fn has(&self, attr: &crate::Object) -> crate::Result<bool>;
	fn get(&self, attr: &crate::Object) -> crate::Result<Option<heapdata::Value>>;
	fn set(&self, attr: crate::Object, value: heapdata::Value) -> crate::Result<()>;
	fn del(&self, attr: &crate::Object) -> crate::Result<Option<heapdata::Value>>;
	fn add_parent(&self, val: crate::Object) -> crate::Result<()>;
	fn keys(&self, include_parents: bool) -> crate::Result<Vec<crate::Object>>;
}

// 	pub fn id(&self) -> usize {
// 		match self {
// 			ObjectRepr::HeapData(heap) => heap.id()
// 		}
// 	}

// 	pub fn typename(&self) -> &'static str {
// 		match self {
// 			ObjectRepr::HeapData(heap) => heap.typename()
// 		}
// 	}
// }

use super::{AllocatedType, Allocated};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class {
	name: &'static str
}

impl Class {
	pub fn new(name: &'static str) -> Self {
		Self { name }
	}
}

unsafe impl AllocatedType for Class {
	fn into_alloc(self) -> Allocated {
		todo!()
		// Allocated {
		// 	flags: super::FLAG_INSTANCE_CLASS,
		// 	data: super::Data {
		// 		class: std::mem::ManuallyDrop::new(self)
		// 	}
		// }
	}

	fn is_alloc_a(alloc: &Allocated) -> bool {
		todo!()
	/*
		Object::try_alloc_as_ref(alloc).map_or(false, Object::is_a::<Self>)
	*/}

	unsafe fn alloc_into_unchecked(alloc: Allocated) -> Self {
		todo!()
	/*
		debug_assert!(Self::is_alloc_a(&alloc), "invalid value given: {:#?}", alloc);
		
		Object::alloc_into_unchecked(alloc).into_unchecked()
	*/}

	unsafe fn alloc_as_ref_unchecked(alloc: &Allocated) -> &Self {
		todo!()
	/*
		debug_assert!(Self::is_alloc_a(alloc), "invalid value given: {:#?}", alloc);
		
		Object::alloc_as_ref_unchecked(alloc).as_ref_unchecked()
	*/}

	unsafe fn alloc_as_mut_unchecked(alloc: &mut Allocated) -> &mut Self {
		todo!()
	/*
		debug_assert!(Self::is_alloc_a(alloc), "invalid value given: {:#?}", alloc);
		
		Object::alloc_as_mut_unchecked(alloc).as_mut_unchecked()
	*/}
}

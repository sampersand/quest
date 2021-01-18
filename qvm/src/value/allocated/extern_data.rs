use std::fmt::{self, Debug, Formatter};
use std::any::{Any, TypeId};
use try_traits::clone::TryClone;
use crate::{Value, Literal, ShallowClone, DeepClone};
use crate::value::NamedType;
use crate::value::allocated::{Allocated, AllocatedType};
use crate::lmap::LMap;

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub(crate) struct ExternData {
	parents: Vec<Value>,
	attrs: LMap,
	data: *mut (),
	vtable: *const VTable
}

#[derive(Clone, Copy)]
#[doc(hidden)]
pub struct VTable {
	type_id: fn() -> TypeId,

	// SAFETY: the pointer must be a valid pointer to a type of `type_id`.
	drop: Option<unsafe fn(*mut ())>,

	// SAFETY: the pointer must be a valid pointer to a type of `type_id`.
	try_clone: unsafe fn(*const ()) -> crate::Result<*mut ()>,

	// SAFETY: the pointer must be a valid pointer to a type of `type_id`.
	debug: for<'b> unsafe fn(*const (), &mut Formatter<'b>) -> fmt::Result
}

#[inline]
fn allocate<T>(data: T) -> *mut () {
	Box::into_raw(Box::new(data)) as *mut ()
}

/// Data that's supplied by someone else's quest binndings.
pub trait ExternType : Debug + TryClone<Error=crate::Error> + NamedType + Any {
	/// The initial parents associated with some value.
	fn parents() -> Vec<Value> {
		use std::mem::MaybeUninit;
		use std::sync::Once;

		static mut PARENTS: MaybeUninit<Value> = MaybeUninit::uninit();
		static ONCE: Once = Once::new();

		// SAFETY: Since we only call this once, we can be guaranteed that (a) we won't have leaks and (b) we won't won't 
		// initialize it twice. Additionally, we know the pointer returned from `PARENTS.as_mut_ptr` is always valid.
		ONCE.call_once(|| unsafe {
			PARENTS.as_mut_ptr().write(Value::new(super::Class::new(Self::typename())));
		});

		// SAFETY: We know that it's initialized, as the `call_once` was run before we get here.
		unsafe {
			vec![(*PARENTS.as_ptr()).try_clone().expect("unable to clone parent!")]
		}
	}

	#[doc(hidden)]
	const _VTABLE: &'static VTable = {
		// SAFETY: pointers passed to this must be valid `T`.
		unsafe fn _drop<T>(ptr: *mut ()) {
			std::ptr::drop_in_place(ptr as *mut T)
		}

		// SAFETY: pointers passed to this must be valid `T`.
		unsafe fn _try_clone<T: TryClone<Error=crate::Error>>(ptr: *const ()) -> crate::Result<*mut ()> {
			(&*(ptr as *const T)).try_clone().map(allocate)
		}

		// SAFETY: pointers passed to this must be valid `T`.
		unsafe fn _debug<T: Debug>(ptr: *const (), f: &mut Formatter) -> fmt::Result {
			Debug::fmt(&*(ptr as *const T), f)
		}

		&VTable {
			type_id: TypeId::of::<Self>, 
			drop:  if std::mem::needs_drop::<Self>() { Some(_drop::<Self>) } else { None },
			try_clone: _try_clone::<Self>,
			debug: _debug::<Self>
		}
	};
}

impl Debug for ExternData {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		struct DataDebugger<'a>(&'a ExternData);

		impl<'a> Debug for DataDebugger<'a> {
			fn fmt(&self, f: &mut Formatter) -> fmt::Result {
				// SAFETY: `data`'s type never changes, so calling this is valid.
				unsafe {
					((*self.0.vtable).debug)(self.0.data, f)
				}
			}
		}

		struct ParentsDebugger<'a>(&'a [Value]);

		impl<'a> Debug for ParentsDebugger<'a> {
			fn fmt(&self, f: &mut Formatter) -> fmt::Result {
				write!(f, "[")?;

				if !self.0.is_empty() {
					write!(f, "{:?}", self.0[0])?;

					for parent in &self.0[1..] {
						write!(f, ", {:?}", self.0)?;
					}
				}

				write!(f, "]")
			}
		}

		if f.alternate() {
			f.debug_tuple("ExternData").field(&DataDebugger(self)).finish()
		} else {
			f.debug_struct("ExternData")
				.field("data", &DataDebugger(self))
				.field("parents", &ParentsDebugger(&self.parents))
				.field("attrs", &self.attrs)
				.finish()
		}
	}
}

impl Drop for ExternData {
	fn drop(&mut self) {
		// SAFETY: `data`'s type never changes, so calling this is valid.
		unsafe {
			if let Some(dropfn) = (*self.vtable).drop {
				(dropfn)(self.data)
			}
		}
	}
}


impl ExternData {
	pub fn new<T: ExternType>(data: T) -> Self {
		Self {
			parents: T::parents(),
			attrs: LMap::default(),
			data: allocate(data),
			vtable: T::_VTABLE
		}
	}

	pub fn typename(&self) -> &'static str {
		todo!()
	}

	pub fn has_attr(&self, attr: Literal) -> bool {
		self.attrs.has(attr)
	}

	pub fn call_attr(&self, attr: Literal, args: &[&Value]) -> Value {
		todo!()
	}

	pub fn get_attr(&self, attr: Literal) -> Option<&Value> {
		self.attrs.get(attr)
	}

	pub fn get_attr_mut(&mut self, attr: Literal) -> Option<&mut Value> {
		self.attrs.get_mut(attr)
	}

	pub fn set_attr(&mut self, attr: Literal, value: Value) {
		self.attrs.set(attr, value);
	}

	pub fn del_attr(&mut self, attr: Literal) -> Option<Value> {
		self.attrs.del(attr)
	}

	pub fn is_a<T: 'static>(&self) -> bool {
		TypeId::of::<T>() == self.vtable.type_id()
	}

	pub fn try_into<T: 'static>(self) -> Result<T, Self> {
		if self.is_a::<T>() {
			// SAFETY: We just verified `self` is a `T`.
			Ok(unsafe { self.into_unchecked() })
		} else {
			Err(self)
		}
	}

	#[inline]
	pub unsafe fn into_unchecked<T: 'static>(self) -> T {
		debug_assert!(self.is_a::<T>(), "invalid value given: {:?}", self);

		// SAFETY: it's up to the caller to ensure that `ptr` is a valid `T`.
		let data = unsafe {
			Box::from_raw(self.data as *mut T)
		};

		std::mem::forget(self);

		*data
	}

	pub fn try_as_ref<T: ExternType>(&self) -> Option<&T> {
		if self.is_a::<T>() {
			// SAFETY: We just verified `self` is a `T`.
			Some(unsafe { self.as_ref_unchecked() })
		} else {
			None
		}
	}

	#[inline]
	pub unsafe fn as_ref_unchecked<T: ExternType>(&self) -> &T {
		debug_assert!(self.is_a::<T>(), "invalid value given: {:?}", self);

		&*(self.data as *const T)
	}

	pub fn try_as_mut<T: ExternType>(&mut self) -> Option<&mut T> {
		if self.is_a::<T>() {
			// SAFETY: We just verified `self` is a `T`.
			Some(unsafe { self.as_mut_unchecked() })
		} else {
			None
		}
	}

	#[inline]
	pub unsafe fn as_mut_unchecked<T: ExternType>(&mut self) -> &mut T {
		debug_assert!(self.is_a::<T>(), "invalid value given: {:?}", self);

		&mut *(self.data as *mut T)
	}
}

impl ShallowClone for ExternData {
	fn shallow_clone(&self) -> crate::Result<Self> {
		Ok(Self {
			parents: self.parents.clone(),
			attrs: self.attrs.clone(), 
			data: unsafe { ((*self.vtable).try_clone)(self.data)? },
			vtable: self.vtable
		})
	}
}

impl DeepClone for ExternData {
	fn deep_clone(&self) -> crate::Result<Self> {
		// TODO: maybe require a `DeepClone` impl, so I can add it to the vtable?
		Ok(Self {
			parents: self.parents.clone(),
			attrs: self.attrs.clone(), 
			data: unsafe { ((*self.vtable).try_clone)(self.data)? },
			vtable: self.vtable
		})
	}
}

unsafe impl AllocatedType for ExternData {
	fn into_alloc(self) -> Allocated {
		// FLAG_INSTANCE_OBJECT
		// Allocated::new(ExternData::new(self))
		todo!()
	}

	fn is_alloc_a(alloc: &Allocated) -> bool {
		todo!()
	/*
		ExternData::try_alloc_as_ref(alloc).map_or(false, ExternData::is_a::<Self>)
	*/}

	unsafe fn alloc_into_unchecked(alloc: Allocated) -> Self {
		todo!()
	/*
		debug_assert!(Self::is_alloc_a(&alloc), "invalid value given: {:#?}", alloc);
		
		ExternData::alloc_into_unchecked(alloc).into_unchecked()
	*/}

	unsafe fn alloc_as_ref_unchecked(alloc: &Allocated) -> &Self {
		todo!()
	/*
		debug_assert!(Self::is_alloc_a(alloc), "invalid value given: {:#?}", alloc);
		
		ExternData::alloc_as_ref_unchecked(alloc).as_ref_unchecked()
	*/}

	unsafe fn alloc_as_mut_unchecked(alloc: &mut Allocated) -> &mut Self {
		todo!()
	/*
		debug_assert!(Self::is_alloc_a(alloc), "invalid value given: {:#?}", alloc);
		
		ExternData::alloc_as_mut_unchecked(alloc).as_mut_unchecked()
	*/}
}

unsafe impl<T: ExternType> AllocatedType for T {
	fn into_alloc(self) -> Allocated {
		Allocated::new(ExternData::new(self))
	}

	fn is_alloc_a(alloc: &Allocated) -> bool {
		ExternData::try_alloc_as_ref(alloc).map_or(false, ExternData::is_a::<Self>)
	}

	#[inline]
	unsafe fn alloc_into_unchecked(alloc: Allocated) -> Self {
		debug_assert!(Self::is_alloc_a(&alloc), "invalid value given: {:#?}", alloc);
		
		ExternData::alloc_into_unchecked(alloc).into_unchecked()
	}

	#[inline]
	unsafe fn alloc_as_ref_unchecked(alloc: &Allocated) -> &Self {
		debug_assert!(Self::is_alloc_a(alloc), "invalid value given: {:#?}", alloc);
		
		ExternData::alloc_as_ref_unchecked(alloc).as_ref_unchecked()
	}

	#[inline]
	unsafe fn alloc_as_mut_unchecked(alloc: &mut Allocated) -> &mut Self {
		debug_assert!(Self::is_alloc_a(alloc), "invalid value given: {:#?}", alloc);
		
		ExternData::alloc_as_mut_unchecked(alloc).as_mut_unchecked()
	}
}

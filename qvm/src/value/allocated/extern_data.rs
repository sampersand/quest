use std::fmt::{self, Debug, Formatter};
use std::any::{Any, TypeId};
use try_traits::cmp::TryPartialEq;
use crate::{Value, Literal, ShallowClone, DeepClone};
use crate::value::{NamedType, HasAttrs};
use crate::value::allocated::{Allocated, AllocatedType, AllocType};
use crate::lmap::LMap;

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub(crate) struct Extern {
	parents: Vec<Value>,
	attrs: LMap,
	data: *mut (),
	vtable: *const VTable
}

#[derive(Clone, Copy)]
#[doc(hidden)]
pub struct VTable {
	type_id: fn() -> TypeId,

	shallow_clone: unsafe fn(*const ()) -> crate::Result<*mut ()>,
	deep_clone: unsafe fn(*const ()) -> crate::Result<*mut ()>,

	try_eq: unsafe fn(*const (), Value) -> crate::Result<bool>,
	debug: for<'b> unsafe fn(*const (), &mut Formatter<'b>) -> fmt::Result,

	drop: unsafe fn(*mut ()),
}

#[inline]
fn allocate<T>(data: T) -> *mut () {
	Box::into_raw(Box::new(data)) as *mut ()
}

#[inline]
unsafe fn deallocate<T>(data: *mut T){
	drop(Box::from_raw(data))
}

/// Data that's supplied by someone else's quest binndings.
pub trait ExternType : Debug + Any + NamedType + ShallowClone + DeepClone + TryPartialEq<Error=crate::Error> {
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
		unsafe { vec![*PARENTS.as_ptr()] }
	}

	#[doc(hidden)]
	const _VTABLE: &'static VTable = {
		// SAFETY: pointers passed to this must be valid `T`.
		unsafe fn _drop<T>(ptr: *mut ()) {
			std::ptr::drop_in_place(ptr as *mut T);
			deallocate(ptr);
		}

		// SAFETY: pointers passed to this must be valid `T`.
		unsafe fn _shallow_clone<T: ShallowClone>(ptr: *const ()) -> crate::Result<*mut ()> {
			(&*(ptr as *const T)).shallow_clone().map(allocate)
		}

		// SAFETY: pointers passed to this must be valid `T`.
		unsafe fn _deep_clone<T: DeepClone>(ptr: *const ()) -> crate::Result<*mut ()> {
			(&*(ptr as *const T)).deep_clone().map(allocate)
		}

		// SAFETY: pointers passed to this must be valid `T`.
		unsafe fn _try_eq<T: TryPartialEq<Error=crate::Error>>(ptr: *const (), rhs: Value) -> crate::Result<bool> {
			// (&*(ptr as *const T)).try_eq(&rhs)
			// (&*(ptr as *const T)).deep_clone().map(allocate)
			todo!()
		}

		// SAFETY: pointers passed to this must be valid `T`.
		unsafe fn _debug<T: Debug>(ptr: *const (), f: &mut Formatter) -> fmt::Result {
			Debug::fmt(&*(ptr as *const T), f)
		}

		&VTable {
			type_id: TypeId::of::<Self>, 
			drop:  _drop::<Self>,
			shallow_clone: _shallow_clone::<Self>,
			deep_clone: _deep_clone::<Self>,
			try_eq: _try_eq::<Self>,
			debug: _debug::<Self>
		}
	};
}

impl Debug for Extern {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		struct DataDebugger<'a>(&'a Extern);

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
				// we write it out ourselves b/c we don't want `format` affecting it.
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
			f.debug_struct("Extern")
				.field("data", &DataDebugger(self))
				.field("parents", &ParentsDebugger(&self.parents))
				.field("attrs", &self.attrs)
				.finish()
		} else {
			f.debug_tuple("Extern").field(&DataDebugger(self)).finish()
		}
	}
}

impl Drop for Extern {
	fn drop(&mut self) {
		// SAFETY: `data`'s type never changes, so calling this is valid.
		unsafe {
			((*self.vtable).drop)(self.data);
		}
	}
}


impl Extern {
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
}

impl HasAttrs for Extern {
	fn has_attr(&self, attr: Literal) -> bool {
		self.attrs.has(attr)
	}

	fn get_attr(&self, attr: Literal) -> Option<&Value> {
		self.attrs.get(attr)
	}

	fn get_attr_mut(&mut self, attr: Literal) -> Option<&mut Value> {
		self.attrs.get_mut(attr)
	}

	fn set_attr(&mut self, attr: Literal, value: Value) {
		self.attrs.set(attr, value);
	}

	fn del_attr(&mut self, attr: Literal) -> Option<Value> {
		self.attrs.del(attr)
	}
}

impl Extern {
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

impl ShallowClone for Extern {
	fn shallow_clone(&self) -> crate::Result<Self> {
		Ok(Self {
			parents: self.parents.clone(),
			attrs: self.attrs.clone(), 
			data: unsafe { ((*self.vtable).shallow_clone)(self.data)? },
			vtable: self.vtable
		})
	}
}

impl DeepClone for Extern {
	fn deep_clone(&self) -> crate::Result<Self> {
		// TODO: maybe require a `DeepClone` impl, so I can add it to the vtable?
		Ok(Self {
			parents: self.parents.clone(),
			attrs: self.attrs.clone(), 
			data: unsafe { ((*self.vtable).deep_clone)(self.data)? },
			vtable: self.vtable
		})
	}
}

unsafe impl AllocatedType for Extern {
	fn into_alloc(self) -> Allocated {
		Allocated::new(AllocType::Extern(self))
	}

	fn is_alloc_a(alloc: &Allocated) -> bool {
		matches!(alloc.inner().data, AllocType::Extern(_))
	}

	unsafe fn alloc_as_ref_unchecked(alloc: &Allocated) -> &Self {
		debug_assert!(Self::is_alloc_a(alloc), "invalid value given: {:#?}", alloc);

		if let AllocType::Extern(ref externdata) = alloc.inner().data {
			externdata
		} else {
			std::hint::unreachable_unchecked()
		}
	}

	unsafe fn alloc_as_mut_unchecked(alloc: &mut Allocated) -> &mut Self {
		debug_assert!(Self::is_alloc_a(alloc), "invalid value given: {:#?}", alloc);

		if let AllocType::Extern(ref mut externdata) = alloc.inner_mut().data {
			externdata
		} else {
			std::hint::unreachable_unchecked()
		}
	}
}

unsafe impl<T: ExternType> AllocatedType for T {
	fn into_alloc(self) -> Allocated {
		Allocated::new(AllocType::Extern(Extern::new(self)))
	}

	fn is_alloc_a(alloc: &Allocated) -> bool {
		Extern::try_alloc_as_ref(alloc).map_or(false, Extern::is_a::<Self>)
	}

	#[inline]
	unsafe fn alloc_as_ref_unchecked(alloc: &Allocated) -> &Self {
		debug_assert!(Self::is_alloc_a(alloc), "invalid value given: {:#?}", alloc);
		
		Extern::alloc_as_ref_unchecked(alloc).as_ref_unchecked()
	}

	#[inline]
	unsafe fn alloc_as_mut_unchecked(alloc: &mut Allocated) -> &mut Self {
		debug_assert!(Self::is_alloc_a(alloc), "invalid value given: {:#?}", alloc);
		
		Extern::alloc_as_mut_unchecked(alloc).as_mut_unchecked()
	}
}

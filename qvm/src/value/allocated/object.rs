use try_traits::clone::TryClone;
use crate::{Value, LMap};
use std::fmt::{self, Debug, Formatter};
use std::any::{Any, TypeId};
use crate::value::allocated::AllocatedType;

pub struct Object {
	parents: Vec<Value>,
	attrs: crate::LMap,
	data: *mut (),
	vtable: &'static VTable
}

#[derive(Clone, Copy)]
#[doc(hidden)]
pub struct VTable {
	type_id: fn() -> TypeId,

	// SAFETY: the pointer must be a valid pointer to a type of `type_id`.
	drop: Option<unsafe fn(*mut ())>,

	// SAFETY: the pointer must be a valid pointer to a type of `type_id`.
	try_clone: unsafe fn(*const ()) -> crate::Result<*mut ()>,

	debug: for<'b> unsafe fn(*const (), &mut Formatter<'b>) -> fmt::Result
}

#[inline]
fn allocate<T>(data: T) -> *mut () {
	Box::into_raw(Box::new(data)) as *mut ()
}

/// A heap allocated object.
pub trait QuestObject : Debug + TryClone<Error=crate::Error> + Any {
	/// The parents associated with some value.
	fn parents(&self) -> Vec<Value>;

	#[doc(hidden)]
	const VTABLE: &'static VTable = {
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

impl Debug for Object {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		struct DataDebugger<'a>(&'a Object);

		impl<'a> Debug for DataDebugger<'a> {
			fn fmt(&self, f: &mut Formatter) -> fmt::Result {
				// SAFETY: `data`'s type never changes, so calling this is valid.
				unsafe {
					(self.0.vtable.debug)(self.0.data, f)
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
			f.debug_tuple("Object").field(&DataDebugger(self)).finish()
		} else {
			f.debug_struct("Object")
				.field("data", &DataDebugger(self))
				.field("parents", &ParentsDebugger(&self.parents))
				.field("attrs", &self.attrs)
				.finish()
		}
	}
}

impl Drop for Object {
	fn drop(&mut self) {
		// SAFETY: `data`'s type never changes, so calling this is valid.
		if let Some(dropfn) = self.vtable.drop {
			unsafe {
				(dropfn)(self.data)
			}
		}
	}
}

impl Object {
	pub fn new<T: QuestObject>(data: T) -> Self {
		Self {
			parents: data.parents(),
			attrs: LMap::default(),
			data: allocate(data),
			vtable: T::VTABLE
		}
	}

	pub fn is_a<T: 'static>(&self) -> bool {
		TypeId::of::<T>() == self.vtable.type_id()
	}

	pub fn try_into<T: 'static>(self) -> Result<T, Self> {
		if self.is_a::<T>() {
			// SAFETY: We just verified `self` is a `T`.
			Ok(unsafe { self.try_into_unchecked() })
		} else {
			Err(self)
		}
	}

	pub unsafe fn try_into_unchecked<T: 'static>(self) -> T {
		debug_assert!(self.is_a::<T>(), "invalid value given: {:?}", self);

		// SAFETY: it's up to the caller to ensure that `ptr` is a valid `T`.
		let data = unsafe {
			Box::from_raw(self.data as *mut T)
		};

		std::mem::forget(self);

		*data
	}

	pub fn try_as_ref<T: QuestObject>(&self) -> Option<&T> {
		if self.is_a::<T>() {
			// SAFETY: We just verified `self` is a `T`.
			Some(unsafe { self.try_as_ref_unchecked() })
		} else {
			None
		}
	}

	pub unsafe fn try_as_ref_unchecked<T: QuestObject>(&self) -> &T {
		debug_assert!(self.is_a::<T>(), "invalid value given: {:?}", self);

		&*(self.data as *const T)
	}

	pub fn try_as_mut<T: QuestObject>(&mut self) -> Option<&mut T> {
		if self.is_a::<T>() {
			// SAFETY: We just verified `self` is a `T`.
			Some(unsafe { self.try_as_mut_unchecked() })
		} else {
			None
		}
	}

	pub unsafe fn try_as_mut_unchecked<T: QuestObject>(&mut self) -> &mut T {
		debug_assert!(self.is_a::<T>(), "invalid value given: {:?}", self);

		&mut *(self.data as *mut T)
	}
}

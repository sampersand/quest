use crate::value::{Value, Literal, QuestValue};
use std::fmt::{self, Debug, Display, Pointer, Formatter};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::{HashMap, Entry};
use parking_lot::RwLock;

type Func = fn(&Value, &[&Value]) -> crate::Result<Value>;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct BuiltinFn(fn(&Value, &[&Value]) -> crate::Result<Value>);

lazy_static::lazy_static! {
	static ref FUNCTIONS: RwLock<HashMap<BuiltinFn, Literal>> = RwLock::new(HashMap::new());
}

impl Hash for BuiltinFn {
	#[inline]
	fn hash<H: Hasher>(&self, h: &mut H) {
		(self.0 as usize).hash(h);
	}
}

impl BuiltinFn {
	/// Attempts to register a function with the given name.
	///
	/// The function should not have been registered before, and the `name` must not be associated with another function.
	///
	/// # Panics
	/// Panics if the name was registered before.
	pub fn new(name: Literal, func: Func) -> Self {
		debug_assert_eq!(func as usize & 0b111, 0, "function wasn't 8-bit aligned?");

		let builtinfn = Self(func);

		if FUNCTIONS.read().values().find(|&&x| x == name).is_some() {
			panic!("Cannot add name '{}' in twice!", name);
		}

		match FUNCTIONS.write().entry(builtinfn) {
			Entry::Vacant(mut entry) => entry.insert(name),
			Entry::Occupied(entry) =>
				panic!("Cannot add name '{}' to function '{:p}' twice; old name: {}", name, builtinfn, entry.get())
		};

		builtinfn
	}

	/// Creates a new [`BuiltinFn`], which should have been registered before.
	///
	/// # Safety
	/// The caller must ensure that `func` has been registered before via calling [`new`].
	#[inline]
	pub unsafe fn new_unchecked(func: Func) -> Self {
		let builtinfn = Self(func);

		debug_assert!(FUNCTIONS.read().contains_key(&builtinfn), "called `new_unchecked` with unknown function {:p}", builtinfn);

		builtinfn
	}

	fn name(&self) -> Literal {
		*FUNCTIONS.read().get(self).expect("All functions should have names.")
	}
}

impl Debug for BuiltinFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_tuple("BuiltinFn")
			.field(&self.name().repr())
			.finish()
	}
}

impl Display for BuiltinFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(self.name().repr(), f)
	}
}

impl Pointer for BuiltinFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Pointer::fmt(&(self.0 as usize as *const ()), f)
	}
}

impl Eq for BuiltinFn {}
impl PartialEq for BuiltinFn {
	fn eq(&self, rhs: &Self) -> bool {
		// if two functions have the same address, we define them as the same BuiltinFn.
		(self.0 as u64) == (rhs.0 as u64)
	}
}

const BUILTINFN_TAG: u64   = 0b0100;
const BUILTINFN_SHIFT: u64 = 0b0100;
const BUILTINFN_MASK: u64  = 0b0111;

unsafe impl QuestValue for BuiltinFn {
	const TYPENAME: &'static str = "qvm::BuiltinFn";

	#[inline]
	fn into_value(self) -> Value {
		// SAFETY: This is the definition of a valid float.
		unsafe {
			Value::from_bits_unchecked(((self.0 as u64) << BUILTINFN_SHIFT) | BUILTINFN_TAG)
		}
	}

	#[inline]
	fn is_value_a(value: &Value) -> bool {
		(value.bits() & BUILTINFN_MASK) == BUILTINFN_TAG
	}

	/// Note the value has to have been a valid builtinfn.
	unsafe fn value_into_unchecked(value: Value) -> Self {
		debug_assert!(value.is_a::<Self>());

		let bits: u64 = value.bits() >> BUILTINFN_SHIFT;
		debug_assert_ne!(0, bits, "null function encountered.");

		// SAFETY: if `value` was previously a `BuiltinFn`, we know it's valid.
		Self::new_unchecked(unsafe {
			std::mem::transmute::<usize, Func>(bits as usize)
		})
	}

	fn get_attr(&self, attr: Literal) -> Option<&Value> {
		todo!()
	}

	fn get_attr_mut(&mut self, attr: Literal) -> Option<&mut Value> {
		todo!()
	}

	fn del_attr(&mut self, attr: Literal) -> Option<Value> {
		todo!()
	}

	fn set_attr(&mut self, attr: Literal, value: Value) {
		todo!()
	}
}

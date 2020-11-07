mod registers;

pub use self::registers::{Registers, RegisterIndex};

use quest_core::Object;

/// A general-purpose register that can hold any [`Object`].
#[derive(Debug, Clone, Default)]
pub struct Register {
	// Note: we use `Option` so we don't have to allocate every time we `.take`.
	obj: Object
}

impl Register {
	/// Creates a new register containing the given object.
	pub fn new(obj: Object) -> Self {
		Self { obj }
	}

	/// Retrieves this register's object.
	pub fn load(&self) -> &Object {
		&self.obj
	}

	/// Takes ownership of this register's object, replacing it with `null`.
	pub fn take(&mut self) -> Object {
		std::mem::replace(&mut self.obj, Object::default())
	}

	/// Inserts the `value` into this register, returning the previous value.
	pub fn store(&mut self, value: Object) {
		self.obj = value;
	}

	/// A current hack to convert a register to an integer.
	/// this will be replaced in the future most likely.
	pub fn as_integer(&self) -> i64 {
		self.load()
			.downcast::<quest_core::types::Number>()
			.expect("register isn't an int!")
			.truncate()
	}
}

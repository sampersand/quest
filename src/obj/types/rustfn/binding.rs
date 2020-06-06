use crate::obj::{Object, Args, Result, types};
use std::sync::RwLock;
use std::ops::Deref;

type Stack = Vec<Binding>;

#[derive(Debug, Clone)]
pub struct Binding(Object);

impl Default for Binding {
	fn default() -> Self {
		Binding::instance()
	}
}

impl Binding {
	pub fn try_instance() -> Option<Binding> {
		Binding::with_stack(|stack| {
			stack.read()
				.expect("stack poisoned")
				.last()
				.map(|x| x.clone())
		})
	}

	pub fn instance() -> Binding {
		Binding::try_instance().expect("we should always have a stackframe")
	}

	pub fn set_binding(new: Object) -> Binding {
		let new = Binding(new);
		Binding::with_stack(|stack| {
			let mut stack = stack.write().expect("stack poisoned");
			assert!(stack.pop().is_some());
			stack.push(new.clone());
			new
		})
	}

	fn new_child(&self, binding: Binding) -> Result<Binding> {
		let new_scope = Object::from(types::Scope);
		new_scope.add_mixin(binding.as_ref().clone())?;
		Ok(Binding(new_scope))
	}

	pub fn new_stackframe<F: FnOnce(&Binding) -> Result<Object>>(args: Args, func: F) -> Result<Object> {
		struct StackGuard<'a>(&'a RwLock<Stack>, &'a Binding);
		impl Drop for StackGuard<'_> {
			fn drop(&mut self) {
				let mut stack = self.0.write().expect("stack poisoned");
				match stack.pop() {
					None => eprintln!("nothing left to pop?"),
					Some(binding) if binding.0.is_identical(self.1.as_ref()) => {},
					// this is now ok, as you can set __this__.
					Some(binding) => {/*eprintln!("bindings don't match: {:?}", binding)*/}
				}
			}
		}

		Binding::with_stack(|stack| {
			let binding = {
				let binding = Object::from(types::Scope);
				if let Some(caller) = args.this().ok() {
					binding.set_attr("__caller__", caller.clone());
				}
				binding.set_attr("__args__", Object::from(Vec::from(args.args(..)?)))?;
				Binding(binding)
			};

			{
				let mut stack = stack.write().expect("stack poisoned");
				stack.push(binding.clone());
			};

			let guard = StackGuard(stack, &binding);
			func(&binding)
		})
	}

	fn with_stack<F: FnOnce(&RwLock<Stack>) -> R, R>(func: F) -> R {
		thread_local!(
			// static STACK: RwLock<Stack> = RwLock::new(vec![]);
			static STACK: RwLock<Stack> = RwLock::new(vec![Binding(Object::new(types::Scope))]);
		);

		STACK.with(func)
	}
}

impl From<Object> for Binding {
	fn from(obj: Object) -> Self {
		Binding(obj)
	}
}

impl AsRef<Object> for Binding {
	fn as_ref(&self) -> &Object {
		&self
	}
}

impl Deref for Binding {
	type Target = Object;
	fn deref(&self) -> &Object {
		&self.0
	}
}
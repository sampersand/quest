use crate::{Object, Args, types};
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
			stack.read().expect("stack poisoned").last().cloned()
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


	#[deprecated]
	pub fn new_stackframe_old<F>(args: crate::ArgsOld, func: F) -> crate::Result<Object>
	where
		F: FnOnce(&Binding) -> crate::Result<Object>,
	{
		struct StackGuard<'a>(&'a RwLock<Stack>, &'a Binding);
		impl Drop for StackGuard<'_> {
			fn drop(&mut self) {
				let mut stack = self.0.write().expect("stack poisoned");
				match stack.pop() {
					None => eprintln!("nothing left to pop?"),
					Some(binding) if binding.0.is_identical(self.1.as_ref()) => {},
					// this is now ok, as you can set __this__.
					Some(_binding) => {/*eprintln!("bindings don't match: {:?}", binding)*/}
				}
			}
		}

		Binding::with_stack(|stack| {
			let binding = {
				let binding = Object::from(types::Scope);

				if let Ok(caller) = args.this() {
					binding.add_parent(caller.clone())?;
				}

				for (i, arg) in args.args(..)?.as_ref().iter().enumerate() {
					// `+1` because we don't start at 0
					binding.set_attr(Object::from(format!("_{}", i/* + 1*/)), arg.clone())?;
					// binding.set_attr(Object::from(format!("_{}", i + 1)), arg.clone())?;
				}

				binding.set_attr("__args__", Object::from(Vec::from(args.args(..)?)))?;
				if let Some(callee) = stack.read().expect("bad stack").last() {
					binding.set_attr("__callee__", callee.as_ref().clone())?;
				}
				Binding(binding)
			};

			{
				let mut stack = stack.write().expect("stack poisoned");
				stack.push(binding.clone());
			};


			let _guard = StackGuard(stack, &binding);
			
			match func(&binding) {
				Err(crate::Error::Return { to, what }) if to.as_ref().eq_obj(binding.as_ref())?
					=> Ok(what),
				other => other
			}
		})
	}

	pub fn new_stackframe<F>(parent: Option<Object>, args: Args, func: F) -> crate::Result<Object>
	where
		F: FnOnce(&Binding) -> crate::Result<Object>,
	{
		struct StackGuard<'a>(&'a RwLock<Stack>, &'a Binding);
		impl Drop for StackGuard<'_> {
			fn drop(&mut self) {
				self.0.write().expect("stack poisoned").pop();
			}
		}

		Binding::with_stack(|stack| {
			let binding = {
				let binding = Object::from(types::Scope);

				if let Some(parent) = parent {
					binding.add_parent(parent)?;
				}


				for (i, arg) in args.iter().enumerate() {
					binding.set_attr(Object::from(format!("_{}", i)), (*arg).clone())?;
				}

				binding.set_attr("__args__", Object::from(types::List::from(args)))?;

				if let Some(callee) = stack.read().expect("bad stack").last() {
					binding.set_attr("__callee__", callee.as_ref().clone())?;
				}

				Binding(binding)
			};

			{
				let mut stack = stack.write().expect("stack poisoned");
				stack.push(binding.clone());
			};


			let _guard = StackGuard(stack, &binding);
			
			match func(&binding) {
				Err(crate::Error::Return { to, what }) if to.as_ref().eq_obj(binding.as_ref())?
					=> Ok(what),
				other => other
			}
		})
	}

	pub fn with_stack<F: FnOnce(&RwLock<Stack>) -> R, R>(func: F) -> R {
		thread_local!(
			static STACK: RwLock<Stack> = RwLock::new(vec![]);
			// static STACK: RwLock<Stack> = RwLock::new(vec![Binding(Object::new(types::Scope))]);
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
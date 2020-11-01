use crate::{Object, Args};
use crate::types::{Scope, List};
use std::ops::Deref;
use parking_lot::RwLock;

type Stack = Vec<Binding>;

#[derive(Debug, Clone)]
pub struct Binding(Object);

impl Default for Binding {
	#[inline]
	fn default() -> Self {
		Binding::instance()
	}
}

impl Binding {
	pub fn instance() -> Binding {
		Binding::with_stack(|stack| {
			stack.read().last()
				.expect("we should always have a stackframe")
				.clone()
		})
	}

	pub fn set_binding(new: Object) -> Binding {
		let new = Binding(new);
		Binding::with_stack(|stack| {
			let mut stack = stack.write();
			assert!(stack.pop().is_some());
			stack.push(new.clone());
			new
		})
	}

	pub fn stack() -> Vec<Binding> {
		Self::with_stack(|s| {
			let mut stack = s.read().clone();
			stack.reverse();
			stack
		})
	}


	#[tracing::instrument(name="Binding::new_stackframe", level="debug", skip(func))]
	pub fn new_stackframe<F>(parent: Option<Object>, args: Args, func: F) -> crate::Result<Object>
	where
		F: FnOnce(&Binding) -> crate::Result<Object>,
	{
		struct StackGuard<'a>(&'a RwLock<Stack>, &'a Binding);
		impl Drop for StackGuard<'_> {
			#[inline]
			fn drop(&mut self) {
				self.0.write().pop();
			}
		}

		let span = 
			if let Some(ref parent) = parent {
				tracing::trace_span!("stackframe", id=%parent.id())
			} else {
				tracing::trace_span!("stackframe")
			};
		let _guard = span.enter();


		Binding::with_stack(|stack| {
			let binding = {
				let binding = Object::from(Scope);

				if let Some(parent) = parent {
					binding.add_parent(parent)?;
				}

				for (i, arg) in args.iter().enumerate() {
					binding.set_attr(Object::from(format!("_{}", i)), (*arg).clone())?;
				}

				binding.set_attr_lit("__args__", Object::from(List::from(args)))?;

				if let Some(callee) = stack.read().last() {
					binding.set_attr_lit("__callee__", callee.as_ref().clone())?;
					binding.add_parent(callee.as_ref().clone())?;
				}

				Binding(binding)
			};

			{
				let mut stack = stack.write();
				stack.push(binding.clone());
			};

			let _guard = StackGuard(stack, &binding);
 			
			match func(&binding) {
				Err(crate::Error::Return { to, obj }) if to.as_ref().eq_obj(binding.as_ref())?
					=> Ok(obj),
				other => other
			}
		})
	}


	pub fn run_stackframe<F>(binding: Binding, func: F) -> crate::Result<Object>
	where
		F: FnOnce(&Binding) -> crate::Result<Object>
	{
		struct StackGuard<'a>(&'a RwLock<Stack>, &'a Binding);
		impl Drop for StackGuard<'_> {
			#[inline]
			fn drop(&mut self) {
				self.0.write().pop();
			}
		}

		Self::with_stack(|stack| {
			{
				let mut stack = stack.write();
				stack.push(binding.clone());
			};

			let _guard = StackGuard(stack, &binding);
 			
			match func(&binding) {
				Err(crate::Error::Return { to, obj }) if to.as_ref().eq_obj(binding.as_ref())?
					=> Ok(obj),
				other => other
			}
		})
	}

	#[inline]
	pub fn with_stack<F: FnOnce(&RwLock<Stack>) -> R, R>(func: F) -> R {
		thread_local!(
			// static STACK: RwLock<Stack> = RwLock::new(vec![]);
			static STACK: RwLock<Stack> = RwLock::new(vec![Binding(Object::new(Scope))]);
		);

		STACK.with(func)
	}
}

impl From<Object> for Binding {
	#[inline]
	fn from(obj: Object) -> Self {
		Binding(obj)
	}
}

impl AsRef<Object> for Binding {
	#[inline]
	fn as_ref(&self) -> &Object {
		&self
	}
}

impl From<Binding> for Object {
	#[inline]
	fn from(binding: Binding) -> Self {
		binding.0
	}
}

impl Deref for Binding {
	type Target = Object;
	#[inline]
	fn deref(&self) -> &Object {
		&self.0
	}
}

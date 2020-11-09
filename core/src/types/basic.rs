use crate::{Object, Result, Args, Literal};
use tracing::instrument;

/// A class that holds all the basic functions objects can have.
///
/// Note that this class isn't meant to be instantiated; rather, it's meant to be
/// used as a parent.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Basic;

/// Quest functions
impl Basic {
	/// Convert `this` into a [`Boolean`](crate::types::Boolean).
	///
	/// All objects are [`true`](crate::types::Boolean::TRUE) by default.
	///
	/// # Arguments
	/// None.
	///
	/// # Returns
	/// The value [true](crate::types::Boolean::TRUE).
	///
	/// # Errors
	/// None.
	///
	/// # Rust Examples
	/// ```rust
	/// # use quest_core::{impl_object_type, types::ObjectType};
	/// use quest_core::{Object, Literal, types::Boolean};
	/// #[derive(Debug, Clone, PartialEq)]
	/// struct Dummy;
	/// 
	/// impl_object_type! { for Dummy [(parents quest_core::types::Basic)]: }
	/// # quest_core::init();
	/// # <Dummy as quest_core::types::ObjectType>::initialize().unwrap();
	/// 
	/// assert_eq!(
	/// 	*Object::new(Dummy)
	/// 		.call_attr_lit(&Literal::AT_BOOL, &[]).unwrap()
	/// 		.downcast::<Boolean>().unwrap(),
	/// 	Boolean::new(true)
	/// );
	/// ```
	///
	/// # Quest Examples
	/// ```quest
	/// assert(Basic.@bool() == true);
	/// ```
	#[instrument(name="Basic::@bool", level="trace", skip(_this), fields(self=?_this))]
	pub fn qs_at_bool(_this: &Object, _: Args) -> Result<Object> {
		Ok(true.into())
	}

	/// Converts `this` into a [`Text`](crate::types::Text).
	///
	/// This is simply a redirect to the `inspect` method.
	///
	/// # Arguments
	/// Any arguments given are passed to the `inspect` method.
	///
	/// # Returns
	/// The result of calling `this`'s `inspect` attribute with the given `args`.
	///
	/// # Errors
	/// Any errors returned by calling `this`'s `inspect` attribute with the given `args` are returned.
	///
	/// # Rust Examples
	/// ```rust
	/// # use quest_core::{impl_object_type, types::ObjectType};
	/// use quest_core::{Object, Literal, types::Text};
	/// #[derive(Debug, Clone, PartialEq)]
	/// struct Dummy;
	/// 
	/// impl_object_type! { for Dummy [(parents quest_core::types::Basic)]:
	/// 	"inspect" => method |_, _| Ok("hi friend".into())
	/// }
	/// # quest_core::init();
	/// # <Dummy as quest_core::types::ObjectType>::initialize().unwrap();
	/// 
	/// assert_eq!(
	/// 	*Object::new(Dummy)
	/// 		.call_attr_lit(&Literal::AT_TEXT, &[]).unwrap()
	/// 		.downcast::<Text>().unwrap(),
	/// 	Text::new("hi friend")
	/// );
	/// ```
	///
	/// # Quest Examples
	/// ```quest
	/// assert(Basic.@text() == Basic.inspect());
	/// ```
	#[instrument(name="Basic::@text", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_at_text<'o>(this: &'o Object, args: Args<'_, 'o>) -> Result<Object> {
		this.call_attr_lit(&Literal::INSPECT, args)
	}

	/// See if `this` is equal to the first argument.
	///
	/// Unless overriden, objects are the same when they are the _exact same object_.
	///
	/// # Arguments
	/// 1. (required) The other object.
	///
	/// # Returns
	/// Returns whether `this` [is identical](Object::is_identical) to the first argument.
	///
	/// # Errors
	/// An [`ArgumentError`] is returned if no arguments are given.
	///
	/// # Rust Examples
	/// ```rust
	/// # use quest_core::{impl_object_type, types::ObjectType};
	/// use quest_core::{Object, Literal, types::Boolean};
	/// #[derive(Debug, Clone, PartialEq)]
	/// struct Dummy;
	/// 
	/// impl_object_type! { for Dummy [(parents quest_core::types::Basic)]: }
	/// # quest_core::init();
	/// # <Dummy as quest_core::types::ObjectType>::initialize().unwrap();
	/// 
	/// let obj = Object::new(Dummy);
	///
	/// // When they're not identical, they won't be equal.
	/// assert_eq!(
	/// 	*obj
	/// 		.call_attr_lit(&Literal::EQL, vec![&Object::new(Dummy)]).unwrap()
	/// 		.downcast::<Boolean>().unwrap(),
	/// 	Boolean::new(false)
	/// );
	///
	/// // when they're identical they're the same.
	/// assert_eq!(
	/// 	*obj
	/// 		.call_attr_lit(&Literal::EQL, vec![&obj]).unwrap()
	/// 		.downcast::<Boolean>().unwrap(),
	/// 	Boolean::new(true)
	/// );
	/// ```
	///
	/// # Quest Examples
	/// ```quest
	/// assert(Basic == Basic);
	/// ```
	#[instrument(name="Basic::==", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_eql(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.try_arg(0)?;

		Ok(this.is_identical(rhs).into())
	}

	/// See if `this` isn't equal to the first argument.
	///
	/// This simply calls the `==` method and then the `!` method on the result.
	///
	/// # Arguments
	/// 1. (required) The other object.
	#[instrument(name="Basic::!=", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_neq<'o>(this: &'o Object, args: Args<'_, 'o>) -> Result<Object> {
		this.call_attr_lit(&Literal::EQL, args)?
			.call_attr_lit(&Literal::NOT, &[])
	}

	/// Get the logical inverse of `this`.
	///
	/// This simply calls the `@bool` method and then the `!` method on the result.
	#[instrument(name="Basic::!", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_not<'o>(this: &'o Object, args: Args<'_, 'o>) -> Result<Object> {
		this.call_attr_lit(&Literal::AT_BOOL, args)?
			.call_attr_lit(&Literal::NOT, &[])
	}

	/// Get a hash of `this`.
	///
	/// This generates a unique hash per object by hashing the object's [`id`](Object::id).
	#[instrument(name="Basic::hash", level="trace", skip(this), fields(self=?this))]
	pub fn qs_hash(this: &Object, _: Args) -> Result<Object> {
		Ok(crate::utils::hash(&this.id()).into())
	}

	/// Creates a clone of `this`.
	///
	/// This doesn't actually clone the underlying data---it marks it as "shared", and if it's
	/// modified it will be cloned.
	#[instrument(name="Basic::clone", level="trace", skip(this), fields(self=?this))]
	pub fn qs_clone(this: &Object, _: Args) -> Result<Object> {
		Ok(this.deep_clone())
	}

	/// Simply returns `this`.
	///
	/// This is useful for passing methods around to functions.
	#[instrument(name="Basic::itself", level="trace", skip(this), fields(self=?this))]
	pub fn qs_itself(this: &Object, _: Args) -> Result<Object> {
		Ok(this.clone())
	}

	#[instrument(name="Basic::instance_exec", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_instance_exec(this: &Object, args: Args) -> Result<Object> {
		let to_exec = args.try_arg(0)?;

		crate::Binding::run_stackframe(this.clone().into(), |_| {
			if to_exec.has_attr_lit("call_noscope")? {
				to_exec.call_attr_lit("call_noscope", &[])
			} else {
				to_exec.call_attr_lit(&Literal::CALL, &[])
			} 
		})
	}
}

impl_object_type!{
for Basic [(parents super::Pristine)]:
	"@bool" => method Self::qs_at_bool,
	"@text" => method Self::qs_at_text,
	"==" => method Self::qs_eql,
	"!=" => method Self::qs_neq,
	"!" => method Self::qs_not,
	"clone" => method Self::qs_clone,
	"hash" => method Self::qs_hash,
	"itself" => method Self::qs_itself,
	"instance_exec" => method Self::qs_instance_exec,

	// TODO: move these out of kernel
	"if" => method super::Kernel::qs_if, 
	"disp" => function super::Kernel::qs_disp,
	"dispn" => function super::Kernel::qs_dispn,
	"while" => method super::Kernel::qs_while,
	"loop" => method super::Kernel::qs_loop,
	"return" => function super::Kernel::qs_return,
	"assert" => method super::Kernel::qs_assert,

	"then" => method super::Kernel::qs_if, 
	"else" => method super::Kernel::qs_unless, 
	"or" => method |this, args| {
		let if_false = args.try_arg(0)?;
		let is_truthy = this.call_downcast::<super::Boolean>()?.into_inner();

		if is_truthy {
			Ok(this.clone())
		} else {
			Ok(if_false.clone())
		}
	},
	"and" => method |this, args| {
		let if_true = args.try_arg(0)?;
		let is_truthy = this.call_downcast::<super::Boolean>()?.into_inner();

		if is_truthy {
			Ok(if_true.clone())
		} else {
			Ok(this.clone())
		}
	}
	// "||"    => Self::or,
	// "&&"    => Self::and,
}


#[cfg(test)]
mod tests {
	use super::*;

	mod qs {
		use super::*;
		use crate::types::*;

		#[test]
		fn at_bool() {
			assert_call_eq!(Basic::qs_at_bool(Basic) -> Boolean, true);
			assert_call_eq!(Basic::qs_at_bool(Basic, false) -> Boolean, true);
		}

		#[test]
		fn at_text() {
			#[derive(Debug, Clone, PartialEq)]
			struct Dummy;

			impl_object_type! { for Dummy [(parents Basic)]:
				"inspect" => method |_, _| Ok("foo".into())
			}

			<Dummy as crate::types::ObjectType>::initialize().unwrap();

			assert_eq!(
				*Object::from(Dummy)
					.call_attr_lit(&Literal::INSPECT, &[]).unwrap()
					.call_downcast::<Text>().unwrap(),
				*"foo"
			);

			assert_call_eq!(Basic::qs_at_text(Dummy) -> Text, *"foo");
			assert_call_eq!(Basic::qs_at_text(Dummy, 13) -> Text, *"foo");

			assert_call_idempotent!(Basic::qs_at_text(Dummy) -> Dummy, Dummy);
		}

		#[test]
		fn eql() {
			let obj1 = Object::from(Basic);
			let obj2 = Object::from(Basic);

			assert_call_eq!(Basic::qs_eql(obj1.clone(), obj1.clone()) -> Boolean, true);
			assert_call_eq!(Basic::qs_eql(obj1.clone(), obj2.clone()) -> Boolean, false);
			assert_call_eq!(Basic::qs_eql(obj1.clone(), obj1.clone(), obj2.clone()) -> Boolean, true);

			assert_call_missing_parameter!(Basic::qs_eql(obj1.clone()), 0);
			assert_call_idempotent!(Basic::qs_eql(obj1, obj2) -> Basic, Basic);
		}

		#[test]
		fn neq() {
			#[derive(Debug, Clone, PartialEq)]
			struct Dummy(i64);

			impl_object_type! { for Dummy [(parents Basic)]:
				"==" => method |this: &Object, args: Args| {
					let rhs = args.try_arg(0)?;
					this.try_downcast::<Dummy>().and_then(|this| {
						rhs.try_downcast::<Dummy>().map(|rhs| Object::from(*this == *rhs))
					})
				}
			}

			<Dummy as crate::types::ObjectType>::initialize().unwrap();

			let obj1 = Object::from(Dummy(12));
			let obj2 = Object::from(Dummy(12));
			let obj3 = Object::from(Dummy(14));

			assert_eq!(
				obj1.call_attr_lit(&Literal::EQL, &[&obj1]).unwrap()
					.call_downcast::<Boolean>().unwrap()
					.into_inner(),
				true
			);

			assert_eq!(
				obj1.call_attr_lit(&Literal::EQL, &[&obj2]).unwrap()
					.call_downcast::<Boolean>().unwrap()
					.into_inner(),
				true
			);

			assert_eq!(
				obj1.call_attr_lit(&Literal::EQL, &[&obj3]).unwrap()
					.call_downcast::<Boolean>().unwrap()
					.into_inner(),
				false
			);

			assert_call_eq!(Basic::qs_neq(obj1.clone(), obj1.clone()) -> Boolean, false);
			assert_call_eq!(Basic::qs_neq(obj1.clone(), obj2.clone()) -> Boolean, false);
			assert_call_eq!(Basic::qs_neq(obj1.clone(), obj3.clone()) -> Boolean, true);		
			assert_call_eq!(Basic::qs_neq(obj1.clone(), obj2.clone(), obj3) -> Boolean, false);

			assert_call_missing_parameter!(Basic::qs_eql(obj1.clone()), 0);
			assert_call_idempotent!(Basic::qs_eql(obj1, obj2) -> Dummy, Dummy(12));
		}

		#[test]
		fn not() {
			#[derive(Debug, Clone, PartialEq)]
			struct Dummy(bool);

			impl_object_type! { for Dummy [(parents Basic)]:
				"@bool" => method |this: &Object, _: Args| {
					this.try_downcast::<Dummy>().map(|this| Object::from(this.0))
				}
			}

			<Dummy as crate::types::ObjectType>::initialize().unwrap();

			assert_call_eq!(Basic::qs_not(Dummy(true)) -> Boolean, false);
			assert_call_eq!(Basic::qs_not(Dummy(false)) -> Boolean, true);
			assert_call_eq!(Basic::qs_not(Dummy(false), Dummy(true)) -> Boolean, true);

			assert_call_idempotent!(Basic::qs_not(Dummy(true)) -> Dummy, Dummy(true));
		}


		#[test]
		fn hash() {
			let obj1 = Object::from(Basic);
			let obj2 = Object::from(Basic);

			let hash = *Basic::qs_hash(&obj1, args!()).unwrap()
				.call_downcast::<Number>().unwrap();
			// make sure repeated hashes are the same.
			assert_eq!(hash, call_unwrap!(Basic::qs_hash(obj1) -> Number; |n| *n));
			// make sure two hashes aren't identical for the same object.
			assert_ne!(hash, call_unwrap!(Basic::qs_hash(obj2) -> Number; |n| *n));

			assert_call_idempotent!(Basic::qs_hash(Basic));
		}

		#[test]
		fn clone() {
			#[derive(Debug, Clone, PartialEq)]
			struct Dummy(i32);

			impl_object_type! { for Dummy [(parents Basic)]:
				"==" => method |this: &Object, args: Args| {
					this.try_downcast::<Dummy>().and_then(|this|
						args.try_arg(0)?.try_downcast::<Dummy>().map(|rhs| *this == *rhs)
						.map(Object::from)
					)
				}
			}

			<Dummy as crate::types::ObjectType>::initialize().unwrap();

			let obj = &Object::from(Dummy(12));
			let clone = Basic::qs_clone(obj, args!()).unwrap();

			assert!(!obj.is_identical(&clone));
			assert!(obj.eq_obj(&clone).unwrap());

		}

		#[test]
		fn itself() {
			let obj = Object::from(Basic);
			assert!(obj.is_identical(&Basic::qs_itself(&obj, args!()).unwrap()));
			assert!(!obj.is_identical(&Basic::qs_itself(&Basic.into(), args!()).unwrap()));
		}
	}
}

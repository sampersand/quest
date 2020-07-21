use crate::{Object, Result, Args};
use crate::literals::{EQL, AT_BOOL, NOT, INSPECT};

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
	#[inline]
	pub fn qs_at_bool(_: &Object, _: Args) -> Result<Object> {
		Ok(true.into())
	}

	/// Converts `this` into a [`Text`](crate::types::Text).
	///
	/// This is simply a redirect to the `inspect` method.
	#[inline]
	pub fn qs_at_text<'o>(this: &'o Object, args: Args<'_, 'o>) -> Result<Object> {
		this.call_attr_lit(INSPECT, args)
	}

	/// See if `this` is equal to the first argument.
	///
	/// Unless overriden, objects are the same when they are the _exact same object_.
	///
	/// # Arguments
	/// 1. (required) The other object.
	#[inline]
	pub fn qs_eql(this: &Object, args: Args) -> Result<Object> {
		Ok(this.is_identical(args.arg(0)?).into())
	}

	/// See if `this` isn't equal to the first argument.
	///
	/// This simply calls the `==` method and then the `!` method on the result.
	///
	/// # Arguments
	/// 1. (required) The other object.
	#[inline]
	pub fn qs_neq<'o>(this: &'o Object, args: Args<'_, 'o>) -> Result<Object> {
		this.call_attr_lit(EQL, args)?.call_attr_lit(NOT, &[])
	}

	/// Get the logical inverse of `this`.
	///
	/// This simply calls the `@bool` method and then the `!` method on the result.
	#[inline]
	pub fn qs_not<'o>(this: &'o Object, args: Args<'_, 'o>) -> Result<Object> {
		this.call_attr_lit(AT_BOOL, args)?.call_attr_lit(NOT, &[])
	}

	/// Get a hash of `this`.
	///
	/// This generates a unique hash per object by hashing the object's [`id`](Object::id).
	#[inline]
	pub fn qs_hash(this: &Object, _: Args) -> Result<Object> {
		Ok(crate::utils::hash(&this.id()).into())
	}

	/// Creates a clone of `this`.
	///
	/// This doesn't actually clone the underlying data---it marks it as "shared", and if it's
	/// modified it will be cloned.
	#[inline]
	pub fn qs_clone(this: &Object, _: Args) -> Result<Object> {
		Ok(this.deep_clone())
	}

	/// Simply returns `this`.
	///
	/// This is useful for passing methods around to functions.
	#[inline]
	pub fn qs_itself(this: &Object, _: Args) -> Result<Object> {
		Ok(this.clone())
	}
}

impl_object_type!{
for Basic [(parents super::Kernel)]:
	"@bool" => function Basic::qs_at_bool,
	"@text" => function Basic::qs_at_text,
	"==" => function Basic::qs_eql,
	"!=" => function Basic::qs_neq,
	"!" => function Basic::qs_not,
	"clone" => function Basic::qs_clone,
	"hash" => function Basic::qs_hash,
	"itself" => function Basic::qs_itself,
	// "||"    => impls::or,
	// "&&"    => impls::and,
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
				"inspect" => function |_, _| Ok("foo".into())
			}

			<Dummy as crate::types::ObjectType>::initialize().unwrap();

			assert_eq!(
				Object::from(Dummy)
					.call_attr_lit(INSPECT, &[]).unwrap()
					.call_downcast_map(Text::clone).unwrap(),
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
				"==" => function |this: &Object, args: Args| {
					let rhs = args.arg(0)?;
					this.try_downcast_and_then(|this: &Dummy| {
						rhs.try_downcast_map(|rhs: &Dummy| Object::from(this == rhs))
					})
				}
			}

			<Dummy as crate::types::ObjectType>::initialize().unwrap();

			let obj1 = Object::from(Dummy(12));
			let obj2 = Object::from(Dummy(12));
			let obj3 = Object::from(Dummy(14));

			assert_eq!(
				obj1.call_attr_lit(EQL, &[&obj1]).unwrap()
					.call_downcast_map(Boolean::clone).unwrap()
					.into_inner(),
				true
			);

			assert_eq!(
				obj1.call_attr_lit(EQL, &[&obj2]).unwrap()
					.call_downcast_map(Boolean::clone).unwrap()
					.into_inner(),
				true
			);

			assert_eq!(
				obj1.call_attr_lit(EQL, &[&obj3]).unwrap()
					.call_downcast_map(Boolean::clone).unwrap()
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
				"@bool" => function |this: &Object, _: Args| {
					this.try_downcast_map(|this: &Dummy| Object::from(this.0))
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

			let hash = Basic::qs_hash(&obj1, args!()).unwrap()
				.call_downcast_map(Number::clone).unwrap();
			// make sure repeated hashes are the same.
			assert_eq!(hash, call_unwrap!(Basic::qs_hash(obj1) -> Number; Number::clone));
			// make sure two hashes aren't identical for the same object.
			assert_ne!(hash, call_unwrap!(Basic::qs_hash(obj2) -> Number; Number::clone));

			assert_call_idempotent!(Basic::qs_hash(Basic));
		}

		#[test]
		fn clone() {
			#[derive(Debug, Clone, PartialEq)]
			struct Dummy(i32);

			impl_object_type! { for Dummy [(parents Basic)]:
				"==" => function |this: &Object, args: Args| {
					this.try_downcast_and_then(|this: &Dummy|
						args.arg(0)?.try_downcast_map(|rhs: &Dummy| this == rhs).map(Object::from)
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

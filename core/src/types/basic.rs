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
	pub fn qs_at_text(this: &Object, args: Args) -> Result<Object> {
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
	pub fn qs_neq(this: &Object, args: Args) -> Result<Object> {
		this.call_attr_lit(EQL, args)?
		    .call_attr_lit(NOT, &[])
	}

	/// Get the logical inverse of `this`.
	///
	/// This simply calls the `@bool` method and then the `!` method on the result.
	#[inline]
	pub fn qs_not(this: &Object, args: Args) -> Result<Object> {
		this.call_attr_lit(AT_BOOL, args)?
		    .call_attr_lit(NOT, &[])
	}

	/// Get a hash of `this`.
	///
	/// This generates a unique hash per object, by taking the object's `ptr`'s hash.
	#[inline]
	pub fn qs_hash(this: &Object, _: Args) -> Result<Object> {
		Ok(this._ptr_hash().into())
	}

	/// Creates a clone of `this`.
	///
	/// This doesn't actually clone the underlying data—it marks it as "shared", and if it's modified
	/// it will be cloned.
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
	use crate::types::{Text, Boolean, Number};

	#[test]
	fn at_bool() {
		assert_contains!(Basic, "@bool");
		assert_downcast_eq!(Boolean; Basic::qs_at_bool(&Basic.into(), args!()).unwrap(), true);
		assert_downcast_eq!(Boolean; Basic::qs_at_bool(&Basic.into(), args!(false)).unwrap(), true);
	}

	#[test]
	fn at_text() {
		#[derive(Debug, Clone)]
		struct Dummy(i64);

		impl_object_type! { for Dummy [(parents Basic)]:
			"inspect" => function |this: &Object, _: Args| {
				this.try_downcast_map(|this: &Self| this.0.to_string())
			}
		}

		<Dummy as crate::types::ObjectType>::_wait_for_setup_to_finish();
		assert_contains!(Basic, "@text");

		let dummy = Object::from(Dummy(12));

		assert_downcast_eq!(Text; dummy.call_attr_lit(INSPECT, &[]).unwrap(), *"12");
		assert_downcast_eq!(Text; Basic::qs_at_text(&dummy, args!()).unwrap(), *"12");
		assert_downcast_eq!(Text; Basic::qs_at_text(&dummy, args!(13)).unwrap(), *"12");
	}

	#[test]
	fn eql() {
		assert_contains!(Basic, "==");

		let ref obj1 = Object::from(Basic);
		let ref obj2 = Object::from(Basic);

		assert_downcast_eq!(Boolean; Basic::qs_eql(obj1, args!(obj1.clone())).unwrap(), true);
		assert_downcast_eq!(Boolean; Basic::qs_eql(obj1, args!(obj2.clone())).unwrap(), false);


		assert_missing_parameter!(Basic::qs_eql(obj1, args!()), 0);
		assert_downcast_eq!(Boolean;
			Basic::qs_eql(obj1, args!(obj2.clone(), obj1.clone())).unwrap(), false);
	}

	#[test]
	fn neq() {

		#[derive(Debug, Clone, PartialEq)]
		struct Dummy(i64);

		impl_object_type! { for Dummy [(parents Basic)]:
			"==" => function |this: &Object, args: Args| {
				let rhs = args.arg(0)?;
				this.try_downcast_and_then(|this: &Self| {
					rhs.try_downcast_map(|rhs: &Self| {
						this == rhs
					})
				})
			}
		}

		<Dummy as crate::types::ObjectType>::_wait_for_setup_to_finish();
		assert_contains!(Basic, "!=");

		let ref obj1 = Object::from(Dummy(12));
		let ref obj2 = Object::from(Dummy(12));
		let ref obj3 = Object::from(Dummy(14));
		assert_downcast_eq!(Boolean; obj1.call_attr_lit(EQL, &[obj1]).unwrap(), true);
		assert_downcast_eq!(Boolean; obj1.call_attr_lit(EQL, &[obj2]).unwrap(), true);
		assert_downcast_eq!(Boolean; obj1.call_attr_lit(EQL, &[obj3]).unwrap(), false);

		assert_downcast_eq!(Boolean; Basic::qs_neq(obj1, args!(obj1.clone())).unwrap(), false);
		assert_downcast_eq!(Boolean; Basic::qs_neq(obj1, args!(obj2.clone())).unwrap(), false);
		assert_downcast_eq!(Boolean; Basic::qs_neq(obj1, args!(obj3.clone())).unwrap(), true);		

		assert_missing_parameter!(Basic::qs_eql(obj1, args!()), 0);
		assert_downcast_eq!(Boolean;
			Basic::qs_neq(obj1, args!(obj3.clone(), obj1.clone())).unwrap(), true);
	}

	#[test]
	fn not() {
		<Basic as crate::types::ObjectType>::_wait_for_setup_to_finish();

		#[derive(Debug, Clone, PartialEq)]
		struct Dummy(bool);

		impl_object_type! { for Dummy [(parents Basic)]:
			"@bool" => function |this: &Object, _: Args| {
				this.try_downcast_map(|this: &Self| Object::from(this.0))
			}
		}

		assert_contains!(Basic, "!");
		<Dummy as crate::types::ObjectType>::_wait_for_setup_to_finish();

		assert_downcast_eq!(Boolean; Basic::qs_not(&Dummy(true).into(), args!()).unwrap(), false);
		assert_downcast_eq!(Boolean; Basic::qs_not(&Dummy(false).into(), args!()).unwrap(), true);

		assert_downcast_eq!(Boolean; Basic::qs_not(&Dummy(false).into(), args!(true)).unwrap(), true);
	}

	#[test]
	fn hash() {
		assert_contains!(Basic, "hash");

		let ref obj1 = Object::from(Basic);
		let ref obj2 = Object::from(Basic);

		let hash1 = Basic::qs_hash(obj1, args!()).unwrap();
		// make sure repeated hashes are the same.
		assert_downcast_both_eq!(Number; hash1, Basic::qs_hash(obj1, args!()).unwrap());
		// make sure two hashes aren't identical
		assert_downcast_both_ne!(Number; hash1, Basic::qs_hash(obj2, args!()).unwrap());

		assert_downcast_both_eq!(Number; hash1, Basic::qs_hash(obj1, args!(obj2.clone())).unwrap());
	}

	#[test]
	fn clone() {
		#[derive(Debug, Clone, PartialEq)]
		struct Dummy(i32);

		impl_object_type! { for Dummy [(parents Basic)]:
			"==" => function |this: &Object, args: Args| {
				this.try_downcast_and_then(|this: &Self|
					args.arg(0)?.try_downcast_map(|rhs: &Self| this == rhs)
				)
			}
		}

		assert_contains!(Basic, "clone");
		<Dummy as crate::types::ObjectType>::_wait_for_setup_to_finish();

		let ref obj = Object::from(Dummy(12));
		let clone = Basic::qs_clone(obj, args!()).unwrap();

		assert!(!obj.is_identical(&clone));
		assert!(obj.eq_obj(&clone).unwrap());

	}
	#[test]
	fn itself() {
		<Basic as crate::types::ObjectType>::_wait_for_setup_to_finish();

		let ref obj = Object::from(Basic);
		assert!(obj.is_identical(&Basic::qs_itself(obj, args!()).unwrap()));
	}
}





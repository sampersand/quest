#![allow(unused)]
use crate::types::ObjectType;

use std::ops::{Deref, DerefMut};
use std::fmt::{Debug};

#[path="object/nanbox/mod.rs"]
pub mod nanbox;
use nanbox::Data;

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Object(Data);

impl Object {
	#[inline]
	pub fn new<T: ObjectType>(data: T) -> Self {
		Self(Data::from(data))
	}

	fn attrs(&self) -> &crate::obj::attributes::Attributes {
		self.0.attrs()
	}
}

use crate::Args;
use crate::error::{TypeError, KeyError};
use crate::types;
use crate::types::{Boolean};
use crate::literal::{EQL, Literal};

use std::any::{type_name};

use crate::obj::attributes as attributes;
pub(crate) use self::attributes::Value;

impl Object {
	/// Create a new object with the specified set of parents.
	///
	/// Note that `Parents` isn't publically visible from the outside world---this means that only
	/// `Object`, `Vec<Object>`, and `()` are allowed to be parents.
	#[inline]
	pub fn new_with_parent<T: 'static, P>(data: T, parents: P) -> Self
	where
		T: ObjectType,
		P: Into<attributes::Parents>
	{
		Self(Data::new_with_parents(data, parents))
	}


	/// Fetches the id of this object.
	///
	/// If two objects share the same id, they are identical.
	#[inline]
	pub fn id(&self) -> usize {
		self.attrs().id()
	}

	/// Fetches the name of the internal data.
	#[inline]
	pub fn typename(&self) -> &'static str {
		unimplemented!()
		// self.0.data.typename()
	}

	/// Checks to see if two objects are idental.
	#[inline]
	pub fn is_identical(&self, rhs: &Object) -> bool {
		self.0 == rhs.0
	}

	/// Compares two objects using [`==`](EQL) to see if they are equal
	pub fn eq_obj(&self, rhs: &Object) -> crate::Result<bool> {
		// self.call_attr_lit(EQL, &[rhs])
		// 	.map(|obj| obj.downcast::<Boolean>().map(|b| (*b).into_inner()).unwrap_or(false))
		unimplemented!()
	}

	/// Copies the actual data of the object.
	///
	/// When you [`clone()`] an [`Object`], you're actually just creating another reference to the
	/// same object in memory. This actually creates another distinct object.
	pub fn deep_clone(&self) -> Object {
		unimplemented!()
		// Object::from_parts(self.0.data.clone(), self.0.attrs.clone())
	}
}

// /// Methods to interact with the Object's data.
// #[allow(unreachable_code)]
// impl Object {
// 	/// Checks to see this object is a `T`.
// 	#[inline]
// 	pub fn is_a<T: ObjectType>(&self) -> bool {
// 		self.0.is_a::<T>()
// 	}

// 	#[inline]
// 	pub fn downcast<'a, T: ObjectType>(&'a self) -> Option<impl Deref<Target=T> + 'a> {
// 		self.0.downcast()
// 	}

// 	#[inline]
// 	pub fn downcast_mut<'a, T: ObjectType>(&'a self) -> Option<impl DerefMut<Target=T> + 'a> {
// 		self.0.downcast_mut()
// 	}

// 	pub fn try_downcast<'a, T: ObjectType>(&'a self) -> crate::Result<impl Deref<Target=T> + 'a> {
// 		self.downcast()
// 			.ok_or_else(|| TypeError::WrongType {
// 				expected: type_name::<T>(),
// 				got: self.typename()
// 			}.into())
// 	}

// 	pub fn try_downcast_mut<'a, T: ObjectType>(&'a self) -> crate::Result<impl DerefMut<Target=T> + 'a> {
// 		self.downcast_mut()
// 			.ok_or_else(|| TypeError::WrongType {
// 				expected: type_name::<T>(),
// 				got: self.typename()
// 			}.into())
// 	}
// }

// /// Methods to interact with the Object's attributes.
// ///
// /// Because you're able to assign arbitrary [`Object`]s as object fields, every function returns a
// /// [`Result<T>`](crate::Result) in case the `==` attribute on custom fields raises an error.
// ///
// /// The `xxx_lit` methods exist because it's _much_ faster to check if a `&str` exists compared to
// /// [`Object`]s.
// impl Object {
// 	/// Checks to see if the object has the attribute `attr`.
// 	#[inline]
// 	pub fn has_attr_lit(&self, attr: &str) -> crate::Result<bool> {
// 		self.0.has_lit(attr)
// 	}


// 	/// Fetches a value, returning `None` if it doesn't exist.
// 	fn get_value_lit(&self, attr: &str) -> crate::Result<Option<Value>> {
// 		self.0.get_lit(attr)
// 	}

// 	/// Fetches the attribute `attr`, returning a [`KeyError`] if it doesn't exist.
// 	pub fn get_attr_lit(&self, attr: &str) -> crate::Result<Self> {
// 		self.0.get_lit(attr)?
// 			.map(Self::from)
// 			.ok_or_else(|| KeyError::DoesntExist1 {
// 				attr: attr.to_string().into(), obj: self.clone() }.into())
// 	}

// 	/// Sets the attribute `attr` to `value`.
// 	pub fn set_value_lit<V>(&self, attr: Literal, value: V) -> crate::Result<()>
// 	where
// 		V: Into<Value>
// 	{
// 		// TODO: this will just set a literal value even if the corresponding nonliteral works.
// 		self.0.set_lit(attr, value.into());
// 		Ok(())
// 	}

// 	/// Assigns the attribute `attr` to `value`.
// 	#[inline]
// 	pub fn set_attr_lit(&self, attr: Literal, value: Self) -> crate::Result<()> {
// 		self.set_value_lit(attr, value)
// 	}

// 	/// Deletes the object corresponding to `attr`, returning [`KeyError`] if no such object existed.
// 	pub fn del_attr_lit(&self, attr: &str) -> crate::Result<Self> {
// 		self.0.del_lit(attr)
// 			.map(Self::from)
// 			.ok_or_else(|| KeyError::DoesntExist1 {
// 				attr: attr.to_string().into(), obj: self.clone() }.into())
// 	}

// 	/// Calls an attribute with the given args.
// 	pub fn call_attr_lit<'s, 'o: 's, A>(&'o self, attr: &str, args: A) -> crate::Result<Self>
// 	where
// 		A: Into<Args<'s, 'o>>
// 	{
// 		// self.get_value_lit(attr)?
// 		// 	.ok_or_else(|| KeyError::DoesntExist1 { attr: attr.to_string().into(), obj: self.clone() })?
// 		// 	.call(self, args.into())
// 		unimplemented!()
// 	}

// 	/// checks to see if the attribute exists
// 	#[inline]
// 	pub fn has_attr(&self, attr: &Self) -> crate::Result<bool> {
// 		self.0.has(attr)
// 	}

// 	/// Gets the attribute `attr`, returning `None` if it didn't exist
// 	#[inline]
// 	pub(crate) fn get_value(&self, attr: &Self) -> crate::Result<Option<Value>> {
// 		self.0.get(attr)
// 	}

// 	/// Gets an attribute, returning a [`KeyError`] if it doesn't exist.
// 	pub fn get_attr(&self, attr: &Self) -> crate::Result<Self> {
// 		self.get_value(attr)?
// 			.map(Self::from)
// 			.ok_or_else(|| KeyError::DoesntExist1 { attr: attr.clone(), 
// 				obj: self.clone() }.into())
// 	}

// 	/// Sets the attribute `attr` to `value`.
// 	#[inline]
// 	pub fn set_attr(&self, attr: Self, value: Self) -> crate::Result<()> {
// 		self.0.set(attr, value.into())
// 	}

// 	/// Deletes the attribute `attr`, returning a [`KeyError`] if the attr didn't exist.
// 	pub fn del_attr(&self, attr: &Object) -> crate::Result<Object> {
// 		self.0.del(attr)?
// 			.map(Object::from)
// 			.ok_or_else(|| KeyError::DoesntExist1 { attr: attr.clone(), obj: self.clone() }.into())
// 	}

// 	/// Calls the attribute `attr` with the given args, returning a [`KeyError`] if it doesn't exist.
// 	pub fn call_attr<'s, 'o: 's, A>(&'o self, attr: &Object, args: A) -> crate::Result<Object>
// 	where
// 		A: Into<Args<'s, 'o>>
// 	{
// 		self.get_value(attr)?
// 			.ok_or_else(|| KeyError::DoesntExist1 { attr: attr.clone(), obj: self.clone() })?
// 			.call(self, args.into())
// 	}

// 	/// This will probably be deprecated in the future 
// 	pub(crate) fn dot_get_attr(&self, attr: &Object) -> crate::Result<Object> {
// 		let result = self.get_attr(attr)?;

// 		// remove this hack? lol
// 		if result.is_a::<types::RustFn>() || format!("{:?}", result).starts_with("Object(Block") ||
// 				result.is_a::<types::BoundFunction>() {
// 			let bound_res = Object::new(crate::types::BoundFunction);
// 			bound_res.set_attr_lit("__bound_object_owner__", self.clone())?;
// 			bound_res.add_parent(result.clone())?;
// 			bound_res.set_attr_lit("__bound_object__", result)?;
// 			Ok(bound_res)
// 		} else {
// 			Ok(result)
// 		}
// 	}


// 	/// Dynamically add a new parent.
// 	///
// 	/// Generally, the [`new_with_parent()`] method is a better idea, as it creates a new obejct with
// 	/// parents.
// 	#[inline]
// 	pub fn add_parent(&self, val: Object) -> crate::Result<()> {
// 		self.attrs().add_parent(val)
// 	}

// 	/// Gets the list of keys corresponding to this object.
// 	#[inline]
// 	pub(crate) fn mapping_keys(&self, include_parents: bool) -> crate::Result<Vec<Object>> {
// 		self.attrs().keys(include_parents)
// 	}
// }


use crate::Args;
use crate::error::{TypeError, KeyError};
use crate::types::{self, ObjectType};
use crate::types::{Boolean};
use crate::literals::{EQL, Literal};

use std::sync::Arc;
use std::fmt::{self, Debug, Formatter};
use std::any::{Any, type_name};

mod data;
mod attributes;
use attributes::{Attributes, Value};
use data::Data;

/// The struct that represents any type within Quest.
#[derive(Clone)]
pub struct Object(Arc<Internal>);

impl Default for Object {
	#[inline]
	fn default() -> Self {
		Object::new(types::Null)
	}
}

struct Internal {
	/// The attributes (such as id, keys, and parents) of this object
	attrs: Attributes,
	/// The actual data of this object
	data: Data,
}

impl Debug for Object {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			f.debug_struct("Object")
				.field("data", &self.0.data)
				.field("attrs", &self.0.attrs)
				.finish()
		} else {
			f.debug_tuple("Object")
				.field(&self.0.data)
				.field(&self.0.attrs.id())
				.finish()
		}
	}
}


impl From<!> for Object {
	fn from(x: !) -> Self { x }
}

impl<T: Any + ObjectType> From<T> for Object {
	#[inline]
	fn from(data: T) -> Object {
		Object::new(data)
	}
}

impl Object {
	/// Create a new object with the specified set of parents.
	///
	/// Note that `Parents` isn't publically visible from the outside world---this means that only
	/// `Object`, `Vec<Object>`, and `()` are allowed to be parents.
	#[inline]
	pub fn new_with_parent<T, P>(data: T, parents: P) -> Self
	where
		T: Any + Debug + Send + Sync + Clone,
		P: Into<attributes::Parents>
	{
		// println!("creating new object: {:?} ({:?})", data, type_name<T>());
		Object::from_parts(Data::new(data), Attributes::new(parents))
	}

	#[inline]
	fn from_parts(data: Data, attrs: Attributes) -> Self {
		Object(Arc::new(Internal { data, attrs }))
	}

	/// Creates a new object with its default parents.
	#[inline]
	pub fn new<T: ObjectType>(data: T) -> Self {
		data.new_object()
	}

	/// Fetches the id of this object.
	///
	/// If two objects share the same id, they are identical.
	#[inline]
	pub fn id(&self) -> usize {
		self.0.attrs.id()
	}

	/// Fetches the name of the internal data.
	#[inline]
	pub fn typename(&self) -> &'static str {
		self.0.data.typename()
	}

	/// Checks to see if two objects are idental.
	#[inline]
	pub fn is_identical(&self, rhs: &Object) -> bool {
		Arc::ptr_eq(&self.0, &rhs.0)
	}

	/// Compares two objects using [`==`](EQL) to see if they are equal
	pub fn eq_obj(&self, rhs: &Object) -> crate::Result<bool> {
		self.call_attr_lit(EQL, &[rhs])
			.map(|obj| obj.downcast_and_then::<Boolean, _, _>(|b| b.into_inner()).unwrap_or(false))
	}

	/// Copies the actual data of the object.
	///
	/// When you [`clone()`] an [`Object`], you're actually just creating another reference to the
	/// same object in memory. This actually creates another distinct object.
	pub fn deep_clone(&self) -> Object {
		Object::from_parts(self.0.data.clone(), self.0.attrs.clone())
	}
}

/// Methods to interact with the Object's data.
impl Object {
	/// Checks to see this object is a `T`.
	#[inline]
	pub fn is_a<T: Any>(&self) -> bool {
		self.0.data.is_a::<T>()
	}

	/// Tries to downcast this object as a `T`, and if it is, calls `f`.
	///
	/// If the object isn't a `T`, a [`TypeError`] is returned.
	#[inline]
	pub fn try_downcast_map<T, O, F>(&self, f: F) -> crate::Result<O>
	where
		T: Any,
		F: FnOnce(&T) -> O,
	{
		self.try_downcast_and_then::<T, O, !, _>(|x| Ok(f(x)))
	}

	/// Tries to downcast this object as a mutable `T`, and if it is, calls `f`.
	///
	/// If the object isn't a `T`, a [`TypeError`] is returned.
	#[inline]
	pub fn try_downcast_mut_map<T, O, F>(&self, f: F) -> crate::Result<O>
	where
		T: Any,
		F: FnOnce(&mut T) -> O
	{
		self.try_downcast_mut_and_then::<T, O, !, _>(|x| Ok(f(x)))
	}

	/// Tries to downcast this object as a `T`, and if it is, calls `f`.
	///
	/// If the object isn't a `T`, a [`TypeError`] is returned.
	pub fn try_downcast_and_then<T, O, E, F>(&self, f: F) -> crate::Result<O>
	where
		T: Any,
		E: Into<crate::Error>,
		F: FnOnce(&T) -> Result<O, E>,
	{
		self.downcast_and_then(|opt| f(opt).map_err(Into::into))
			.unwrap_or_else(|| Err(TypeError::WrongType {
				expected: type_name::<T>(),
				got: self.typename()
			}.into()))
	}

	/// Tries to downcast this object as a mutable `T`, and if it is, calls `f`.
	///
	/// If the object isn't a `T`, a [`TypeError`] is returned.
	pub fn try_downcast_mut_and_then<T, O, E, F>(&self, f: F) -> crate::Result<O>
	where
		T: Any,
		E: Into<crate::Error>,
		F: FnOnce(&mut T) -> Result<O, E>
	{
		self.downcast_mut_and_then(|opt| f(opt).map_err(Into::into))
			.unwrap_or_else(|| Err(TypeError::WrongType {
				expected: type_name::<T>(),
				got: self.typename()
			}.into()))
	}

	/// Tries to downcast this object as a `T`, and if it is, calls `f`.
	///
	/// If the object isn't a `T`, `None` is returned.
	#[inline]
	pub fn downcast_and_then<T, R, F>(&self, f: F) -> Option<R>
	where
		T: Any,
		F: FnOnce(&T) -> R
	{
		self.0.data.downcast_and_then(|x| x.map(f))
	}

	/// Tries to downcast this object as a mutable `T`, and if it is, calls `f`.
	///
	/// If the object isn't a `T`, `None` is returned.
	#[inline]
	pub fn downcast_mut_and_then<T, R, F>(&self, f: F) -> Option<R>
	where
		T: Any,
		F: FnOnce(&mut T) -> R
	{
		self.0.data.downcast_mut_and_then(|x| x.map(f))
	}

	// this is soft deprecated
	#[inline]
	pub(crate) unsafe fn downcast_unchecked_and_then<T, R, F>(&self, f: F) -> R
	where
		T: Any, 
		F: FnOnce(&T) -> R
	{
		self.0.data.downcast_unchecked_and_then(f)
	}
}

/// Methods to interact with the Object's attributes.
///
/// Because you're able to assign arbitrary [`Object`]s as object fields, every function returns a
/// [`Result<T>`](crate::Result) in case the `==` attribute on custom fields raises an error.
///
/// The `xxx_lit` methods exist because it's _much_ faster to check if a `&str` exists compared to
/// [`Object`]s.
impl Object {
	/// Checks to see if the object has the attribute `attr`.
	#[inline]
	pub fn has_attr_lit(&self, attr: &str) -> crate::Result<bool> {
		self.0.attrs.has_lit(attr)
	}


	/// Fetches a value, returning `None` if it doesn't exist.
	fn get_value_lit(&self, attr: &str) -> crate::Result<Option<Value>> {
		self.0.attrs.get_lit(attr)
	}

	/// Fetches the attribute `attr`, returning a [`KeyError`] if it doesn't exist.
	pub fn get_attr_lit(&self, attr: &str) -> crate::Result<Object> {
		self.get_value_lit(attr)?
			.map(Object::from)
			.ok_or_else(|| KeyError::DoesntExist {
				attr: attr.to_string().into(), obj: self.clone() }.into())
	}

	/// Sets the attribute `attr` to `value`.
	pub fn set_value_lit<V>(&self, attr: Literal, value: V) -> crate::Result<()>
	where
		V: Into<Value>
	{
		// TODO: this will just set a literal value even if the corresponding nonliteral works.
		Ok(self.0.attrs.set_lit(attr, value.into()))
	}

	/// Assigns the attribute `attr` to `value`.
	#[inline]
	pub fn set_attr_lit(&self, attr: Literal, value: Object) -> crate::Result<()> {
		self.set_value_lit(attr, value)
	}

	/// Deletes the object corresponding to `attr`, returning [`KeyError`] if no such object existed.
	pub fn del_attr_lit(&self, attr: &str) -> crate::Result<Object> {
		self.0.attrs.del_lit(attr)
			.map(Object::from)
			.ok_or_else(|| KeyError::DoesntExist {
				attr: attr.to_string().into(), obj: self.clone() }.into())
	}

	/// Calls an attribute with the given args.
	pub fn call_attr_lit<'s, 'o: 's, A>(&'o self, attr: &str, args: A) -> crate::Result<Object>
	where
		A: Into<Args<'s, 'o>>
	{
		self.get_value_lit(attr)?
			.ok_or_else(|| KeyError::DoesntExist { attr: attr.to_string().into(), obj: self.clone() })?
			.call(self, args.into())
	}

	/// checks to see if the attribute exists
	#[inline]
	pub fn has_attr(&self, attr: &Object) -> crate::Result<bool> {
		self.0.attrs.has(attr)
	}

	/// Gets the attribute `attr`, returning `None` if it didn't exist
	#[inline]
	pub(crate) fn get_value(&self, attr: &Object) -> crate::Result<Option<Value>> {
		self.0.attrs.get(attr)
	}

	/// Gets an attribute, returning a [`KeyError`] if it doesn't exist.
	pub fn get_attr(&self, attr: &Object) -> crate::Result<Object> {
		self.get_value(attr)?
			.map(Object::from)
			.ok_or_else(|| KeyError::DoesntExist { attr: attr.clone(), obj: self.clone() }.into())
	}

	/// Sets the attribute `attr` to `value`.
	#[inline]
	pub fn set_attr(&self, attr: Object, value: Object) -> crate::Result<()> {
		self.0.attrs.set(attr, value.into())
	}

	/// Deletes the attribute `attr`, returning a [`KeyError`] if the attr didn't exist.
	pub fn del_attr(&self, attr: &Object) -> crate::Result<Object> {
		self.0.attrs.del(attr)?
			.map(Object::from)
			.ok_or_else(|| KeyError::DoesntExist { attr: attr.clone(), obj: self.clone() }.into())
	}

	/// Calls the attribute `attr` with the given args, returning a [`KeyError`] if it doesn't exist.
	pub fn call_attr<'s, 'o: 's, A>(&'o self, attr: &Object, args: A) -> crate::Result<Object>
	where
		A: Into<Args<'s, 'o>>
	{
		self.get_value(attr)?
			.ok_or_else(|| KeyError::DoesntExist { attr: attr.clone(), obj: self.clone() })?
			.call(self, args.into())
	}

	/// This will probably be deprecated in the future 
	pub(crate) fn dot_get_attr(&self, attr: &Object) -> crate::Result<Object> {
		let result = self.get_attr(attr)?;

		// remove this hack? lol
		if result.is_a::<types::RustFn>() || format!("{:?}", result).starts_with("Object(Block") ||
				result.is_a::<types::BoundFunction>() {
			let bound_res = Object::new(crate::types::BoundFunction);
			bound_res.set_attr_lit("__bound_object_owner__", self.clone())?;
			bound_res.add_parent(result.clone())?;
			bound_res.set_attr_lit("__bound_object__", result)?;
			Ok(bound_res)
		} else {
			Ok(result)
		}
	}


	/// Dynamically add a new parent.
	///
	/// Generally, the [`new_with_parent()`] method is a better idea, as it creates a new obejct with
	/// parents.
	#[inline]
	pub fn add_parent(&self, val: Object) -> crate::Result<()> {
		self.0.attrs.add_parent(val)
	}

	/// Gets the list of keys corresponding to this object.
	#[inline]
	pub(crate) fn mapping_keys(&self, include_parents: bool) -> crate::Result<Vec<Object>> {
		self.0.attrs.keys(include_parents)
	}
}

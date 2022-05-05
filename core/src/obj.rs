use crate::{Args, Literal};
use crate::error::{TypeError, KeyError};
use crate::types::{self, ObjectType, Boolean};

use std::sync::Arc;
use std::fmt::{self, Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::hash::Hash;
use std::borrow::Borrow;

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

impl<T: ObjectType> From<Option<T>> for Object {
	#[inline]
	fn from(data: Option<T>) -> Self {
		data.map(Self::new).unwrap_or_default()
	}
}

impl<T: ObjectType> From<T> for Object {
	#[inline]
	fn from(data: T) -> Self {
		Self::new(data)
	}
}

//-----------------------------------------------------------------------
// Object creation and metadata
//-----------------------------------------------------------------------

impl Internal {
	#[inline]
	fn id(&self) -> usize {
		self.attrs.id()
	}

	#[inline]
	fn typename(&self) -> &'static str {
		self.data.typename()
	}
}

impl Object {
	/// Create a new object with the specified set of parents.
	///
	/// Note that `Parents` isn't publically visible from the outside world---this means that only
	/// `Object`, `Vec<Object>`, and `()` are allowed to be parents.
	#[inline]
	pub fn new_with_parent<T: 'static, P>(data: T, parents: P) -> Self
	where
		T: Send + Sync + Clone + Debug,
		P: Into<attributes::Parents>
	{
		// println!("creating new object: {:?} ({:?})", data, type_name<T>());
		Self::from_parts(Data::new(data), Attributes::new(parents))
	}

	#[inline]
	fn from_parts(data: Data, attrs: Attributes) -> Self {
		Self(Arc::new(Internal { data, attrs }))
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
		self.0.id()
	}

	/// Fetches the name of the internal data.
	#[inline]
	pub fn typename(&self) -> &'static str {
		self.0.typename()
	}

	/// Checks to see if two objects are idental.
	#[inline]
	pub fn is_identical(&self, rhs: &Self) -> bool {
		Arc::ptr_eq(&self.0, &rhs.0)
	}

	/// Compares two objects using [`==`](EQL) to see if they are equal
	pub fn eq_obj(&self, rhs: &Self) -> crate::Result<bool> {
		self.call_attr_lit(&Literal::EQL, &[rhs])
			.map(|obj| obj.downcast::<Boolean>().map(|b| (*b).into_inner()).unwrap_or(false))
	}

	/// Copies the actual data of the object.
	///
	/// When you [`clone()`] an [`Object`], you're actually just creating another reference to the
	/// same object in memory. This actually creates another distinct object.
	pub fn deep_clone(&self) -> Self {
		Object::from_parts(self.0.data.clone(), self.0.attrs.clone())
	}
}

//-----------------------------------------------------------------------
// Interacting with object data
//-----------------------------------------------------------------------

impl Internal {
	#[inline]
	fn is_a<T: ObjectType>(&self) -> bool {
		self.data.is_a::<T>()
	}

	#[inline]
	fn downcast<'a, T: ObjectType>(&'a self) -> Option<impl Deref<Target=T> + 'a> {
		self.data.downcast()
	}

	#[inline]
	fn downcast_mut<'a, T: ObjectType>(&'a self) -> Option<impl DerefMut<Target=T> + 'a> {
		self.data.downcast_mut()
	}
}

/// Methods to interact with the Object's data.
impl Object {
	/// Checks to see this object is a `T`.
	#[inline]
	pub fn is_a<T: ObjectType>(&self) -> bool {
		self.0.is_a::<T>()
	}

	/// Attempts to downcast data to the given type, returning `None` if it's not the same type that
	/// was used to construct this object.
	#[inline]
	pub fn downcast<'a, T: ObjectType>(&'a self) -> Option<impl Deref<Target=T> + 'a> {
		self.0.downcast()
	}

	/// Attempts to mutably downcast data to the given type, returning `None` if it's not the same
	/// type that was used to construct this object.
	#[inline]
	pub fn downcast_mut<'a, T: ObjectType>(&'a self) -> Option<impl DerefMut<Target=T> + 'a> {
		self.0.downcast_mut()
	}

	/// The same as [`Object::downcast`], except this returns a [`TypeError`] instead of [`None`].
	pub fn try_downcast<'a, T: ObjectType>(&'a self) -> crate::Result<impl Deref<Target=T> + 'a> {
		self.downcast()
			.ok_or_else(|| TypeError::WrongType {
				expected: std::any::type_name::<T>(),
				got: self.typename()
			}.into())
	}

	/// The same as [`Object::downcast_mut`], except this returns a [`TypeError`] instead of [`None`].
	pub fn try_downcast_mut<'a, T: ObjectType>(&'a self) -> crate::Result<impl DerefMut<Target=T> + 'a> {
		self.downcast_mut()
			.ok_or_else(|| TypeError::WrongType {
				expected: std::any::type_name::<T>(),
				got: self.typename()
			}.into())
	}
}

impl Internal {
	#[inline]
	fn has_lit<L: ?Sized>(&self, attr: &L) -> crate::Result<bool>
	where
		Literal: Borrow<L>,
		L: Hash + Eq
	{
		self.attrs.has_lit(attr)
	}

	#[inline]
	fn get_lit<L: ?Sized>(&self, attr: &L) -> crate::Result<Option<Value>>
	where
		Literal: Borrow<L>,
		L: Hash + Eq + ToString
	{
		self.attrs.get_lit(attr)
	}

	#[inline]
	fn set_lit(&self, attr: impl Into<Literal>, value: impl Into<Value>) -> crate::Result<()> {
		self.attrs.set_lit(attr, value);
		Ok(())
	}

	#[inline]
	fn del_lit<L: ?Sized>(&self, attr: &L) -> crate::Result<Option<Value>>
	where
		Literal: Borrow<L>,
		L: Hash + Eq
	{
		Ok(self.attrs.del_lit(attr))
	}

	#[inline]
	fn has(&self, attr: &Object) -> crate::Result<bool> {
		self.attrs.has(attr)
	}

	#[inline]
	fn get(&self, attr: &Object) -> crate::Result<Option<Value>> {
		self.attrs.get(attr)
	}

	#[inline]
	fn set(&self, attr: Object, value: Value) -> crate::Result<()> {
		self.attrs.set(attr, value)
	}

	#[inline]
	fn del(&self, attr: &Object) -> crate::Result<Option<Value>> {
		self.attrs.del(attr)
	}

	#[inline]
	fn add_parent(&self, val: Object) -> crate::Result<()> {
		self.attrs.add_parent(val)
	}

	#[inline]
	fn prepend_parent(&self, val: Object) -> crate::Result<()> {
		self.attrs.prepend_parent(val)
	}

	#[inline]
	fn keys(&self, include_parents: bool) -> crate::Result<Vec<Object>> {
		self.attrs.keys(include_parents)
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
	// Implementation note: All these simply delegate to `self.0` as it will allow for easier
	// conversion of how objects are represented in the future.

	/// Checks to see if the object has the attribute `attr`.
	#[inline]
	pub fn has_attr_lit<L: ?Sized>(&self, attr: &L) -> crate::Result<bool>
	where
		Literal: Borrow<L>,
		L: Hash + Eq
	{
		self.0.has_lit(attr)
	}

	/// Fetches a value, returning `None` if it doesn't exist.
	fn get_value_lit<L: ?Sized>(&self, attr: &L) -> crate::Result<Option<Value>>
	where
		Literal: Borrow<L>,
		L: Hash + Eq + ToString
	{
		if let Some(value) = self.0.get_lit(attr)? {
			Ok(Some(value))
		} else if self.has_attr_lit::<Literal>(&Literal::__ATTR_MISSING__)? {
			// there's an inf recusion issue here but i cba to figure it out.
			self.call_attr_lit::<Literal, _>(&Literal::__ATTR_MISSING__, &[
				&attr.to_string().into()
			]).map(Value::Object).map(Some)
		} else {
			Ok(None)
		}
	}

	/// Fetches the attribute `attr`, returning a [`KeyError`] if it doesn't exist.
	pub fn get_attr_lit<L: ?Sized>(&self, attr: &L) -> crate::Result<Self>
	where
		Literal: Borrow<L>,
		L: Hash + Eq + ToString
	{
		self.get_value_lit(attr)?
			.map(Self::from)
			.ok_or_else(|| KeyError::DoesntExist {
				attr: attr.to_string().into(),
				obj: self.clone()
		}.into())
	}

	/// Sets the attribute `attr` to `value`.
	pub fn set_value_lit(&self, attr: impl Into<Literal>, value: impl Into<Value>)
		-> crate::Result<()>
	{
		// TODO: this will just set a literal value even if the corresponding nonliteral works.
		self.0.set_lit(attr, value)
	}

	/// Assigns the attribute `attr` to `value`.
	#[inline]
	pub fn set_attr_lit(&self, attr: impl Into<Literal>, value: Self) -> crate::Result<()> {
		self.set_value_lit(attr, value)
	}

	/// Deletes the object corresponding to `attr`, returning [`KeyError`] if no such object existed.
	pub fn del_attr_lit<L: ?Sized>(&self, attr: &L) -> crate::Result<Self>
	where
		Literal: Borrow<L>,
		L: Hash + Eq + ToString
	{
		self.0.del_lit(attr)?
			.map(Self::from)
			.ok_or_else(|| KeyError::DoesntExist {
				attr: attr.to_string().into(),
				obj: self.clone()
			}.into())
	}

	/// Calls an attribute with the given args.
	pub fn call_attr_lit<'s, 'o: 's, L, A>(&'o self, attr: &L, args: A) -> crate::Result<Self>
	where
		Literal: Borrow<L>,
		L: Hash + Eq + ToString + ?Sized,
		A: Into<Args<'s, 'o>>
	{
		self.get_value_lit(attr)?
			.ok_or_else(|| KeyError::DoesntExist {
				attr: attr.to_string().into(),
				obj: self.clone()
			})?
			.call(self, args.into())
	}

	/// checks to see if the attribute exists
	#[inline]
	pub fn has_attr(&self, attr: &Self) -> crate::Result<bool> {
		self.0.has(attr)
	}

	/// Gets the attribute `attr`, returning `None` if it didn't exist
	#[inline]
	pub(crate) fn get_value(&self, attr: &Self) -> crate::Result<Option<Value>> {
		if let Some(value) = self.0.get(attr)? {
			Ok(Some(value))
		} else {
			Ok(None)
		}
	}

	/// Gets an attribute, returning a [`KeyError`] if it doesn't exist.
	pub fn get_attr(&self, attr: &Self) -> crate::Result<Self> {
		if let Some(attr) = self.get_value(attr)?.map(Self::from) {
			Ok(attr) 
		} else if self.has_attr_lit(&Literal::__ATTR_MISSING__)? {
			self.call_attr_lit(&Literal::__ATTR_MISSING__, &[attr])
		} else {
			Err(KeyError::DoesntExist { attr: attr.clone(), obj: self.clone() }.into())
		}
	}

	/// Sets the attribute `attr` to `value`.
	#[inline]
	pub fn set_attr(&self, attr: Self, value: Self) -> crate::Result<()> {
		self.0.set(attr, value.into())
	}

	/// Deletes the attribute `attr`, returning a [`KeyError`] if the attr didn't exist.
	pub fn del_attr(&self, attr: &Self) -> crate::Result<Self> {
		self.0.del(attr)?
			.map(Self::from)
			.ok_or_else(|| KeyError::DoesntExist { attr: attr.clone(), obj: self.clone() }.into())
	}

	/// Calls the attribute `attr` with the given args, returning a [`KeyError`] if it doesn't exist.
	pub fn call_attr<'s, 'o: 's, A>(&'o self, attr: &Self, args: A) -> crate::Result<Self>
	where
		A: Into<Args<'s, 'o>>
	{
		self.get_value(attr)?
			.ok_or_else(|| KeyError::DoesntExist { attr: attr.clone(), obj: self.clone() })?
			.call(self, args.into())
	}

	/// This will probably be deprecated in the future 
	pub(crate) fn dot_get_attr(&self, attr: &Self) -> crate::Result<Self> {
		let result = self.get_attr(attr)?;

		// assert_eq!(
		// 	result.is_a::<types::RustFn>() || result.is_a::<types::RustClosure>() || 
		// 		result.is_a::<types::BoundFunction>() ||
		// 		format!("{:?}", result).starts_with("Object(Block"),
		// 	result.is_a::<types::RustFn>() || result.is_a::<types::RustClosure>() || 
		// 		result.is_a::<types::BoundFunction>() ||
		// 		result.typename().contains("::Block"));
		if result.is_a::<types::RustFn>() || result.is_a::<types::RustClosure>() || 
				result.is_a::<types::BoundFunction>() ||
				result.typename().contains("::Block") {
			let bound_res = Self::new(crate::types::BoundFunction);
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
	/// Generally, the [`new_with_parent()`] method is a better idea, as it creates a new object with
	/// parents.
	#[inline]
	pub fn add_parent(&self, val: Self) -> crate::Result<()> {
		self.0.add_parent(val)
	}

	///
	/// Generally, the [`new_with_parent()`] method is a better idea, as it creates a new object with
	/// parents.
	#[inline]
	pub fn prepend_parent(&self, val: Self) -> crate::Result<()> {
		self.0.prepend_parent(val)
	}

	/// Gets the list of keys corresponding to this object.
	#[inline]
	pub(crate) fn mapping_keys(&self, include_parents: bool) -> crate::Result<Vec<Self>> {
		self.0.keys(include_parents)
	}
}

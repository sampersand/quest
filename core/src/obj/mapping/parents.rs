use crate::{Result, Object, types};
use std::fmt::{self, Debug, Formatter};
use std::sync::RwLock;

pub struct Parents(RwLock<Inner>);

#[derive(Clone)]
enum Inner {
	Object(Object),
	List(Vec<Object>),
	None
}

impl Debug for Parents {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match &*self.0.read().expect("can't read") {
			Inner::Object(obj) => write!(f, "{:?}", obj),
			Inner::List(list) => write!(f, "{:?}", list),
			Inner::None => write!(f, "[]")
		}
	}
}

impl Clone for Parents {
	fn clone(&self) -> Self {
		Parents::new(self.0.read().expect("can't read").clone())
	}
}

impl Default for Parents {
	fn default() -> Self {
		Parents::new(Inner::None)
	}
}

impl From<Vec<Object>> for Parents {
	fn from(list: Vec<Object>) -> Self {
		Parents::new(Inner::List(list))
	}
}

impl From<Object> for Parents {
	fn from(object: Object) -> Self {
		// TODO: this is a hack and should be removed; this will remove all mappings from
		// user-provided lists.
		{
			if let Some(obj) = object.downcast_ref::<crate::types::List>()  {
				return Parents::new(Inner::List(obj.clone().into()))
			}
		}

		Parents::new(Inner::Object(object))
	}
}

impl From<()> for Parents {
	fn from(_: ()) -> Self {
		Self::default()
	}
}

impl Parents {
	fn new(inner: Inner) -> Self {
		Parents(RwLock::new(inner))
	}

	pub fn add_parent(&mut self, parent: Object) -> Result<()> {
		let mut inner = self.0.write().expect("can't write");

		match &mut *inner {
			Inner::Object(object) => { object.call_attr("push", &[&parent])?; },
			Inner::List(ref mut list) => list.push(parent),
			Inner::None => *inner = Inner::List(vec![parent])
		};

		Ok(())
	}

	pub fn as_object(&self) -> Object {
		let inner = self.0.read().expect("can't read");

		if let Inner::Object(object) = &*inner {
			return object.clone();
		}

		drop(inner);
		let mut inner = self.0.write().expect("can't write");

		match &mut *inner {
			// in case someone else updated it before we acquired the read lock
			Inner::Object(object) => object.clone(),
			Inner::List(list) => {
				let list = Object::from(std::mem::take(list));
				*inner = Inner::Object(list.clone());
				list
			},
			Inner::None => {
				let list = Object::from(vec![]);
				*inner = Inner::Object(list.clone());
				list
			},
		}
	}

	pub fn iter<'a>(&'a self) -> Result<impl Iterator<Item=Object> + 'a> {
		enum ParentsIter<'a> {
			#[allow(unused)]
			List(std::slice::Iter<'a, Object>),
			#[allow(unused)]
			Object(std::vec::IntoIter<Object>),
			None,
		}

		impl<'a> Iterator for ParentsIter<'a> {
			type Item = Object;
			fn next(&mut self) -> Option<Self::Item> {
				match self {
					ParentsIter::None => None,
					ParentsIter::List(_l) => unimplemented!(),//l.next(),
					ParentsIter::Object(l) => l.next()//.as_ref()
				}
			}
		}

		Ok(match &*self.0.read().unwrap() {
			Inner::Object(obj) => ParentsIter::Object(
				Vec::from(obj.downcast_call::<types::List>()?).into_iter()
			),
			// TODO: not make this `to_vec`
			Inner::List(list) => ParentsIter::Object(list.to_vec().into_iter()),
			Inner::None => ParentsIter::None
		})
	}
}

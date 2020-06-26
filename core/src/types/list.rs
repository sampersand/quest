use crate::Object;
use std::fmt::{self, Debug, Formatter};


type Inner = Vec<Object>;

#[derive(Clone)]
pub struct List(Inner);

impl Debug for List {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			write!(f, "List({:?})", self.as_ref())
		} else {
			Debug::fmt(&self.as_ref(), f)
		}
	}
}

impl IntoIterator for List {
	type Item = <Vec<Object> as IntoIterator>::Item;
	type IntoIter = <Vec<Object> as IntoIterator>::IntoIter;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}

}

impl List {
	pub fn new(list: Inner) -> Self {
		List(list)
	}
}

impl From<List> for Inner {
	fn from(list: List) -> Self {
		list.0
	}
}

impl From<Inner> for List {
	fn from(list: Inner) -> Self {
		List::new(list)
	}
}

impl From<Inner> for Object {
	fn from(list: Inner) -> Self {
		List::from(list).into()
	}
}

impl AsRef<[Object]> for List {
	fn as_ref(&self) -> &[Object] {
		self.0.as_ref()
	}
}

mod impls {
	use super::List;
	use crate::{Object, Result, Args, types};

	pub fn at_text(args: Args) -> Result<Object> {
		let this = args.this()?.try_downcast_ref::<List>()?;
		let mut l = vec![];
		for item in this.0.iter() {
			l.push(item.downcast_call::<types::Text>()?.as_ref().to_string());
		}
		Ok(format!("[{}]", l.join(", ")).into())
	}

	pub fn at_bool(args: Args) -> Result<Object> {
		let this = args.this()?;
		this.call_attr("len", args.clone())?
			.call_attr("@bool", vec![])
	}

	pub fn at_map(_args: Args) -> Result<Object> {
		todo!("List::at_map");
	}

	pub fn at_list(args: Args) -> Result<Object> { // "@list"
		let this = args.this()?;
		this.call_attr("clone", args.clone())
	}

	pub fn clone(args: Args) -> Result<Object> {
		let this = args.this()?.try_downcast_ref::<List>()?;
		Ok(this.clone().into())
	}

	pub fn does_include(_args: Args) -> Result<Object> {
		todo!("List::does_include");
	}

	pub fn index_of(_args: Args) -> Result<Object> {
		todo!("List::index_of");
	}

	pub fn clear(args: Args) -> Result<Object> {
		let mut this = args.this()?.try_downcast_mut::<List>()?;
		this.0.clear();
		Ok(args.this()?.clone())
	}

	pub fn len(args: Args) -> Result<Object> {
		let this = args.this()?.try_downcast_ref::<List>()?;
		Ok(this.0.len().into())
	}

	fn correct_index(idx: isize, len: usize) -> Result<Option<usize>> {
		if idx.is_positive() {
			let idx = (idx - 1) as usize;
			if idx < len {
				Ok(Some(idx))
			} else {
				Ok(None)
			}
		} else if idx.is_negative() {
			let idx = (-idx) as usize;
			if idx <= len {
				Ok(Some(len - idx))
			} else {
				Ok(None)
			}
		} else {
			Err(crate::error::KeyError::CantIndexByZero.into())
		}
	}

	pub fn index(args: Args) -> Result<Object> {
		let this = args.this()?.try_downcast_ref::<List>()?;

		let len = this.0.len();
		let start = args.arg(0)?
			.try_downcast_ref::<types::Number>()?
			.floor() as isize;
		let end = args.arg(1)
			.ok()
			.map(Object::downcast_call::<types::Number>)
			.transpose()?
			.map(|x| x.floor() as isize);

		let start =
			if let Some(start) = correct_index(start, len)? {
				start
			} else {
				return Ok(Object::default())
			};

		match end {
			None => Ok(this.0[start].clone()),
			Some(end) => {
				let end = correct_index(end, len)?.map(|x| x + 1).unwrap_or(len);
				if end < start {
					Ok(Object::default())
				} else {
					Ok(this.0[start..end].to_owned().into())
				}
			}
		}
	}
	pub fn index_assign(_args: Args) -> Result<Object> {
		todo!("index_assign")
	}

	pub fn join(args: Args) -> Result<Object> {
		let this = args.this()?.try_downcast_ref::<List>()?;
		let joiner = 
			match args.arg(0) {
				Ok(arg) => arg.downcast_call::<types::Text>()?.as_ref().to_string(),
				Err(_) => "".to_string()
			};

		Ok(this.0.iter()
			.map(|obj| obj.downcast_call::<types::Text>()
				.map(|txt| txt.as_ref().to_string()))
			.collect::<Result<Vec<_>>>()?
			.join(joiner.as_ref()).into())
	}

	pub fn add(args: Args) -> Result<Object> {
		let this = args.this()?;
		this.call_attr("clone", vec![])?
			.call_attr("+=", args.args(..)?)
	}

	pub fn add_assign(args: Args) -> Result<Object> {
		let this = args.this()?;
		let rhs = args.arg(0)?.downcast_call::<List>()?;

		#[allow(clippy::redundant_clone)]
		this.try_downcast_mut::<List>()?.0.append(&mut rhs.clone().0);
		Ok(this.clone())
	}

	pub fn push(args: Args) -> Result<Object> {
		let this = args.this()?;
		let rhs = args.arg(0)?;
		this.try_downcast_mut::<List>()?.0.push(rhs.clone());
		Ok(this.clone())
	}

	pub fn pop(args: Args) -> Result<Object> {
		let this = args.this()?;
		Ok(this.try_downcast_mut::<List>()?.0.pop().unwrap_or_default())
	}

	pub fn unshift(args: Args) -> Result<Object> {
		let this = args.this()?;
		let rhs = args.arg(0)?;
		this.try_downcast_mut::<List>()?.0.insert(0, rhs.clone());
		Ok(this.clone())
	}
	pub fn shift(args: Args) -> Result<Object> {
		let this = &mut args.this()?.try_downcast_mut::<List>()?.0;
		if this.is_empty() {
			Ok(Object::default())
		} else {
			Ok(this.remove(0))
		}
	}

	pub fn intersect(_args: Args) -> Result<Object> {
		todo!("List::intersect");
	}

	pub fn union(_args: Args) -> Result<Object> {
		todo!("List::union");
	}

	pub fn not_shared(_args: Args) -> Result<Object> {
		todo!("List::not_shared");
	}

	pub fn difference(_args: Args) -> Result<Object> {
		todo!("List::difference");
	}

}


impl_object_type!{
for List [(parents super::Basic) (convert "@list")]:
	"@text" => impls::at_text,
	"@bool" => impls::at_bool,
	"@map" => impls::at_map,
	"@list" => impls::at_list,
	"clone" => impls::clone,

	"does_include" => impls::does_include,
	"clear" => impls::clear,
	"index_of" => impls::index_of,
	"len" => impls::len,
	"[]" => impls::index,
	"[]=" => impls::index_assign,
	"get" => impls::index,
	"join" => impls::join,
	"<<" => impls::push,
	"push" => impls::push,
	"pop" => impls::pop,
	"unshift" => impls::unshift,
	"shift" => impls::shift,

	"+" => impls::add,
	"+=" => impls::add_assign,
	"&" => impls::intersect,
	"|" => impls::union,
	"^" => impls::not_shared,
	"-" => impls::difference
}


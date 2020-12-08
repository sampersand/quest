use crate::{Object, Args};
use crate::types::{RustClosure, Boolean};
use tracing::instrument;
use std::sync::Arc;
use parking_lot::Mutex;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Iterable;

fn foreach<F>(this: &Object, func: F) -> crate::Result<Object>
where
	F: Fn(Args, &mut Vec<Object>) -> crate::Result<Object> + Send + Sync + 'static
{
	let ret = Arc::new(Mutex::new(vec![]));
	let ret_clone = ret.clone();

	let closure = RustClosure::new(move |args| (func)(args, &mut ret_clone.lock()));

	this.call_attr_lit("each", &[&closure.into()])?;

	match Arc::try_unwrap(ret) {
		// no one else has a reference, so we're all good.
		Ok(mutex) => Ok(mutex.into_inner().into()),
		// we have to clone it now. darn!
		Err(arc) => Ok(arc.lock().clone().into())
	}
}

impl Iterable {
	#[instrument(name="Iterable::map", level="trace", skip(this, args), fields(self = ?this, ?args))]
	pub fn qs_map(this: &Object, args: Args) -> crate::Result<Object> {
		let block = args.try_arg(0)?.clone();

		foreach(this, move |args, list| {
			let mapped = block.call_attr_lit("()", args.shorten())?;
			list.push(mapped.clone());
			Ok(mapped)
		})
	}

	#[instrument(name="Iterable::zip", level="trace", skip(this, args), fields(self = ?this, ?args))]
	pub fn qs_zip(this: &Object, args: Args) -> crate::Result<Object> {
		// TODO: actually call `each` on the RHS, as rn it makes the rhs into a list, which fails with infinite iterators.
		let rhs = Mutex::new(args.try_arg(0)?.call_downcast::<crate::types::List>()?.clone().into_iter());

		foreach(this, move |args, list| {
			let val: Object = vec![args.try_arg(0)?.clone(), rhs.lock().next().map(|x| x.clone()).unwrap_or_default()].into();
			list.push(val.clone());
			Ok(val)
		})
	}

	#[instrument(name="Iterable::enumerate", level="trace", skip(this, args), fields(self = ?this, ?args))]
	pub fn qs_enumerate(this: &Object, args: Args) -> crate::Result<Object> {
		let block = args.try_arg(0)?.clone();

		foreach(this, move |args, list| {
			let mut args = args.shorten();
			let len = Object::from(list.len());
			args.push(&len);
			let mapped = block.call_attr_lit("()", args)?;
			list.push(mapped.clone());
			Ok(mapped)
		})
	}


	#[instrument(name="Iterable::count", level="trace", skip(this, args), fields(self = ?this, ?args))]
	pub fn qs_count(this: &Object, args: Args) -> crate::Result<Object> {
		Ok(Self::qs_select(this, args)?
			.downcast::<crate::types::List>()
			.unwrap()
			.len()
			.into())
	}

	#[instrument(name="Iterable::select", level="trace", skip(this, args), fields(self = ?this, ?args))]
	pub fn qs_select(this: &Object, args: Args) -> crate::Result<Object> {
		let block = args.try_arg(0)?.clone();

		foreach(this, move |args, list| {
			let obj = args.try_arg(0)?.clone();

			let should_keep = block.call_attr_lit("()", args.shorten())?;

			if should_keep.call_downcast::<Boolean>()?.into_inner() {
				list.push(obj);
			}

			Ok(should_keep) // or should it be `obj`?
		})
	}

	#[instrument(name="Iterable::reject", level="trace", skip(this, args), fields(self = ?this, ?args))]
	pub fn qs_reject(this: &Object, args: Args) -> crate::Result<Object> {
		let block = args.try_arg(0)?.clone();

		foreach(this, move |args, list| {
			let obj = args.try_arg(0)?.clone();

			let should_keep = block.call_attr_lit("()", args.shorten())?;

			if !should_keep.call_downcast::<Boolean>()?.into_inner() {
				list.push(obj);
			}

			Ok(should_keep) // or should it be `obj`?
		})
	}

	#[instrument(name="Iterable::chunk_while", level="trace", skip(this, args), fields(self = ?this, ?args))]
	pub fn qs_chunk_while(this: &Object, args: Args) -> crate::Result<Object> {
		let block = args.try_arg(0)?.clone();

		foreach(this, move |args, list| {
			let obj = args.try_arg(0)?.clone();

			let chunked = block.call_attr_lit("()", args.shorten())?;

			if list.is_empty() || !chunked.call_downcast::<Boolean>()?.into_inner() {
				list.push(Vec::new().into());
			}

			list.last().unwrap().call_attr_lit("push", &[&obj])?;

			Ok(chunked) // or should it be `obj`?
		})
	}

	#[instrument(name="Iterable::reduce", level="trace", skip(this, args), fields(self = ?this, ?args))]
	pub fn qs_reduce(this: &Object, args: Args) -> crate::Result<Object> {
		let block = args.try_arg(0)?.clone();

		foreach(this, move |args, list| {
			let obj = args.try_arg(0)?.clone();

			let chunked = block.call_attr_lit("()", args.shorten())?;

			if list.is_empty() || !chunked.call_downcast::<Boolean>()?.into_inner() {
				list.push(Vec::new().into());
			}

			list.last().unwrap().call_attr_lit("push", &[&obj])?;

			Ok(chunked) // or should it be `obj`?
		})
	}
}

impl_object_type!{
for Iterable [(parents super::Basic)]:
	"map" => method Self::qs_map,
	"enumerate" => method Self::qs_enumerate,
	"select" => method Self::qs_select,
	// "reduce" => method Self::qs_reduce,
	"reject" => method Self::qs_reject,
	"count" => method Self::qs_count,
	"zip" => method Self::qs_zip,
	"chunk_while" => method Self::qs_chunk_while,
	// "all" => method Self::qs_all,
	// "any" => method Self::qs_any,
}

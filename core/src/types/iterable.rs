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
}

impl_object_type!{
for Iterable [(parents super::Basic)]:
	"map" => method Self::qs_map,
	"enumerate" => method Self::qs_enumerate,
	"select" => method Self::qs_select,
	"reject" => method Self::qs_reject,
	"count" => method Self::qs_count,
}


 // => [:each_slice, :each_cons, :each_with_object, :zip, :take, :take_while, :drop, :drop_while, :cycle, :chunk, :slice_before, :slice_after, :slice_when, :chunk_while, :sum, :uniq, :chain, :lazy, :to_set, :to_h, :include?, :max, :min, :to_a, :find, :entries, :sort, :sort_by, :grep, :grep_v, :count, :detect, :find_index, :find_all, :select, :filter, :filter_map, :reject, :collect, :map, :flat_map, :collect_concat, :inject, :reduce, :partition, :group_by, :tally, :first, :all?, :any?, :one?, :none?, :minmax, :min_by, :max_by, :minmax_by, :member?, :each_with_index, :reverse_each, :each_entry] 

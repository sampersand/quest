mod iter;
pub use iter::{Iter, StopIteration};

use tracing::instrument;
use crate::error::ArgumentError;
use crate::{Object, Args, Literal};
use crate::types::{Boolean, Number};
// use crate::types::Boolean;

/// The class that represents types that can be iterated over.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Iterable;

impl Iterable {
	/// Finishes the iterable and converts it to a [`List`].
	#[instrument(name="Iterable::@list", level="trace", skip(this), fields(self=?this))]
	pub fn qs_at_list(this: &Object, _: Args) -> crate::Result<Object> {
		this.call_downcast::<Iter>()?
			.clone()
			.try_into_list()
			.map(Object::from)
	}

	/// Finishes the iterable and converts it to a [`List`].
	#[instrument(name="Iterable::@list", level="trace", skip(this), fields(self=?this))]
	pub fn qs_run(this: &Object, _: Args) -> crate::Result<Object> {
		this.call_downcast::<Iter>()?
			.clone()
			.run()
			.map(|_| Object::default())
	}

	/// Enumerates the iterable by returning `[ele, idx]`
	#[instrument(name="Iterable::enumerate", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_enumerate(this: &Object, args: Args) -> crate::Result<Object> {
		let iter = this.call_downcast::<Iter>()?.clone();

		Ok(iter.enumerate().into())
	}

	// Returns a new [`Iter`], where each element is `args[0](this.next())`
	#[instrument(name="Iterable::map", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_map(this: &Object, args: Args) -> crate::Result<Object> {
		let iter = this.call_downcast::<Iter>()?.clone();
		let block = args.try_arg(0)?.clone();

		Ok(iter.map(move |obj| block.call_attr_lit(&Literal::CALL, &[&obj])).into())
	}

	// Calls the block with each element, then returns the element.
	#[instrument(name="Iterable::each", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_each(this: &Object, args: Args) -> crate::Result<Object> {
		let iter = Self::qs_eachl(this, args)?;
		Self::qs_run(&iter, Args::default()).and(Ok(iter))
	}

	// Calls the block with each element, then returns the element.
	// this version is lazy, and doesn't actually evaluate it.
	#[instrument(name="Iterable::eachl", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_eachl(this: &Object, args: Args) -> crate::Result<Object> {
		let iter = this.call_downcast::<Iter>()?.clone();
		let block = args.try_arg(0)?.clone();

		Ok(iter.each(move |obj| block.call_attr_lit(&Literal::CALL, &[obj]).and(Ok(()))).into())
	}

	#[instrument(name="Iterable::select", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_select(this: &Object, args: Args) -> crate::Result<Object> {
		let iter = this.call_downcast::<Iter>()?.clone();
		let block = args.try_arg(0)?.clone();

		Ok(iter.select(move |obj| 
			block.call_attr_lit(&Literal::CALL, &[obj])?
				.call_downcast::<Boolean>()
				.map(|x| x.into_inner())
		).into())
	}

	#[instrument(name="Iterable::reject", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_reject(this: &Object, args: Args) -> crate::Result<Object> {
		let iter = this.call_downcast::<Iter>()?.clone();
		let block = args.try_arg(0)?.clone();

		Ok(iter.select(move |obj| 
			block.call_attr_lit(&Literal::CALL, &[obj])?
				.call_downcast::<Boolean>()
				.map(|x| !x.into_inner())
		).into())
	}

	#[instrument(name="Iterable::reduce", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_reduce(this: &Object, args: Args) -> crate::Result<Object> {
		let mut iter = this.call_downcast::<Iter>()?.clone();

		let start;
		let block;

		if let Some(blk) = args.arg(1) {
			start = args.arg(0).unwrap().clone();
			block = blk;
		} else if let Some(init) = iter.next() {
			start = init?;
			block = args.try_arg(0)?;
		} else {
			return Ok(Object::default())
		}

		iter.reduce(start, move |acc, next| block.call_attr_lit(&Literal::CALL, &[&acc, &next]))
	}

	/// Zip as many arguments as given in `args` into one array.
	#[instrument(name="Iterable::zip", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_zip(this: &Object, args: Args) -> crate::Result<Object> {
		// if we have nothing to zip, just return the original thing.
		if args.len() <= 1 {
			return Ok(this.clone());
		}

		let this = this.call_downcast::<Iter>()?.clone();

		let zippers =
			args.as_ref()
				.iter()
				.map(|arg| arg.call_downcast::<Iter>().map(|iter| iter.clone()))
				.collect::<crate::Result<Vec<_>>>()?;

		Ok(this.zip(zippers).into())
	}

	/// Only return a maximum of `n` elements, where `n` is the first argument.
	#[instrument(name="Iterable::take", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_take(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.call_downcast::<Iter>()?.clone();
		let amnt = args.try_arg(0)?.call_downcast::<Number>()?.truncate();

		if amnt < 0 {
			Err(ArgumentError::Messaged(format!("negative number given to take: '{}'", amnt)).into())
		} else {
			Ok(this.take(amnt as usize).into())
		}
	}

	/// Only take elements while the give block (ie first arg) evaluates to true.
	#[instrument(name="Iterable::take_while", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_take_while(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.call_downcast::<Iter>()?.clone();
		let block = args.try_arg(0)?.clone();

		Ok(this.take_while(move |obj| {
			block.call_attr_lit(&Literal::CALL, &[obj])?
				.call_downcast::<Boolean>()
				.map(|boolean| boolean.into_inner())
		}).into())
	}

	/// Only take elements while the give block (ie first arg) evaluates to false.
	#[instrument(name="Iterable::take_until", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_take_until(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.call_downcast::<Iter>()?.clone();
		let block = args.try_arg(0)?.clone();

		Ok(this.take_while(move |obj| {
			block.call_attr_lit(&Literal::CALL, &[obj])?
				.call_downcast::<Boolean>()
				.map(|boolean| !boolean.into_inner())
		}).into())
	}

	/// Ignore the first `n` elements (ie the first argument)
	#[instrument(name="Iterable::drop", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_drop(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.call_downcast::<Iter>()?.clone();
		let amnt = args.try_arg(0)?.call_downcast::<Number>()?.truncate();

		if amnt < 0 {
			Err(ArgumentError::Messaged(format!("negative number given to drop: '{}'", amnt)).into())
		} else {
			Ok(this.drop(amnt as usize).into())
		}
	}

	/// Ignore elements while the give block (ie first arg) evaluates to true.
	#[instrument(name="Iterable::drop_while", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_drop_while(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.call_downcast::<Iter>()?.clone();
		let block = args.try_arg(0)?.clone();

		Ok(this.drop_while(move |obj| {
			block.call_attr_lit(&Literal::CALL, &[obj])?
				.call_downcast::<Boolean>()
				.map(|boolean| boolean.into_inner())
		}).into())
	}

	/// Ignore elements while the given block (ie first arg) evaluates to false
	#[instrument(name="Iterable::drop_until", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_drop_until(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.call_downcast::<Iter>()?.clone();
		let block = args.try_arg(0)?.clone();

		Ok(this.drop_while(move |obj| {
			block.call_attr_lit(&Literal::CALL, &[obj])?
				.call_downcast::<Boolean>()
				.map(|boolean| !boolean.into_inner())
		}).into())
	}

	/// Group every `n` (ie the first argument) into an array.
	#[instrument(name="Iterable::chunk", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_chunk(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.call_downcast::<Iter>()?.clone();
		let amnt = args.try_arg(0)?.call_downcast::<Number>()?.truncate();

		if amnt < 0 {
			Err(ArgumentError::Messaged(format!("negative number given to chunk: '{}'", amnt)).into())
		} else {
			Ok(this.chunk(amnt as usize).into())
		}
	}

	/// Chunk elements while the given block (ie first argument) evaluates to truncate.
	#[instrument(name="Iterable::chunk_while", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_chunk_while(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.call_downcast::<Iter>()?.clone();
		let block = args.try_arg(0)?.clone();

		Ok(this.chunk_while(move |obj| {
			block.call_attr_lit(&Literal::CALL, &[obj])?
				.call_downcast::<Boolean>()
				.map(|boolean| boolean.into_inner())
		}).into())
	}

	/// Chunk elements while the given block (ie first argument) evaluates to false.
	#[instrument(name="Iterable::chunk_until", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_chunk_until(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.call_downcast::<Iter>()?.clone();
		let block = args.try_arg(0)?.clone();

		Ok(this.chunk_while(move |obj| {
			block.call_attr_lit(&Literal::CALL, &[obj])?
				.call_downcast::<Boolean>()
				.map(|boolean| !boolean.into_inner())
		}).into())
	}

	#[instrument(name="Iterable::windows", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_windows(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.call_downcast::<Iter>()?.clone();
		let amnt = args.try_arg(0)?.call_downcast::<Number>()?.truncate();

		if amnt <= 0 {
			Err(ArgumentError::Messaged(format!("nonpositive number given to windows: '{}'", amnt)).into())
		} else {
			Ok(this.windows(amnt as usize).into())
		}
	}

	#[instrument(name="Iterable::cycle", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_cycle(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.call_downcast::<Iter>()?.clone();

		Ok(this.cycle(
			if let Some(arg) = args.arg(0) {
				let amnt = arg.call_downcast::<Number>()?.truncate();

				if amnt < 0 {
					return Err(ArgumentError::Messaged(format!("negative number given to cycle: '{}'", amnt)).into());
				} else {
					Some(amnt as usize)
				}
			} else {
				None
			}
		).into())
	}

	#[instrument(name="Iterable::group_by", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_group_by(this: &Object, args: Args) -> crate::Result<Object> {
		todo!();
	}

	#[instrument(name="Iterable::chain", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_chain(this: &Object, args: Args) -> crate::Result<Object> {
		todo!();
	}

	#[instrument(name="Iterable::flatten", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_flatten(this: &Object, args: Args) -> crate::Result<Object> {
		todo!();
	}

	#[instrument(name="Iterable::find_first", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_find_first(this: &Object, args: Args) -> crate::Result<Object> {
		todo!();
	}


	#[instrument(name="Iterable::sum", level="trace", skip(this), fields(self=?this))]
	pub fn qs_sum(this: &Object, _: Args) -> crate::Result<Object> {
		let mut this = this.call_downcast::<Iter>()?.clone();

		if let Some(init) = this.next() {
			this.reduce(init?, |acc, new| acc.call_attr_lit("+", &[&new]))
		} else {
			Ok(0.into())
		}
	}

	#[instrument(name="Iterable::prod", level="trace", skip(this), fields(self=?this))]
	pub fn qs_prod(this: &Object, _: Args) -> crate::Result<Object> {
		let mut this = this.call_downcast::<Iter>()?.clone();

		if let Some(init) = this.next() {
			this.reduce(init?, |acc, new| acc.call_attr_lit("*", &[&new]))
		} else {
			Ok(1.into())
		}
	}

	/// TODO: unique, this is a stopgap
	#[instrument(name="Iterable::unique", level="trace", skip(this), fields(self=?this, ?args))]
	pub fn qs_unique(this: &Object, args: Args) -> crate::Result<Object> {
		if args.len() != 0 { panic!("todo: nonzero args."); }

		let this = this.call_downcast::<Iter>()?.clone();

		let mut unique = Vec::<Object>::new();

		'a: for ele in this {
			let ele = ele?;
			for past in &unique {
				if past.eq_obj(&ele)? {
					continue 'a;
				}
			}
			unique.push(ele);
		}

		Ok(Object::from(unique).call_downcast::<Iter>()?.clone().into())
	}

	#[instrument(name="Iterable::sort", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_sort(this: &Object, args: Args) -> crate::Result<Object> {
		todo!("sort");
	}

	#[instrument(name="Iterable::min", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_min(this: &Object, args: Args) -> crate::Result<Object> {
		let this = this.call_downcast::<Iter>()?.clone();

		let min =
			if let Some(start) = args.arg(0).cloned().or_else(|| this.next());

		panic!();
	}

	#[instrument(name="Iterable::max", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_max(this: &Object, args: Args) -> crate::Result<Object> {
		// let iter = this.call_downcast::<Iter>()?;

		// start =
		// 	if let Some(start) = iter.next().transpose()? {
		// 		start
		// 	} else {
		// 		return Object::default();
		// 	};

		// let block =
		// 	if let Some(cmp) = args.arg(0) {
		// 		move |acc, next| {
		// 			if cmp.call_attr_lit(&Literal::CALL, &[&acc,acc])?.call_downcast::<Boolean>()?.into_inner() {
		// 				Some(acc)
		// 			} else {
		// 				Some(next)
		// 			}
		// 		}
		// 	} else {
		// 		move |acc, next| {
		// 			if mac.call_attr_lit("<", &[&acc])?.call_downcast::<Boolean>()?.into_inner() {
		// 				Some(acc)
		// 			} else {
		// 				Some(next)
		// 			}					
		// 		}
		// 	};

		// iter.fold(start, move |acc, next| block.call_attr_lit(&Literal::CALL, &[&acc, &next]))

		// this.reduce()
		todo!();
	}

	/// Gets the amount of elements in this iterator.
	#[instrument(name="Iterable::len", level="trace", skip(this), fields(self=?this))]
	pub fn qs_len(this: &Object, _: Args) -> crate::Result<Object> {
		let this = this.call_downcast::<Iter>()?.clone();
		let mut i = 0;

		for ele in this {
			ele?;
			i += 1;
		}

		Ok(i.into())
	}

	/// The same as [`qs_len`], except it accepts an optional block which will be used to filter beforehand.
	///
	/// `.count { ... }` is the same as `.select { ... }.len()`.
	#[instrument(name="Iterable::count", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_count(this: &Object, args: Args) -> crate::Result<Object> {
		// if we're given a block, filter those values, and then return the count.
		if args.is_empty() {
			Self::qs_len(this, Args::default())
		} else {
			Self::qs_len(&Self::qs_select(this, args)?, Args::default())
		}
	}

	/// Gets the first element, or the first `n` elements if a value's given.
	/// This is the same as `take`, except it has a default value of one.
	#[instrument(name="Iterable::first", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_first(this: &Object, args: Args) -> crate::Result<Object> {
		if !args.is_empty() {
			return Self::qs_take(this, args);
		}

		let this = this.call_downcast::<Iter>()?.clone();
		Ok(this.take(1).into())
	}

	#[instrument(name="Iterable::last", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_last(this: &Object, args: Args) -> crate::Result<Object> {
		todo!()
		// if !args.is_empty() {
		// 	return Self::qs_take(this, args);
		// }

		// let this = this.call_downcast::<Iter>()?.clone();
		// Ok(this.last().into())
	}

	#[instrument(name="Iterable::all?", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_all_q(this: &Object, args: Args) -> crate::Result<Object> {
		let mut this = this.call_downcast::<Iter>()?.clone();

		if let Some(block) = args.arg(0) {
			let block = block.clone();
			this.all(move |ele|
					block.call_attr_lit(&Literal::CALL, &[&ele])?
						.call_downcast::<Boolean>()
						.map(|x| x.into_inner()))
		} else {
			this.all(move |ele| ele.call_downcast::<Boolean>().map(|x| x.into_inner()))
		}.map(Object::from)
	}

	#[instrument(name="Iterable::any?", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_any_q(this: &Object, args: Args) -> crate::Result<Object> {
		let mut this = this.call_downcast::<Iter>()?.clone();

		if let Some(block) = args.arg(0) {
			let block = block.clone();
			this.any(move |ele|
					block.call_attr_lit(&Literal::CALL, &[&ele])?
						.call_downcast::<Boolean>()
						.map(|x| x.into_inner()))
		} else {
			this.all(move |ele| ele.call_downcast::<Boolean>().map(|x| x.into_inner()))
		}.map(Object::from)
	}

	#[instrument(name="Iterable::one?", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_one_q(this: &Object, args: Args) -> crate::Result<Object> {
		let mut this = this.call_downcast::<Iter>()?.clone();

		if let Some(block) = args.arg(0) {
			let block = block.clone();
			this.one(move |ele|
					block.call_attr_lit(&Literal::CALL, &[&ele])?
						.call_downcast::<Boolean>()
						.map(|x| x.into_inner()))
		} else {
			this.all(move |ele| ele.call_downcast::<Boolean>().map(|x| x.into_inner()))
		}.map(Object::from)
	}

	#[instrument(name="Iterable::none?", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_none_q(this: &Object, args: Args) -> crate::Result<Object> {
		let mut this = this.call_downcast::<Iter>()?.clone();

		if let Some(block) = args.arg(0) {
			let block = block.clone();
			this.one(move |ele|
					block.call_attr_lit(&Literal::CALL, &[&ele])?
						.call_downcast::<Boolean>()
						.map(|x| x.into_inner()))
		} else {
			this.all(move |ele| ele.call_downcast::<Boolean>().map(|x| x.into_inner()))
		}.map(Object::from)
	}

	#[instrument(name="Iterable::include?", level="trace", skip(this, args), fields(self=?this, ?args))]
	pub fn qs_include_q(this: &Object, args: Args) -> crate::Result<Object> {
		let mut this = this.call_downcast::<Iter>()?.clone();
		let to_find = args.try_arg(0)?.clone();

		this.any(move |obj|
			to_find.call_attr_lit(&Literal::EQL, &[&obj])?
				.call_downcast::<Boolean>()
				.map(|x| x.into_inner())
			).map(Object::from)
	}
}

impl_object_type! { for Iterable [(parents super::Class)]:
	"@list"       => method Self::qs_at_list,
	"run"         => method Self::qs_run,
	"enumerate"   => method Self::qs_enumerate,
	"map"         => method Self::qs_map,
	"each"        => method Self::qs_each,
	"eachl"       => method Self::qs_eachl,
	"select"      => method Self::qs_select,
	"reject"      => method Self::qs_reject,
	"reduce"      => method Self::qs_reduce,
	"zip"         => method Self::qs_zip,
	"cycle"       => method Self::qs_cycle,
	"take"        => method Self::qs_take,
	"take_while"  => method Self::qs_take_while,
	"take_until"  => method Self::qs_take_until,
	"drop"        => method Self::qs_drop,
	"drop_while"  => method Self::qs_drop_while,
	"drop_until"  => method Self::qs_drop_until,
	"chunk"       => method Self::qs_chunk,
	"chunk_while" => method Self::qs_chunk_while,
	"chunk_until" => method Self::qs_chunk_until,
	"windows"     => method Self::qs_windows,
	"sum"         => method Self::qs_sum,
	"prod"        => method Self::qs_prod,
	"unique"      => method Self::qs_unique,
	"chain"       => method Self::qs_chain,
	"find"        => method Self::qs_find_first,
	"sort"        => method Self::qs_sort,
	"flatten"     => method Self::qs_flatten,
	"group_by"    => method Self::qs_group_by,

	"min"       => method Self::qs_min,
	"max"       => method Self::qs_max,
	"len"       => method Self::qs_len,
	"count"     => method Self::qs_count,
	"first"     => method Self::qs_first,
	"last"      => method Self::qs_last,
	"all?"      => method Self::qs_all_q,
	"any?"      => method Self::qs_any_q,
	"one?"      => method Self::qs_one_q,
	"none?"     => method Self::qs_none_q,
	"include?"  => method Self::qs_include_q,
}

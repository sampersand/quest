use crate::{Args, Object, Error, Literal};
use crate::types::{Boolean, Text, Null, Number};
use tracing::instrument;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Kernel;

fn display(args: &[&Object], newline: bool) -> crate::Result<()> {
	print!("{}",
		args.iter()
			.map(|x| object_to_string(*x))
			.collect::<crate::Result<Vec<_>>>()?
			.join(" ")
	);

	if newline {
		println!();
		Ok(())
	} else {
		use std::io::{self, Write};

		io::stdout()
			.flush()
			.map_err(|err| Error::Messaged(format!("couldn't flush: {}", err)))
	}
}

#[inline]
fn is_object_truthy(object: &Object) -> crate::Result<bool> {
	object.call_downcast::<Boolean>().map(|b| b.into_inner())
}

#[inline]
fn object_to_string(object: &Object) -> crate::Result<String> {
	object.call_downcast::<Text>().map(|t| t.to_string())
}

impl Kernel {
	/// Checks the first attribute
	#[instrument(name="Kernel::if", level="trace")]
	pub fn qs_if(cond: &Object, args: Args) -> crate::Result<Object> {
		let if_true = args.try_arg(0)?;
		let is_truthy = is_object_truthy(cond)?;

		tracing::debug!(?cond, %is_truthy);

		if is_truthy {
			if_true.call_attr_lit(&Literal::CALL, &[])
		} else if let Some(if_false) = args.arg(1) {
			if_false.call_attr_lit(&Literal::CALL, &[])
		} else {
			Ok(cond.clone())
		}
	}

	#[instrument(name="Kernel::unless", level="trace")]
	pub fn qs_unless(cond: &Object, args: Args) -> crate::Result<Object> {
		let if_true = args.try_arg(0)?;
		let is_truthy = is_object_truthy(cond)?;

		tracing::debug!(?cond, %is_truthy);

		if !is_truthy {
			if_true.call_attr_lit(&Literal::CALL, &[])
		} else if let Some(if_false) = args.arg(1) {
			if_false.call_attr_lit(&Literal::CALL, &[])
		} else {
			Ok(cond.clone())
		}
	}


	#[instrument(name="Kernel::disp", level="trace")]
	pub fn qs_disp(args: Args) -> crate::Result<Object> {
		display(args.as_ref(), true)?;

		Ok(Object::default())
	}

	#[instrument(name="Kernel::dispn", level="trace")]
	pub fn qs_dispn(args: Args) -> crate::Result<Object> {
		display(args.as_ref(), false)?;

		Ok(Object::default())
	}

	#[instrument(name="Kernel::while", level="trace")]
	pub fn qs_while(cond: &Object, args: Args) -> crate::Result<Object> {
		let body = args.try_arg(0)?;
		let mut result = None;

		while is_object_truthy(&cond.call_attr_lit(&Literal::CALL, &[])?)? {
			result = Some(body.call_attr_lit(&Literal::CALL, &[])?);
		}

		Ok(result.unwrap_or_default())
	}

	#[instrument(name="Kernel::loop", level="trace")]
	pub fn qs_loop(body: &Object, _: Args) -> crate::Result<Object> {
		loop {
			body.call_attr_lit(&Literal::CALL, &[])?;
		}
	}

	#[instrument(name="Kernel::quit", level="trace")]
	pub fn qs_quit(args: Args) -> crate::Result<Object> {
		use std::convert::TryFrom;

		let code = 
			if let Some(code) = args.arg(0) {
				i32::try_from(*code.call_downcast::<Number>()?)?
			} else {
				1
			};

		if let Some(msg) = args.arg(1) {
			display(&[msg], true)?;
		}

		std::process::exit(code as i32)
	}

	#[instrument(name="Kernel::system", level="trace")]
	pub fn qs_system(cmd: &Object, args: Args) -> crate::Result<Object> {
		use std::process::Command;
		let cmd = object_to_string(cmd)?;
		let mut command = Command::new(cmd);

		for arg in args.as_ref() {
			command.arg(object_to_string(arg)?);
		}

		command.output()
			.map_err(|err| Error::Messaged(format!("couldnt spawn proc: {}", err)))
			.map(|output| String::from_utf8_lossy(&output.stdout).to_string().into())
	}

	#[instrument(name="Kernel::rand", level="trace")]
	pub fn qs_rand(args: Args) -> crate::Result<Object> {
		use crate::types::number::FloatType;

		let mut start: FloatType = 0.0;
		let mut end: FloatType = 1.0;

		if let Some(start_num) = args.arg(0) {
			start = (*start_num.call_downcast::<Number>()?).into();

			if let Some(end_num) = args.arg(1) {
				end = (*end_num.call_downcast::<Number>()?).into();
			} else {
				end = start;
				start = 0.0;
			}
		}

		Ok((rand::random::<FloatType>() * (end - start) + start).into())
	}

	#[instrument(name="Kernel::prompt", level="trace")]
	pub fn qs_prompt(args: Args) -> crate::Result<Object> {
		use std::io;

		if let Some(arg) = args.arg(0) {
			display(&[arg], false)?;
		}

		let mut buf = String::new();

		io::stdin().read_line(&mut buf)
			.map_err(|err| Error::Messaged(format!("couldn't read from stdin: {}", err)))?;

		if buf.is_empty() {
			return Ok(Object::default())
		}

		if buf.ends_with('\n') {
			if cfg!(debug_asserts) {
				assert_eq!(buf.pop(), Some('\n'));
			} else {
				buf.pop();
			}
		}

		Ok(buf.into())
	}

	#[instrument(name="Kernel::return", level="trace")]
	pub fn qs_return(to: &Object, args: Args) -> crate::Result<Object> {
		let to = crate::Binding::from(to.clone());
		let obj = args.arg(0).map(Object::clone).unwrap_or_default();

		Err(Error::Return { to, obj })
	}

	#[instrument(name="Kernel::assert", level="trace")]
	pub fn qs_assert(cond: &Object, args: Args) -> crate::Result<Object> {
		if is_object_truthy(cond)? {
			return Ok(cond.clone());
		}

		let msg = args.arg(0).map(object_to_string).transpose()?;

		Err(Error::AssertionFailed(msg))
	}

	#[instrument(name="Kernel::sleep", level="trace")]
	pub fn qs_sleep(args: Args) -> crate::Result<Object> {
		if let Some(arg) = args.arg(0) {
			let dur: f64 = (*arg.call_downcast::<Number>()?).into();
			std::thread::sleep(std::time::Duration::from_secs_f64(dur));
		} else {
			// technically doesn't sleep forever lol.
			std::thread::sleep(std::time::Duration::from_secs_f64(f64::MAX));
		}

		Ok(Object::default())
	}

	#[instrument(name="Kernel::open", level="trace")]
	pub fn qs_open(_: &Object, _args: Args) -> crate::Result<Object> {
		// let filename = args.try_arg(0)?.downcast_call::<types::Text>();
		todo!("open")
	}
}

impl_object_type!{
for Kernel [(parents super::Pristine)]: // todo: do i want its parent to be pristine?
	"true" => const Boolean::new(true),
	"false" => const Boolean::new(false),
	"null" => const Null::new(),

	"Io" => const super::Io::mapping().clone(),
	"Tcp" => const super::Tcp::mapping().clone(),
	"Basic" => const super::Basic::mapping().clone(),
	"Boolean" => const super::Boolean::mapping().clone(),
	"BoundFunction" => const super::BoundFunction::mapping().clone(),
	"Function" => const super::Function::mapping().clone(),
	"Kernel" => const Kernel::mapping().clone(),
	"List" => const super::List::mapping().clone(),
	"Null" => const super::Null::mapping().clone(),
	"Number" => const super::Number::mapping().clone(),
	"Pristine" => const super::Pristine::mapping().clone(),
	"RustFn" => const super::RustFn::mapping().clone(),
	"Scope" => const super::Scope::mapping().clone(),
	"Text" => const super::Text::mapping().clone(),
	"Comparable" => const super::Comparable::mapping().clone(),
	"Iterable" => const super::Iterable::mapping().clone(),

	"if" => method Self::qs_if, 
	"unless" => method Self::qs_unless, 
	"disp" => function Self::qs_disp,
	"dispn" => function Self::qs_dispn,
	"quit" => function Self::qs_quit,
	"system" => method Self::qs_system,
	"rand" => function Self::qs_rand,
	"prompt" => function Self::qs_prompt,
	"while" => method Self::qs_while,
	"loop" => method Self::qs_loop,
	"sleep" => function Self::qs_sleep,
	"open" => method Self::qs_open,
	"return" => method Self::qs_return,
	"assert" => method Self::qs_assert,
	"spawn" => method |block, _| {
		use std::thread::{self, JoinHandle};
		use std::sync::Arc;
		use parking_lot::Mutex;
		#[derive(Debug, Clone)]
		struct Thread(Arc<Mutex<Option<JoinHandle<crate::Result<Object>>>>>);

		impl_object_type! { for Thread [(parents super::Basic)]:
			"join" => method |this, _| this.try_downcast::<Thread>().and_then(|this| {
				this.0.lock().take().expect("no join handle?")
					.join()
					.unwrap()
			})
		}

		Thread::initialize().unwrap();

		let block = block.clone();
		Ok(Thread(Arc::new(Mutex::new(Some(thread::spawn(move ||
			block.call_attr_lit(&Literal::CALL, &[&block])
		))))).into())
	},
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn constants_exist() {
		use crate::types::*;

		macro_rules! assert_exists_eq {
			($($key:literal $ty:ty, $val:expr),*) => {
				$(
					assert_eq!(
						$val,
						*Kernel::mapping()
							.get_attr_lit($key)
							.unwrap()
							.downcast::<$ty>().unwrap(),
						"constant {:?} doesn't exist or is wrong value",
						$key
					);
				)*
			}
		}

		crate::init();

		assert_exists_eq!(
			"true" Boolean, Boolean::new(true),
			"false" Boolean, Boolean::new(false),
			"null" Null, Null::new()
		);
	}

	#[test]
	fn classes_exist() {
		use crate::types::*;
		crate::init();

		macro_rules! assert_mapping_eq {
			($($key:literal $class:ty),*) => {
				$({
					let expected = <$class as ObjectType>::mapping();
					let got = Object::from(Kernel)
						.get_attr_lit($key)
						.unwrap();
					assert!(
						expected.is_identical(&got),
						"class {:?} doesn't exist or is wrong (expected={:?}, got={:?})",
						$key, expected, got
					);
				})*
			}
		}

		assert_mapping_eq!(
			"Basic" Basic, /*"Block" Block,*/ "Boolean" Boolean, "Function" Function,
			"Kernel" Kernel, "List" List, "Null" Null, "Number" Number,
			"Pristine" Pristine, "RustFn" RustFn, "Text" Text
		);
	}

	#[test]
	#[ignore]
	fn r#if() { todo!() }

	#[test]
	#[ignore]
	fn disp() { todo!() }

	#[test]
	#[ignore]
	fn quit() { todo!() }

	#[test]
	#[ignore]
	fn system() { todo!() }

	#[test]
	#[ignore]
	fn rand() { todo!() }

	#[test]
	#[ignore]
	fn prompt() { todo!() }

	#[test]
	#[ignore]
	fn r#while() { todo!() }

	#[test]
	#[ignore]
	fn r#for() { todo!() }

	#[test]
	#[ignore]
	fn sleep() { todo!() }

	#[test]
	#[ignore]
	fn open() { todo!() }
}

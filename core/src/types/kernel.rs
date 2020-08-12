use crate::{Args, Object, Error};
use crate::types::{Boolean, Text, Null, Number};
use crate::literal::CALL;

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
	pub fn qs_if(_: &Object, args: Args) -> crate::Result<Object> {
		let cond = args.try_arg(0)?;
		let if_true = args.try_arg(1)?;

		if is_object_truthy(cond)? {
			if_true.call_attr_lit(CALL, &[])
		} else if let Some(if_false) = args.arg(2) {
			if_false.call_attr_lit(CALL, &[])
		} else {
			Ok(Object::default())
		}
	}

	pub fn qs_disp(_: &Object, args: Args) -> crate::Result<Object> {
		display(args.as_ref(), true)?;

		Ok(Object::default())
	}

	pub fn qs_dispn(_: &Object, args: Args) -> crate::Result<Object> {
		display(args.as_ref(), false)?;

		Ok(Object::default())
	}

	pub fn qs_while(_: &Object, args: Args) -> crate::Result<Object> {
		let cond = args.try_arg(0)?;
		let body = args.try_arg(1)?;
		let mut result = Object::default();

		while is_object_truthy(&cond.call_attr_lit(CALL, &[])?)? {
			result = body.call_attr_lit(CALL, &[])?;
		}

		Ok(result)
	}

	pub fn qs_loop(_: &Object, args: Args) -> crate::Result<Object> {
		let body = args.try_arg(0)?;

		loop {
			body.call_attr_lit(CALL, &[])?;
		}
	}

	pub fn qs_for(_: &Object, _args: Args) -> crate::Result<Object> {
		todo!("r#for")
	}

	pub fn qs_quit(_: &Object, args: Args) -> crate::Result<Object> {
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

	pub fn qs_system(_: &Object, args: Args) -> crate::Result<Object> {
		use std::process::Command;
		let cmd = object_to_string(args.try_arg(0)?)?;
		let mut command = Command::new(cmd);

		for arg in args.try_args(1..).unwrap_or_default().as_ref() {
			command.arg(object_to_string(arg)?);
		}

		command.output()
			.map_err(|err| Error::Messaged(format!("couldnt spawn proc: {}", err)))
			.map(|output| String::from_utf8_lossy(&output.stdout).to_string().into())
	}

	pub fn qs_rand(_: &Object, args: Args) -> crate::Result<Object> {
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

	pub fn qs_prompt(_: &Object, args: Args) -> crate::Result<Object> {
		use std::io;

		if let Some(arg) = args.arg(0) {
			display(&[arg], false)?;
		}

		let mut buf = String::new();

		io::stdin().read_line(&mut buf)
			.map_err(|err| Error::Messaged(format!("couldn't read from stdin: {}", err)))?;

		if buf.ends_with('\n') {
			if cfg!(debug_asserts) {
				assert_eq!(buf.pop(), Some('\n'));
			} else {
				buf.pop();
			}
		}

		Ok(buf.into())
	}

	pub fn qs_return(_: &Object, args: Args) -> crate::Result<Object> {
		let to = crate::Binding::from(args.try_arg(0)?.clone());
		let obj = args.arg(1).map(Object::clone).unwrap_or_default();

		Err(Error::Return { to, obj })
	}

	pub fn qs_assert(_: &Object, args: Args) -> crate::Result<Object> {
		let arg = args.try_arg(0)?;

		if is_object_truthy(arg)? {
			return Ok(arg.clone());
		}

		let msg = args.arg(1).map(object_to_string).transpose()?;

		Err(Error::AssertionFailed(msg))
	}

	pub fn qs_sleep(_: &Object, args: Args) -> crate::Result<Object> {
		let dur = *args.try_arg(0)?.call_downcast::<Number>()?;

		std::thread::sleep(std::time::Duration::from_secs_f64(dur.into()));
		Ok(Object::default())
	}

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

	"if" => function Self::qs_if, 
	"disp" => function Self::qs_disp,
	"dispn" => function Self::qs_dispn,
	"quit" => function Self::qs_quit,
	"system" => function Self::qs_system,
	"rand" => function Self::qs_rand,
	"prompt" => function Self::qs_prompt,
	"while" => function Self::qs_while,
	"loop" => function Self::qs_loop,
	"for" => function Self::qs_for,
	"sleep" => function Self::qs_sleep,
	"open" => function Self::qs_open,
	"return" => function Self::qs_return,
	"assert" => function Self::qs_assert,
	"spawn" => function |_, args| {
		use std::thread::{self, JoinHandle};
		use std::sync::Arc;
		use parking_lot::Mutex;
		#[derive(Debug, Clone)]
		struct Thread(Arc<Mutex<Option<JoinHandle<crate::Result<Object>>>>>);

		impl_object_type! { for Thread [(parents super::Basic)]:
			"join" => function |this, _| this.try_downcast::<Thread>().and_then(|this| {
				this.0.lock().take().expect("no join handle?")
					.join()
					.unwrap()
			})
		}

		Thread::initialize().unwrap();

		let block = args.try_arg(0)?.clone();
		Ok(Thread(Arc::new(Mutex::new(Some(thread::spawn(move ||
			block.call_attr_lit(crate::literal::CALL, &[&block])
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

		crate::initialize();

		assert_exists_eq!(
			"true" Boolean, Boolean::new(true),
			"false" Boolean, Boolean::new(false),
			"null" Null, Null::new()
		);
	}

	#[test]
	fn classes_exist() {
		use crate::types::*;
		crate::initialize();

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
	fn eval() { todo!() }

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

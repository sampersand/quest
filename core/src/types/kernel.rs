use crate::{Args, Object, Error, Result};
use crate::types::{Boolean, Text, Number};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Kernel;

fn display(args: &[&Object], newline: bool) -> Result<()> {
	print!("{}",
		args.iter()
			.map(|x| object_to_string(*x))
			.collect::<Result<Vec<_>>>()?
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
fn is_object_truthy(object: &Object) -> Result<bool> {
	object.call_downcast_map(Boolean::clone).map(bool::from)
}

#[inline]
fn object_to_string(object: &Object) -> Result<String> {
	object.call_downcast_map(Text::to_string)
}

impl Kernel {
	pub fn qs_if(_: &Object, args: Args) -> Result<Object> {
		if is_object_truthy(args.arg(0)?)? {
			args.arg(1)?.clone()
		} else {
			args.arg(2).map(Clone::clone).unwrap_or_default()
		}.call_attr_lit("()", &[])
	}

	pub fn qs_disp(_: &Object, args: Args) -> Result<Object> {
		display(args.as_ref(), true).map(|_| Object::default())
	}

	pub fn qs_dispn(_: &Object, args: Args) -> Result<Object> {
		display(args.as_ref(), false).map(|_| Object::default())
	}

	pub fn qs_while(_: &Object, args: Args) -> Result<Object> {
		let cond = args.arg(0)?;
		let body = args.arg(1)?;
		// crate::Binding::new_stackframe_old(args.args(2..).unwrap_or_default(), move |b| {
		// 	b.set_attr_old("name", Object::from("while"))?;

			let mut result = Object::default();
			while is_object_truthy(&cond.call_attr_lit("()", &[])?)? {
				result = body.call_attr_lit("()", &[])?;
			};
			Ok(result)
		// })
	}

	pub fn qs_loop(_: &Object, args: Args) -> Result<!> {
		let body = args.arg(0)?;
		// crate::Binding::new_stackframe_old(args.args(1..).unwrap_or_default(), move |b| {
			// b.set_attr_old("name", Object::from("loop"))?;
			loop {
				body.call_attr_lit("()", &[])?;
			}
		// })
	}

	pub fn qs_for(_: &Object, _args: Args) -> Result<Object> {
		todo!("r#for")
	}

	pub fn qs_quit(_: &Object, args: Args) -> Result<Object> {
		use std::convert::TryFrom;

		let code = args.arg(0)
			.ok()
			.map(|x| x.call_downcast_map(Number::floor))
			.transpose()?
			.unwrap_or(Number::ONE);

		let code: i32 = i32::try_from(code)
			.map_err(|err|crate::error::TypeError::NotAnInteger(Number::from(err.0)))?;

		if let Ok(msg) = args.arg(1) {
			display(&[msg], true)?;
		}

		std::process::exit(code as i32)
	}

	pub fn qs_system(_: &Object, args: Args) -> Result<Object> {
		use std::process::Command;
		let cmd = object_to_string(args.arg(0)?)?;
		let mut command = Command::new(cmd);

		for arg in args.args(1..).unwrap_or_default().as_ref() {
			command.arg(object_to_string(arg)?);
		}

		command.output()
			.map_err(|err| Error::Messaged(format!("couldnt spawn proc: {}", err)))
			.map(|output| String::from_utf8_lossy(&output.stdout).to_string().into())
	}

	pub fn qs_rand(_: &Object, args: Args) -> Result<Object> {
		use crate::types::number::FloatType;

		let mut start: FloatType = 0.0;
		let mut end: FloatType = 1.0;

		if let Ok(start_num) = args.arg(0) {
			start = start_num.call_downcast_map(|n: &Number| FloatType::from(*n))?;

			if let Ok(end_num) = args.arg(1) {
				end = end_num.call_downcast_map(|n: &Number| FloatType::from(*n))?;
			} else {
				end = start;
				start = 0.0;
			}
		}

		Ok((rand::random::<FloatType>() * (end - start) + start).into())
	}

	pub fn qs_prompt(_: &Object, args: Args) -> Result<Object> {
		use std::io;

		if let Ok(arg) = args.arg(0) {
			display(&[arg], false)?;
		}

		let mut buf = String::new();

		match io::stdin().read_line(&mut buf) {
			Ok(_) => {
				if buf.ends_with('\n') {
					buf.pop(); // remove trailing newline; only on unix currently...
				}

				Ok(buf.into())
			},
			Err(err) => Err(Error::Messaged(format!("couldn't read from stdin: {}", err)))
		}
	}

	pub fn qs_return(_: &Object, args: Args) -> Result<Object> {
		let to = crate::Binding::from(args.arg(0)?.clone());
		let obj = args.arg(1).map(Clone::clone).unwrap_or_default();

		Err(Error::Return { to, obj })
	}

	pub fn qs_assert(_: &Object, args: Args) -> Result<Object> {
		let arg = args.arg(0)?;
		if is_object_truthy(arg)? {
			Ok(arg.clone())
		} else {
			Err(Error::AssertionFailed(
				if let Ok(msg) = args.arg(1) {
					Some(object_to_string(msg)?)
				} else {
					None
				})
			)
		}
	}

	pub fn qs_sleep(_: &Object, _args: Args) -> Result<Object> {
		todo!("sleep")
	}

	pub fn qs_open(_: &Object, _args: Args) -> Result<Object> {
		// let filename = args.arg(0)?.downcast_call::<types::Text>();
		todo!("open")
	}
}

impl_object_type!{
for Kernel [(parents super::Pristine)]: // todo: do i want its parent to be pristine?
	"true" => const Boolean::new(true),
	"false" => const Boolean::new(false),
	"null" => const Null::new(),

	"Basic" => const super::Basic::mapping(),
	"Boolean" => const super::Boolean::mapping(),
	"BoundFunction" => const super::BoundFunction::mapping(),
	"Function" => const super::Function::mapping(),
	"Kernel" => const Kernel::mapping(),
	"List" => const super::List::mapping(),
	"Null" => const super::Null::mapping(),
	"Number" => const super::Number::mapping(),
	"Pristine" => const super::Pristine::mapping(),
	"RustFn" => const super::RustFn::mapping(),
	"Scope" => const super::Scope::mapping(),
	"Text" => const super::Text::mapping(),
	"Comparable" => const super::Comparable::mapping(),

	"if" => function Kernel::qs_if, 
	"disp" => function Kernel::qs_disp,
	"dispn" => function Kernel::qs_dispn,
	"quit" => function Kernel::qs_quit,
	"system" => function Kernel::qs_system,
	"rand" => function Kernel::qs_rand,
	"prompt" => function Kernel::qs_prompt,
	"while" => function Kernel::qs_while,
	"loop" => function Kernel::qs_loop,
	"for" => function Kernel::qs_for,
	"sleep" => function Kernel::qs_sleep,
	"open" => function Kernel::qs_open,
	"return" => function Kernel::qs_return,
	"assert" => function Kernel::qs_assert,

	// "&&" => impls::and,
	// "||" => impls::or,
}


#[cfg(test)]
mod tests {
	#[test]
	fn constants_exist() {
		use crate::types::*;

		macro_rules! assert_exists_eq {
			($($key:literal $ty:ty, $val:expr),*) => {
				$(
					assert_eq!(
						$val,
						Kernel::mapping()
							.get_attr_lit($key)
							.unwrap()
							.downcast_and_then(<$ty>::clone).unwrap(),
						"constant {:?} doesn't exist or is wrong value",
						$key
					);
				)*
			}
		}

		Kernel::_wait_for_setup_to_finish();

		assert_exists_eq!(
			"true" Boolean, Boolean::new(true),
			"false" Boolean, Boolean::new(false),
			"null" Null, Null::new()
		);
	}

	#[test]
	fn classes_exist() {
		use crate::types::*;
		Kernel::_wait_for_setup_to_finish();

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

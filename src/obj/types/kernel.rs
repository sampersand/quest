#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Kernel;

mod impls {
	use crate::obj::{Object, Result, Args, types};
	// pub const TRUE: Object = Object::new(types::boolean::TRUE);

	pub fn r#if(args: Args) -> Result<Object> {
		if args.arg(0)?.downcast_call::<types::Boolean>()?.into() {
			args.arg(1).map(Clone::clone)
		} else {
			Ok(args.arg(2).map(Clone::clone).unwrap_or_default())
		}
	}

	pub fn disp(args: Args, print_end: bool) -> Result<Object> {
		print!("{}",
			args.args(..)
				.unwrap_or_default()
				.as_ref()
				.iter()
				.map(|arg| arg.downcast_call::<types::Text>())
				.collect::<Result<Vec<_>>>()?
				.into_iter()
				.map(|arg| arg.as_ref().to_string())
				.collect::<Vec<_>>()
				.join(" ")
		);
		if print_end {
			println!();
		}
		use std::io::Write;
		std::io::stdout().flush().map_err(|err| Object::from(format!("couldn't flush: {}", err)))?;
		Ok(Object::default())
	}

	pub fn r#while(args: Args) -> Result<Object> {
		let cond = args.arg(0)?;
		let body = args.arg(1)?;
		let call_args = 
			match args.arg(2) {
				Ok(arg) => Vec::from(arg.downcast_call::<types::List>()?).into(),
				Err(_) => Args::default()
			};
		let mut result = Object::default();
		while cond.call_attr("()", call_args.clone())?.downcast_call::<types::Boolean>()?.into() {
			result = body.call_attr("()", call_args.clone())?;
		}
		Ok(result)
	}

	pub fn r#for(args: Args) -> Result<Object> {
		todo!("r#for")
	}

	pub fn quit(args: Args) -> Result<Object> {
		let code = args.arg(0)
			.and_then(|x| x.downcast_call::<types::Number>())
			.map(|x| x.to_int())
			.unwrap_or(1);

		if let Some(msg) = args.arg(1).ok() {
			disp(vec![msg.clone()].into(), true);
		}

		std::process::exit(code as i32)
	}

	pub fn system(args: Args) -> Result<Object> {
		use std::process::Command;
		let cmd = args.arg(0)?.downcast_call::<types::Text>()?;
		let mut command = Command::new(cmd.as_ref());
		for arg in args.args(1..).unwrap_or_default().as_ref() {
			command.arg(arg.downcast_call::<types::Text>()?.as_ref());
		}
		let output = command.output().map_err(|err| format!("couldnt spawn proc: {}", err))?;
		Ok(String::from_utf8_lossy(&output.stdout).to_string().into())
	}

	pub fn rand(args: Args) -> Result<Object> {
		let mut start: f64 = 0.0;
		let mut end: f64 = 1.0;

		if let Some(start_num) = args.arg(0).ok() {
			start = start_num.downcast_call::<types::Number>()?.into();

			if let Some(end_num) = args.arg(1).ok() {
				end = end_num.downcast_call::<types::Number>()?.into();
			} else {
				end = start;
				start = 0.0;
			}
		}
		Ok((rand::random::<f64>() * (end - start) + start).into())
	}

	pub fn eval(args: Args) -> Result<Object> {
		// use std::thread::Thread;
		todo!("eval")
	}

	pub fn prompt(args: Args) -> Result<Object> {
		if let Some(prompt) = args.arg(0).ok() {
			disp(vec![prompt.clone()].into(), false);
		}
		use std::io::{self, Read};
		let mut buf = String::new();

		match io::stdin().read_line(&mut buf) {
			Ok(_) => {
				if buf.chars().last() == Some('\n') {
					buf.pop(); // remove trailing newline; only on unix lol
				}

				Ok(buf.into())
			},
			Err(err) => Err(format!("couldn't read from stdin: {}", err).into())
		}
	}

	pub fn sleep(args: Args) -> Result<Object> {
		todo!("sleep")
	}

	pub fn open(args: Args) -> Result<Object> {
		todo!("open")
	}
}

impl_object_type!{
for Kernel [(parent super::Pristine)]: // todo: do i want its parent to be pristine?
	"true" => const Boolean::new(true),
	"false" => const Boolean::new(false),
	"null" => const Null::new(),

	"Basic" => const super::Basic::mapping(),
	"Block" => const super::Block::mapping(),
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

	"if" => impls::r#if, 
	"disp" => (|a| impls::disp(a, true)),
	"quit" => impls::quit,
	"system" => impls::system,
	"rand" => impls::rand,
	"eval" => impls::eval,
	"prompt" => impls::prompt,
	"while" => impls::r#while,
	"for" => impls::r#for,
	"sleep" => impls::sleep,
	"open" => impls::open,

	// "&&" => impls::and,
	// "||" => impls::or,
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn constants_exist() {
		use crate::obj::{Object, types::*};

		macro_rules! assert_exists_eq {
			($($key:literal $val:expr),*) => {
				$(
					assert_eq!(
						$val,
						*Kernel::mapping()
							.get_attr(&Object::from($key))
							.unwrap().downcast_ref().unwrap(),
						"constant {:?} doesn't exist or is wrong value",
						$key
					);
				)*
			}
		}

		Kernel::_wait_for_setup_to_finish();

		assert_exists_eq!(
			"true" Boolean::new(true),
			"false" Boolean::new(false),
			"null" Null::new()
		);
	}

	#[test]
	fn classes_exist() {
		use crate::obj::{Object, types::*};
		Kernel::_wait_for_setup_to_finish();

		macro_rules! assert_mapping_eq {
			($($key:literal $class:ty),*) => {
				$({
					let expected = <$class as ObjectType>::mapping();
					let got = Object::from(Kernel)
						.get_attr(&Object::from($key))
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
			"Basic" Basic, "Block" Block, "Boolean" Boolean, "Function" Function,
			"Kernel" Kernel, "List" List, "Map" Map, "Null" Null, "Number" Number,
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
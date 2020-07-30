use crate::{Args, Object, Error};
use crate::types::{Boolean, Text, Null, Number};
use crate::literal::CALL;

/// The kernel is a collection of core methods that are used constantly within Quest code.
///
/// Unlike most other languages, there is no concept of "keywords" within Quest: Instead, we have
/// methods like [`if`](#qs_if), [`while`](#qs_while) and [`return`](#qs_return) for control flow.
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
	object.call_downcast_map(Boolean::clone).map(bool::from)
}

#[inline]
fn object_to_string(object: &Object) -> crate::Result<String> {
	object.call_downcast_map(Text::to_string)
}

impl Kernel {
	/// Executes code based on the condition.
	///
	/// If the first argument is true (via [`@bool`]), then the second argument will be be called.
	/// If it's not, then the third argument will be [called] (or return [`null`] if no third
	/// argument is given)
	///
	/// # Terninary Operator
	/// Because this is a function, it can also be used in place of the traditional terninary
	/// operator from other languages:
	/// ```quest
	/// $name = "Sam";
	/// disp("Hi, there",
	/// 	if(name == "Sam", {
	/// 		name + ", the friend"
	///	}, {
	/// 		"mysterious perosn: " + name
	/// 	}))
	/// ```
	///
	/// # Arguments
	/// 1. (required, [`@bool`]) The condition.
	/// 2. (required, [`()`]) The value to be called when true.
	/// 3. (optional, [`()`]) The value to be called when false, defaults to [`null`] 
	///
	/// [`null`]: Null
	/// [`()`]: CALL
	/// [`@bool`]: crate::literal::AT_BOOL
	pub fn qs_if(_: &Object, args: Args) -> crate::Result<Object> {
		if is_object_truthy(args.arg(0)?)? {
			args.arg(1)?.clone()
		} else {
			args.arg(2).map(Clone::clone).unwrap_or_default()
		}.call_attr_lit(CALL, &[])
	}

	/// Displays values with a trailing newline
	///
	/// Each argument is converted to text and separated by a space, with a trailing newline added.
	/// For a version without the newline, see [`qs_dispn`](#qs_dispn).
	///
	/// # Arguments
	/// 1+ (optional, [`@text`](crate::literal::AT_TEXT)) The values to print.
	pub fn qs_disp(_: &Object, args: Args) -> crate::Result<Object> {
		display(args.as_ref(), true).map(|_| Object::default())
	}

	/// Displays values without a trailing newline
	///
	/// Each argument is converted to text and separated by a space, without a trailing newline
	/// added. For a version with the newline, see [`qs_disp`](#qs_disp).
	///
	/// # Arguments
	/// 1+ (optional, [`@text`](crate::literal::AT_TEXT)) The values to print.
	pub fn qs_dispn(_: &Object, args: Args) -> crate::Result<Object> {
		display(args.as_ref(), false).map(|_| Object::default())
	}

	/// Continues to code whilst a codition is true.
	///
	/// As long as the result of calling the first argument is true, the second argument will be
	/// executed. The last value returned from the body of the while loop is the return value of the
	/// while function.
	///
	/// # Arguments
	/// 1. (required, [`()`]) The condition; its return value must respond to [`@bool`]
	/// 2. (required, [`()`]) The body to execute.
	///
	/// [`()`]: CALL
	/// [`@bool`]: crate::literal::AT_BOOL
	pub fn qs_while(_: &Object, args: Args) -> crate::Result<Object> {
		let cond = args.arg(0)?;
		let body = args.arg(1)?;
		// crate::Binding::new_stackframe_old(args.args(2..).unwrap_or_default(), move |b| {
		// 	b.set_attr_old("name", Object::from("while"))?;

			let mut result = Object::default();
			while is_object_truthy(&cond.call_attr_lit(CALL, &[])?)? {
				result = body.call_attr_lit(CALL, &[])?;
			};
			Ok(result)
		// })
	}

	/// Execute code forever (or until it's [`return`](#qs_return)ed from).
	///
	/// This is functionally identical to calling `while({ true }, ...)`, but is a bit more
	/// efficient.
	///
	/// # Arguments
	/// 1. (required, [`()`](CALL)) The body to execute.
	pub fn qs_loop(_: &Object, args: Args) -> crate::Result<Object> {
		let body = args.arg(0)?;
		// crate::Binding::new_stackframe_old(args.args(1..).unwrap_or_default(), move |b| {
			// b.set_attr_old("name", Object::from("loop"))?;
			loop {
				body.call_attr_lit(CALL, &[])?;
			}
		// })
	}

	#[doc(hidden)]
	pub fn qs_for(_: &Object, _args: Args) -> crate::Result<Object> {
		todo!("r#for")
	}

	/// Stops the execution of the program
	///
	/// # Arguments
	/// 1. (optional, [`@num`]) The status code; defaults to `1`.
	/// 2. (optional, [`@text`]) The message to print before quitting.
	///
	/// [`@num`]: crate::literal::AT_NUM
	/// [`@text`]: crate::literal::AT_TEXT
	pub fn qs_quit(_: &Object, args: Args) -> crate::Result<Object> {
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

	/// Executes a command in a subshell, returning its stdout.
	///
	/// Note that this may be updated in the future to allow for more options, such as specifying 
	/// what happens to stderr, etc.
	///
	/// # Arguments
	/// 1. (required, [`@text`]) The command to execute.
	/// 2+ (optional, [`@text`]) The arguments to pass to the command
	///
	/// [`@text`]: crate::literal::AT_TEXT
	pub fn qs_system(_: &Object, args: Args) -> crate::Result<Object> {
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

	/// Gets a random number based on the given bounds.
	///
	/// Depending on the arguments given, the bounds are:
	/// - [`0`, `1`)
	/// - [`0`, `arg0`)
	/// - [`arg0`, `arg1`)
	///
	/// # Arguments
	/// 1. (optional, [`@num`]) The starting position if two args given. Otherwise the start.
	/// 2. (optional, [`@num`]) The ending position.
	///
	/// [`@num`]: crate::literal::AT_NUM
	pub fn qs_rand(_: &Object, args: Args) -> crate::Result<Object> {
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

	/// Prompt the user for input
	///
	/// The optional message is printed without a newline, and the result of the prompt has its
	/// endings stripped.
	///
	/// In the future, more options may be given, but that's not a priority currently.
	///
	/// # Arguments
	/// 1. (optional, [`@text`](crate::literal::AT_TEXT)) The message to be printed.
	pub fn qs_prompt(_: &Object, args: Args) -> crate::Result<Object> {
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

	/// Returns a value from the specified scope.
	///
	/// Because of how Quest works, there is no concept of "returning from a function"---as such,
	/// You need to specify _which_ function you want to return from.
	///
	/// # Arguments
	/// 1. (required) The stackframe to return from (e.g. `:1`)
	/// 2. (optional) The return value; if omited, [`null`](Null)
	pub fn qs_return(_: &Object, args: Args) -> crate::Result<Object> {
		let to = crate::Binding::from(args.arg(0)?.clone());
		let obj = args.arg(1).map(Clone::clone).unwrap_or_default();

		Err(Error::Return { to, obj })
	}

	/// Assert that a statement is true, raising an [`Error::AssertionFailed`] if it's not.
	///
	/// # Arguments
	/// 1. (required, [`@bool`](crate::literal::AT_BOOL)) The value to check against.
	/// 2. (optional, [`@text`](crate::literal::AT_TEXT)) The optional message to return.
	pub fn qs_assert(_: &Object, args: Args) -> crate::Result<Object> {
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

	/// Sleeps for the specified duration, in seconds.
	///
	/// # Arguments
	/// 1. (required, [`@num`](Crate::types::AT_NUM)) The time to sleep for, in seconds
	pub fn qs_sleep(_: &Object, args: Args) -> crate::Result<Object> {
		let dur = args.arg(0)?.call_downcast_map(Number::clone)?;
		std::thread::sleep(std::time::Duration::from_secs_f64(dur.into()));
		Ok(Object::default())
	}

	#[doc(hidden)]
	pub fn qs_open(_: &Object, _args: Args) -> crate::Result<Object> {
		// let filename = args.arg(0)?.downcast_call::<types::Text>();
		todo!("open")
	}
}

impl_object_type!{
for Kernel [(parents super::Pristine)]: // todo: do i want its parent to be pristine?
	"true" => const Boolean::new(true),
	"false" => const Boolean::new(false),
	"null" => const Null::new(),

	"Tcp" => const super::Tcp::mapping(),
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
	"Iterable" => const super::Iterable::mapping(),

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
			"join" => function |this, _| this.try_downcast_and_then(|this: &Thread| {
				this.0.lock().take().expect("no join handle?")
					.join()
					.unwrap()
			})
		}

		Thread::initialize().unwrap();

		let block = args.arg(0)?.clone();
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

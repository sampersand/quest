#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Kernel;

mod impls {
	use crate::obj::{Object, Result, Args, types};
	// pub const TRUE: Object = Object::new(types::boolean::TRUE);

	pub fn r#if(args: Args) -> Result<Object> {
		if args.arg_call_into::<types::Boolean>(0)?.into() {
			args.arg(1).map(Clone::clone)
		} else {
			args.arg(2).map(Clone::clone)
		}
	}

	pub fn disp(args: Args) -> Result<Object> {
		println!("{}",
			args.as_ref()
				.iter()
				.map(|arg| arg.call_attr("@text", args.new_args_slice(&[]))?
					.try_downcast_ref::<types::Text>().map(|x| (*x).clone()))
				.collect::<Result<Vec<_>>>()?
				.into_iter()
				.map(|arg| arg.as_ref().to_string())
				.collect::<Vec<_>>()
				.join(", ")
		);
		use std::io::Write;
		std::io::stdout().flush()
			.map_err(|err| Object::from(format!("couldn't flush: {}", err)))?;
		Ok(Object::default())
	}

	pub fn r#while(args: Args) -> Result<Object> {
		todo!("r#while")
	}

	pub fn r#for(args: Args) -> Result<Object> {
		todo!("r#for")
	}

	pub fn quit(args: Args) -> Result<Object> {
		todo!("quit")
	}

	pub fn system(args: Args) -> Result<Object> {
		todo!("system")
	}

	pub fn rand(args: Args) -> Result<Object> {
		todo!("rand")
	}

	pub fn eval(args: Args) -> Result<Object> {
		todo!("eval")
	}

	pub fn prompt(args: Args) -> Result<Object> {
		todo!("prompt")
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
	"Function" => const super::Function::mapping(),
	"Kernel" => const Kernel::mapping(),
	"List" => const super::List::mapping(),
	"Map" => const super::Map::mapping(),
	"Null" => const super::Null::mapping(),
	"Number" => const super::Number::mapping(),
	"Pristine" => const super::Pristine::mapping(),
	"RustFn" => const super::RustFn::mapping(),
	"Text" => const super::Text::mapping(),

	"if" => impls::r#if, 
	"disp" => impls::disp,
	"quit" => impls::quit,
	"system" => impls::system,
	"rand" => impls::rand,
	"eval" => impls::eval,
	"prompt" => impls::prompt,
	"while" => impls::r#while,
	"for" => impls::r#for,
	"sleep" => impls::sleep,
	"open" => impls::open,
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
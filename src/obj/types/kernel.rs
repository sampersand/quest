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
				.map(|arg| arg.call("@text", args.new_args_slice(&[]))?
					.try_downcast_ref::<types::Text>().map(|x| (*x).clone()))
				.collect::<Result<Vec<_>>>()?
				.into_iter()
				.map(|arg| arg.as_ref().to_string())
				.collect::<Vec<_>>()
				.join(", ")
		);
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

impl_object_type_!{for Kernel, super::Pristine;
	"true" => (expr boolean::TRUE),
	"false" => (expr boolean::FALSE),
	"null" => (expr null::NULL),

	"Basic" => (expr super::Basic::mapping()),
	"Block" => (expr super::Block::mapping()),
	"Boolean" => (expr super::Boolean::mapping()),
	"Function" => (expr super::Function::mapping()),
	"Kernel" => (expr Kernel::mapping()),
	"List" => (expr super::List::mapping()),
	"Map" => (expr super::Map::mapping()),
	"Null" => (expr super::Null::mapping()),
	"Number" => (expr super::Number::mapping()),
	"Pristine" => (expr super::Pristine::mapping()),
	"RustFn" => (expr super::RustFn::mapping()),
	"Text" => (expr super::Text::mapping()),

	"if" => (impls::r#if), 
	"disp" => (impls::disp),
	"quit" => (impls::quit),
	"system" => (impls::system),
	"rand" => (impls::rand),
	"eval" => (impls::eval),
	"prompt" => (impls::prompt),
	"while" => (impls::r#while),
	"for" => (impls::r#for),
	"sleep" => (impls::sleep),
	"open" => (impls::open),

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
							.get_attr(&$key.into(), &Default::default())
							.unwrap().downcast_ref().unwrap(),
						"constant {:?} doesn't exist or is wrong value",
						$key
					);
				)*
			}
		}

		Kernel::_wait_for_setup_to_finish();

		assert_exists_eq!(
			"true" boolean::TRUE,
			"false" boolean::FALSE,
			"null" null::NULL
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
						.get_attr(&$key.into(), &Default::default())
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
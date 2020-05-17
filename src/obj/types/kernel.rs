#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Kernel;

mod impls {
	use crate::obj::{Object, Result, Args, types};
	// pub const TRUE: Object = Object::new(types::boolean::TRUE);

	pub fn r#if(args: Args) -> Result<Object> {
		let cond = args.get(0)?;
		let if_true = args.get(1)?;
		let if_false = args.get(2).unwrap_or_default();

		if *cond.call("@bool", args.new_args_slice(&[]))?.try_downcast_ref::<types::Boolean>()?.as_ref() {
			Ok(if_true)
		} else {
			Ok(if_false)
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

impl_object_type!{for Kernel, super::Pristine;
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
#![allow(unused)]
#[macro_use]
use qvm::{*, value::*};

fn main() {
	#[derive(Debug, Clone, Copy, PartialEq, QuestType)]
	#[quest(typename="MyCustomName", skip("DeepClone"))]
	struct Custom(u32);

	impl DeepClone for Custom {
		fn deep_clone(&self) -> Result<Self> {
			println!("im deep cloning!");
			Ok(*self)
		}
	}


	let value = Value::new(List::new(vec![
		Value::new(Text::new("abcdef")),
		Value::new(BigNum::new(128i64)),
		Value::new(Custom(34))]));

	println!("{:?}", value.shallow_clone());
	/*
	   [qvm/src/main.rs:21] Value::new(Custom(34)) = Extern {
	    data: Custom(
	        34,
	    ),
	    parents: [Class("MyCustomName")],
	    attrs: {},
	   }
	*/


	// impl ExternType for Custom {}

	// impl ShallowClone for Custom {
	// 	fn shallow_clone(&self) -> Result<Self> {
	// 		Ok(Self(self.0))
	// 	}
	// }

	// impl DeepClone for Custom {
	// 	fn deep_clone(&self) -> Result<Self> {
	// 		Ok(Self(self.0))
	// 	}
	// }

	// let val = Value::new(List::new(vec![Value::new(Custom(34))]));
	// dbg!(val);
	// return;
	// println!("{:?}", Value::new(Boolean::new(true)));
	// println!("{:?}", Value::new(Boolean::new(false)));
	// println!("{:?}", Value::new_smallint(123));
	// println!("{:?}", Value::new(value::Null));
	// println!("{:?}", Value::new_custom("foo"));
}

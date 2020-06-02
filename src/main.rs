#![allow(unused, deprecated)]
extern crate rand;

use crate::obj::types::ObjectType;

mod obj;
mod parse;

fn main() {
	let filename = std::env::args().skip(1).next().unwrap_or("code/test.qs".to_string());
	let mut stream = parse::Stream::from_file(filename.as_ref())
		.expect("couldn't open file")
		.collect::<parse::Result<Vec<_>>>()
		.unwrap()
		.into_iter();

	let expression = parse::Expression::try_from_iter(&mut stream).unwrap();
	let args = std::env::args()
		.skip(1)
		.map(|x| obj::Object::from(String::from(x)))
		.collect::<Vec<_>>().into();
	let result = obj::Binding::new_stackframe(args, |_| expression.execute());
	// if cfg!(debug) {
		println!("{:?}", result);
	// } else {
	// 	result.unwrap();
	// }
}

//	let mut stream = parse::Stream::from_str(r##"

// "Frac" = {
// 	"()" = {
// 		"numer" = (_1."@num"());
// 		"denom" = (_2."@num"());
// 		if((_2 == 0), {
// 			return(-2, "error!")
// 		})();
// 		__this__
// 	};

// 	"@text" = {
// 		__this__."numer" + "/" + __this__."denom"
// 	};
// }();

// "half" = Frac(1, 2);
// disp((half."@text")() + " = half");
// 	"##);

/*

		# a."+@"()
		# + += +@ - -= -@ * *= ** **= % %= / /= ! != = ==
		# < <= <=> << <<= > >= >> >>= ~ & &= && | |= || ^ ^= . .= .~ , ;
#// (4 + (5 * 3)) * 3;
#// (12."floor")(x);
#// "y" = ((1 ** 2) * 3) + 4;
#// 3 + { (x), (y); (z) };
#// "car" = { "x" = [_1, _2]."last"[]; (_1 * _2)."floor"(x) };
#// this.x = "123" + that.34; # this
#// foo = { _1 * (_2.'3' = _3) };
#// disp("hello there," + this.x);
*/
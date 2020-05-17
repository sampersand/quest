#![allow(unused, deprecated)]

use crate::obj::types::ObjectType;

mod obj;
mod parse;

fn main() {
	// let x = [0xff]
	let mut stream = parse::Stream::from_str(r##"
		$kernel = Kernel;
		kernel.$true

# $name = prompt("name: ");
# if(name == "lali", {
# 	disp("<3 lali")
# }, {
# 	disp("?? who are you, " + name)
# })();


#		$_1 = "sam"; $name = _1;
#		$_2 = 22; $age = _2;
#		$__this___ = __this__;
#
#
#		$name = "sallm";
#		if(name == "sam", 
#			{ disp("hi") },
#			{ disp("bye") }
#		)()
#
#
#		$Person = {
#			"()" = {
#				__parent__ = Person;
#				$name = _1;
#				$age = _2;
#				__this__
#			};
#
#			"@text" = {
#				(__this___.$name) + ", aged " + (__this___.$age)
#			};
#
#			__this__
#		}();
#
#		$sam = Person("sam", 22);
#		disp("Hello, " + sam);








#		$sam = {
#			$name = "sam";
#			$age = 22;
#			"@text" = {
#				"sam, age 22"
#			};
#			__this__
#		}();
#
#		disp("Hello, " + sam);#+ (sam.$name) + ", aged " + (sam.$age));


		#$name = "Sam";
		#disp("Hello, " + name + "!");

		#__this__.$f = 3;
		#disp(f)

		#if(name == "Sam", {
		#	disp("Your favorite color is green!");
		#})();

 	"##);

	let mut stream = stream.collect::<parse::Result<Vec<_>>>().unwrap().into_iter();

	let o = obj::Object::from("a");
	// println!("{:#?}", obj::types::Number::mapping());
	// return;
	// println!("{:#?}", o);
	// println!("{:?}", o.get_attr(&"__parent__".into()).unwrap()
	// 		.get_attr(&"name".into())
	// );
	// println!("{:?}", co.get_attr(&"true".into()));
	let expression = parse::Expression::try_from_iter(&mut stream).unwrap();
	let result = expression.execute_default().unwrap();
	println!("{:#?}", result);
	// println!("{:?}", result.call("@text", Default::default()));
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